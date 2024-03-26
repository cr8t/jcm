use std::{fmt, mem};

use crate::{Error, Result};

const COMMON: u8 = 0b0000;
const ACCEPTOR: u8 = 0b0001;
const RECYCLER: u8 = 0b0010;
const ESCROW: u8 = 0b0011;
const RESERVED: u8 = 0xff;

/// Represents the JCM device configuration ID.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FuncId {
    /// Common (entire device).
    Common = COMMON,
    /// Acceptor function.
    Acceptor = ACCEPTOR,
    /// Recycler function.
    Recycler = RECYCLER,
    /// Escrow function.
    Escrow = ESCROW,
    /// Reserved function.
    Reserved = RESERVED,
}

impl FuncId {
    /// Creates a new [FuncId].
    pub const fn new() -> Self {
        Self::Common
    }

    /// Infallible conversion from a [`u8`] into a [FuncId].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            COMMON => Self::Common,
            ACCEPTOR => Self::Acceptor,
            RECYCLER => Self::Recycler,
            ESCROW => Self::Escrow,
            _ => Self::Reserved,
        }
    }

    /// Gets the length of the [FuncId].
    pub const fn len() -> usize {
        mem::size_of::<u8>()
    }

    /// Gets whether the [FuncId] contains a reserved variant.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Reserved)
    }
}

impl Default for FuncId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u8> for FuncId {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidFuncId(val)),
            v => Ok(v),
        }
    }
}

impl From<&FuncId> for &'static str {
    fn from(val: &FuncId) -> Self {
        match val {
            FuncId::Common => "common",
            FuncId::Acceptor => "acceptor",
            FuncId::Recycler => "recycler",
            FuncId::Escrow => "escrow",
            FuncId::Reserved => "reserved",
        }
    }
}

impl From<FuncId> for &'static str {
    fn from(val: FuncId) -> Self {
        (&val).into()
    }
}

impl fmt::Display for FuncId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_id() {
        let raw_denom = [COMMON, ACCEPTOR, RECYCLER, ESCROW];
        let expected = [
            FuncId::Common,
            FuncId::Acceptor,
            FuncId::Recycler,
            FuncId::Escrow,
        ];

        for (raw, exp) in raw_denom.into_iter().zip(expected.into_iter()) {
            assert_eq!(FuncId::try_from(raw), Ok(exp));
            assert_eq!(FuncId::from_u8(raw), exp);
        }

        for stat in (0..=255u8).filter(|s| raw_denom.iter().find(|d| d == &s).is_none()) {
            assert!(FuncId::try_from(stat).is_err());
            assert_eq!(FuncId::from_u8(stat), FuncId::Reserved);
        }
    }
}
