use crate::{Error, Result};

/// Default hold timeout (in seconds).
pub const DEFAULT_HOLD_TIMEOUT: u16 = 10;

/// Represents the timeout in seconds to hold a note in escrow.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HoldTimeout(u16);

impl HoldTimeout {
    /// Creates a new [HoldTimeout].
    pub const fn new() -> Self {
        Self(DEFAULT_HOLD_TIMEOUT)
    }

    /// Creates a new [HoldTimeout] from the provided parameter.
    pub const fn create(val: u16) -> Self {
        Self(val)
    }

    /// Gets the length of the [HoldTimeout].
    pub const fn len() -> usize {
        std::mem::size_of::<u16>()
    }

    /// Gets whether the [HoldTimeout] is empty.
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Converts a [`u16`] into a [HoldTimeout].
    pub const fn from_u16(val: u16) -> Self {
        Self(val)
    }

    /// Converts a [HoldTimeout] into a [`u16`].
    pub const fn to_u16(&self) -> u16 {
        self.0
    }

    /// Infallible conversion from byte buffer into a [HoldTimeout].
    pub const fn from_bytes(val: &[u8]) -> Self {
        match val.len() {
            0 => Self(0),
            1 => Self(u16::from_le_bytes([val[0], 0])),
            _ => Self(u16::from_le_bytes([val[0], val[1]])),
        }
    }

    /// Converts the [HoldTimeout] into a byte array.
    pub const fn to_bytes(&self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}

impl TryFrom<&[u8]> for HoldTimeout {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        match val.len() {
            len if len == Self::len() => Ok(Self::from_bytes(val)),
            len => Err(Error::InvalidRequestLen((len, Self::len()))),
        }
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for HoldTimeout {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for HoldTimeout {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl Default for HoldTimeout {
    fn default() -> Self {
        Self::new()
    }
}
