use std::time::Duration;

/// The port which the relay server listens on for starting a relay
pub const RELAY_PORT: u16 = 2550;

/// Maximum byte length for a JSON frame in the stream.
pub const MAX_FRAME_LENGTH: usize = 256;

/// Timeout for network connections and initial protocol messages.
pub const NETWORK_TIMEOUT: Duration = Duration::from_secs(3);
