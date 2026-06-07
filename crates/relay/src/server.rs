use std::ops::RangeInclusive;

pub struct Server {
    /// The port range to listen on.
    port_range: RangeInclusive<u16>,
}

impl Server {
    pub fn new(port_range: RangeInclusive<u16>) -> Self {
        Self { port_range }
    }
}