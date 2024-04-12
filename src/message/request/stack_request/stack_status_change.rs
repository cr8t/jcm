use std::{fmt, mem};

use crate::{Error, Result};

const IDLE: u8 = 0x0;
const INHIBIT: u8 = 0x1;
const RESERVED: u8 = 0xff;

/// Represents the device status change after completing the collection operation.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StackStatusChange {
    /// Device status changes to `Idle` after collection operation.
    Idle = IDLE,
    /// Device status changes to `Inhibit` after collection operation.
    Inhibit = INHIBIT,
    /// Reserved status change (invalid).
    Reserved = RESERVED,
}

impl StackStatusChange {
    /// Creates a new [StackStatusChange].
    pub const fn new() -> Self {
        Self::Idle
    }

    /// Infallible function to convert a [`u8`] into a [StackStatusChange].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            IDLE => Self::Idle,
            INHIBIT => Self::Inhibit,
            _ => Self::Reserved,
        }
    }

    /// Converts the [StackStatusChange] into a [`u8`].
    pub const fn to_u8(&self) -> u8 {
        *self as u8
    }

    /// Converts the [StackStatusChange] into a [`u8`].
    pub const fn into_u8(self) -> u8 {
        self as u8
    }

    /// Gets the length of the [StackStatusChange].
    pub const fn len() -> usize {
        mem::size_of::<u8>()
    }

    /// Gets whether the [StackStatusChange] is empty.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Reserved)
    }

    /// Gets whether the [StackStatusChange] is valid.
    pub const fn is_valid(&self) -> bool {
        !self.is_empty()
    }
}

impl Default for StackStatusChange {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u8> for StackStatusChange {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidStackStatusChange(val)),
            sc => Ok(sc),
        }
    }
}

impl From<StackStatusChange> for u8 {
    fn from(val: StackStatusChange) -> Self {
        val.into_u8()
    }
}

impl From<&StackStatusChange> for u8 {
    fn from(val: &StackStatusChange) -> Self {
        val.to_u8()
    }
}

impl From<StackStatusChange> for &'static str {
    fn from(val: StackStatusChange) -> Self {
        match val {
            StackStatusChange::Idle => "idle",
            StackStatusChange::Inhibit => "inhibit",
            StackStatusChange::Reserved => "reserved",
        }
    }
}

impl From<&StackStatusChange> for &'static str {
    fn from(val: &StackStatusChange) -> Self {
        (*val).into()
    }
}

impl fmt::Display for StackStatusChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_status_change() {
        let raw_vals = [IDLE, INHIBIT];
        let expected = [StackStatusChange::Idle, StackStatusChange::Inhibit];

        for (raw, exp) in raw_vals.into_iter().zip(expected.into_iter()) {
            assert_eq!(StackStatusChange::try_from(raw), Ok(exp));
            assert_eq!(StackStatusChange::from_u8(raw), exp);
        }

        for val in (0..=255u8).filter(|s| !raw_vals.iter().any(|d| d == s)) {
            assert!(StackStatusChange::try_from(val).is_err());
            assert_eq!(StackStatusChange::from_u8(val), StackStatusChange::Reserved);
        }
    }
}
