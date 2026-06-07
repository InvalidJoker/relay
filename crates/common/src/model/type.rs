#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RelayType {
    /// HTTP relay
    Http,
    /// TCP relay
    Tcp,
    /// UDP relay
    Udp,
    /// WebSocket relay
    Ws,
}