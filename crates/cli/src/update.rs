use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, info};

const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/InvalidJoker/relay/releases/latest";

/// Cached answers are reused for this long, so the check doesn't hit GitHub on every start.
const CACHE_TTL: Duration = Duration::from_secs(24 * 60 * 60);

/// The check must never slow down or break the actual command.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(3);

pub const INSTALL_HINT: &str = "curl -fsSL https://raw.githubusercontent.com/InvalidJoker/relay/main/install.sh | sh\n  (Windows: irm https://raw.githubusercontent.com/InvalidJoker/relay/main/install.ps1 | iex)";

#[derive(Debug, Serialize, Deserialize)]
struct Release {
    tag_name: String,
    html_url: String,
}

#[derive(Serialize, Deserialize)]
struct Cache {
    checked_at: u64,
    release: Release,
}

fn cache_path() -> Option<PathBuf> {
    dirs::cache_dir().map(|dir| dir.join("relay").join("update.json"))
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}

fn read_cache() -> Option<Release> {
    let cache: Cache = serde_json::from_str(&std::fs::read_to_string(cache_path()?).ok()?).ok()?;
    (now().saturating_sub(cache.checked_at) < CACHE_TTL.as_secs()).then_some(cache.release)
}

fn write_cache(release: Release) {
    let Some(path) = cache_path() else { return };
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let cache = Cache {
        checked_at: now(),
        release,
    };
    if let Ok(content) = serde_json::to_string(&cache) {
        let _ = std::fs::write(path, content);
    }
}

async fn fetch_latest(client: &Client) -> anyhow::Result<Release> {
    Ok(client
        .get(LATEST_RELEASE_URL)
        .header("User-Agent", concat!("relay-cli/", env!("GIT_HASH")))
        .header("Accept", "application/vnd.github+json")
        .timeout(REQUEST_TIMEOUT)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

/// Release tags look like `v2026.10.11-<short sha>`, the sha is what identifies a build.
fn is_newer(tag: &str) -> bool {
    let current = env!("GIT_HASH");
    let Some(released) = tag.rsplit('-').next() else {
        return false;
    };
    !current.is_empty()
        && !released.is_empty()
        && !current.starts_with(released)
        && !released.starts_with(current)
}

/// Returns the latest release if it differs from this build.
async fn newer_release(client: &Client, use_cache: bool) -> anyhow::Result<Option<Release>> {
    let release = match use_cache.then(read_cache).flatten() {
        Some(release) => release,
        None => {
            let release = fetch_latest(client).await?;
            write_cache(Release {
                tag_name: release.tag_name.clone(),
                html_url: release.html_url.clone(),
            });
            release
        }
    };
    Ok(is_newer(&release.tag_name).then_some(release))
}

/// Logs a notice if a newer release exists. Silent on any failure, can be disabled with
/// `RELAY_NO_UPDATE_CHECK=1`.
pub async fn notify(client: Client) {
    if std::env::var_os("RELAY_NO_UPDATE_CHECK").is_some() {
        return;
    }
    match newer_release(&client, true).await {
        Ok(Some(release)) => info!(
            "A new version of relay is available ({}): run `relay update` for details",
            release.tag_name
        ),
        Ok(None) => {}
        Err(err) => debug!(%err, "update check failed"),
    }
}

/// The `relay update` command: always asks GitHub and prints how to upgrade.
pub async fn check(client: &Client) -> anyhow::Result<()> {
    let current = env!("GIT_HASH");
    match newer_release(client, false).await? {
        Some(release) => {
            info!("Installed build: {current}");
            info!(
                "Latest release: {} ({})",
                release.tag_name, release.html_url
            );
            info!("Update with:\n  {INSTALL_HINT}");
        }
        None => info!("relay is up to date ({current})"),
    }
    Ok(())
}
