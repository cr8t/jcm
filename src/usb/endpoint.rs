/// Represents a simplified USB endpoint.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Endpoint {
    address: u8,
    max_packet_size: usize,
}

impl Endpoint {
    /// Creates a new [Endpoint].
    pub const fn new() -> Self {
        Self {
            address: 0,
            max_packet_size: 0,
        }
    }

    /// Creates a new [Endpoint] from the provided parameters.
    pub const fn create(address: u8, max_packet_size: usize) -> Self {
        Self {
            address,
            max_packet_size,
        }
    }

    /// Gets the [Endpoint] address.
    pub const fn address(&self) -> u8 {
        self.address
    }

    /// Builder function that sets the [Endpoint] address.
    pub const fn with_address(self, address: u8) -> Self {
        Self {
            address,
            max_packet_size: self.max_packet_size,
        }
    }

    /// Gets the [Endpoint] max packet size.
    pub const fn max_packet_size(&self) -> usize {
        self.max_packet_size
    }

    /// Builder function that sets the [Endpoint] max packet size.
    pub const fn with_max_packet_size(self, max_packet_size: usize) -> Self {
        Self {
            address: self.address,
            max_packet_size,
        }
    }
}

impl Default for Endpoint {
    fn default() -> Self {
        Self::new()
    }
}
