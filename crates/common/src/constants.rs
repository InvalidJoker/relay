use std::time::Duration;

/// The port which the relay server listens on for starting a relay
pub const RELAY_PORT: u16 = 2550;

/// Timeout for network connections and initial protocol messages.
pub const NETWORK_TIMEOUT: Duration = Duration::from_secs(3);
