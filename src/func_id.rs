use std::{fmt, mem};

use crate::{Error, Result};

const COMMON: u8 = 0b0000;
const ACCEPTOR: u8 = 0b0001;
const RECYCLER: u8 = 0b0010;
const ESCROW: u8 = 0b0011;
const RESERVED: u8 = 0xff;
const STATUS_SHIFT: u8 = 12;

/// Represents the function ID of the JCM device.
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

    /// Infallible conversion from a [`u16`] into a [FuncId].
    pub const fn from_u16(val: u16) -> Self {
        Self::from_u8((val >> STATUS_SHIFT) as u8)
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

impl TryFrom<u8> for FuncId {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidFuncId(val)),
            v => Ok(v),
        }
    }
}

impl TryFrom<u16> for FuncId {
    type Error = Error;

    fn try_from(val: u16) -> Result<Self> {
        Self::try_from((val >> STATUS_SHIFT) as u8)
    }
}

impl From<FuncId> for u16 {
    fn from(val: FuncId) -> Self {
        (val as u16) << STATUS_SHIFT
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

impl Default for FuncId {
    fn default() -> Self {
        Self::new()
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
    fn test_func_id() {
        let raw_vals = [COMMON, ACCEPTOR, RECYCLER, ESCROW];
        let expected = [
            FuncId::Common,
            FuncId::Acceptor,
            FuncId::Recycler,
            FuncId::Escrow,
        ];

        for (raw, exp) in raw_vals.into_iter().zip(expected.into_iter()) {
            assert_eq!(FuncId::try_from(raw), Ok(exp));
            assert_eq!(FuncId::from_u8(raw), exp);
        }

        for val in (0..=255u8).filter(|s| !raw_vals.iter().any(|d| d == s)) {
            assert!(FuncId::try_from(val).is_err());
            assert_eq!(FuncId::from_u8(val), FuncId::Reserved);
        }
    }
}
