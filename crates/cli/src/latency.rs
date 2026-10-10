use crate::auth::{RelayEntry, RelayInfoResponse};
use anyhow::{Result, bail};
use relay_common::constants::{NETWORK_TIMEOUT, RELAY_PORT};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tracing::{info, warn};

/// How many connects are timed per relay; the fastest one is used.
const SAMPLES: usize = 3;

/// Picks the relay host to connect to.
///
/// An explicit `preferred` id or name always wins, otherwise the relay with the lowest
/// connect latency is chosen. Falls back to the panel's default relay if none answers.
pub async fn pick_relay(info: &RelayInfoResponse, preferred: Option<&str>) -> Result<String> {
    if let Some(wanted) = preferred {
        return match info
            .relays
            .iter()
            .find(|relay| relay.id == wanted || relay.name.eq_ignore_ascii_case(wanted))
        {
            Some(relay) => Ok(relay.host.clone()),
            None => bail!(
                "unknown relay '{wanted}', available: {}",
                info.relays
                    .iter()
                    .map(|relay| format!("{} ({})", relay.name, relay.id))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        };
    }

    match info.relays.as_slice() {
        [] => Ok(info.relay_url.clone()),
        [only] => Ok(only.host.clone()),
        relays => {
            let results = measure_all(relays).await;

            let mut best: Option<(&RelayEntry, Duration)> = None;
            for (relay, latency) in relays.iter().zip(results) {
                match latency {
                    Some(latency) => {
                        info!(
                            "{} ({}): {} ms",
                            relay.name,
                            relay.region_label(),
                            latency.as_millis()
                        );
                        if best.is_none_or(|(_, current)| latency < current) {
                            best = Some((relay, latency));
                        }
                    }
                    None => warn!("{} ({}): unreachable", relay.name, relay.region_label()),
                }
            }

            match best {
                Some((relay, _)) => {
                    info!("using relay {}", relay.name);
                    Ok(relay.host.clone())
                }
                None => Ok(info.relay_url.clone()),
            }
        }
    }
}

/// Measures all relays concurrently, keeping the input order.
async fn measure_all(relays: &[RelayEntry]) -> Vec<Option<Duration>> {
    let tasks: Vec<_> = relays
        .iter()
        .map(|relay| tokio::spawn(measure(relay.host.clone())))
        .collect();

    let mut results = Vec::with_capacity(tasks.len());
    for task in tasks {
        results.push(task.await.ok().flatten());
    }
    results
}

/// Fastest TCP connect time to a relay's control port, or `None` if it is unreachable.
async fn measure(host: String) -> Option<Duration> {
    let mut fastest: Option<Duration> = None;
    for _ in 0..SAMPLES {
        let start = Instant::now();
        let connected = timeout(
            NETWORK_TIMEOUT,
            TcpStream::connect((host.as_str(), RELAY_PORT)),
        )
        .await;
        if !matches!(connected, Ok(Ok(_))) {
            // Don't wait out more timeouts on a relay that is down
            break;
        }
        let elapsed = start.elapsed();
        fastest = Some(fastest.map_or(elapsed, |current| current.min(elapsed)));
    }
    fastest
}
