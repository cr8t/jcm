use crate::{Error, Result};

/// Represents a set of denominations to disable on the device.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DenominationDisable(u16);

impl DenominationDisable {
    /// Creates a new [DenominationDisable].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [DenominationDisable] from the provided parameter.
    pub const fn create(val: u16) -> Self {
        Self(val)
    }

    /// Converts the [DenominationDisable] into its inner representation.
    pub const fn inner(&self) -> u16 {
        self.0
    }

    /// Consumes and converts the [DenominationDisable] into its inner representation.
    pub const fn into_inner(self) -> u16 {
        self.0
    }

    /// Gets the length of the [DenominationDisable].
    pub const fn len() -> usize {
        std::mem::size_of::<u16>()
    }

    /// Gets whether the [DenominationDisable] is empty.
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Gets the maximum denomination index for the [DenominationDisable].
    pub const fn max_denom() -> usize {
        Self::denom_len() - 1
    }

    /// Gets the number of denominations.
    pub const fn denom_len() -> usize {
        u16::BITS as usize
    }

    /// Gets whether a denomination is disabled.
    pub const fn is_disabled(&self, idx: usize) -> bool {
        if idx > Self::max_denom() {
            false
        } else {
            self.0 & (1 << idx) != 0
        }
    }

    /// Gets whether a denomination is enabled.
    pub const fn is_enabled(&self, idx: usize) -> bool {
        if idx > Self::max_denom() {
            false
        } else {
            self.0 & (1 << idx) == 0
        }
    }

    /// Sets whether the denomination is disabled.
    pub(crate) fn set(&mut self, idx: usize, disable: bool) {
        if disable {
            self.0 |= 1 << idx;
        } else {
            self.0 &= !(1 << idx);
        }
    }

    /// Builder function that sets whether the denomination is disabled.
    pub(crate) fn with_set(mut self, idx: usize, disable: bool) -> Self {
        self.set(idx, disable);
        self
    }

    /// Enables a denomination.
    pub fn enable(&mut self, idx: usize) {
        if idx <= Self::max_denom() {
            self.set(idx, false);
        }
    }

    /// Builder function that enables a denomination.
    pub fn with_enable(mut self, idx: usize) -> Self {
        self.enable(idx);
        self
    }

    /// Disables a denomination.
    pub fn disable(&mut self, idx: usize) {
        if idx <= Self::max_denom() {
            self.set(idx, true);
        }
    }

    /// Builder function that disables a denomination.
    pub fn with_disable(mut self, idx: usize) -> Self {
        self.disable(idx);
        self
    }

    /// Infallible conversion from byte buffer into a [DenominationDisable].
    pub fn from_bytes(val: &[u8]) -> Self {
        match val.len() {
            0 => Self::new(),
            1 => Self(u16::from_le_bytes([val[0], 0])),
            _ => Self(u16::from_le_bytes([val[0], val[1]])),
        }
    }

    /// Converts the [DenominationDisable] into a byte array.
    pub fn to_bytes(&self) -> [u8; 2] {
        self.0.to_le_bytes()
    }

    /// Consumes and converts the [DenominationDisable] into a byte array.
    pub fn into_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}

impl Default for DenominationDisable {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&[u8]> for DenominationDisable {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        match val.len() {
            2 => Ok(Self::from_bytes(val)),
            len => Err(Error::InvalidRequestLen((len, Self::len()))),
        }
    }
}

impl From<DenominationDisable> for [u8; 2] {
    fn from(val: DenominationDisable) -> Self {
        val.into_bytes()
    }
}

impl From<&DenominationDisable> for [u8; 2] {
    fn from(val: &DenominationDisable) -> Self {
        val.to_bytes()
    }
}
