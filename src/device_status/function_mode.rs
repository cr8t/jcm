use std::{fmt, mem};

use crate::{Error, Result};

const COMMON: u8 = 0b0000;
const ACCEPTOR: u8 = 0b0001;
const RECYCLER: u8 = 0b0010;
const ESCROW: u8 = 0b0011;
const RESERVED: u8 = 0xff;
const STATUS_SHIFT: u8 = 12;

/// Represents the function mode of the JCM device.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FunctionMode {
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

impl FunctionMode {
    /// Creates a new [FunctionMode].
    pub const fn new() -> Self {
        Self::Common
    }

    /// Infallible conversion from a [`u8`] into a [FunctionMode].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            COMMON => Self::Common,
            ACCEPTOR => Self::Acceptor,
            RECYCLER => Self::Recycler,
            ESCROW => Self::Escrow,
            _ => Self::Reserved,
        }
    }

    /// Infallible conversion from a [`u16`] into a [FunctionMode].
    pub const fn from_u16(val: u16) -> Self {
        Self::from_u8((val >> STATUS_SHIFT) as u8)
    }

    /// Gets the length of the [FunctionMode].
    pub const fn len() -> usize {
        mem::size_of::<u8>()
    }

    /// Gets whether the [FunctionMode] contains a reserved variant.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Reserved)
    }
}

impl TryFrom<u8> for FunctionMode {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidFunctionMode(val)),
            v => Ok(v),
        }
    }
}

impl TryFrom<u16> for FunctionMode {
    type Error = Error;

    fn try_from(val: u16) -> Result<Self> {
        Self::try_from((val >> STATUS_SHIFT) as u8)
    }
}

impl From<FunctionMode> for u16 {
    fn from(val: FunctionMode) -> Self {
        (val as u16) << STATUS_SHIFT
    }
}

impl From<&FunctionMode> for &'static str {
    fn from(val: &FunctionMode) -> Self {
        match val {
            FunctionMode::Common => "common",
            FunctionMode::Acceptor => "acceptor",
            FunctionMode::Recycler => "recycler",
            FunctionMode::Escrow => "escrow",
            FunctionMode::Reserved => "reserved",
        }
    }
}

impl From<FunctionMode> for &'static str {
    fn from(val: FunctionMode) -> Self {
        (&val).into()
    }
}

impl fmt::Display for FunctionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_mode() {
        let raw_denom = [COMMON, ACCEPTOR, RECYCLER, ESCROW];
        let expected = [
            FunctionMode::Common,
            FunctionMode::Acceptor,
            FunctionMode::Recycler,
            FunctionMode::Escrow,
        ];

        for (raw, exp) in raw_denom.into_iter().zip(expected.into_iter()) {
            assert_eq!(FunctionMode::try_from(raw), Ok(exp));
            assert_eq!(FunctionMode::from_u8(raw), exp);
        }

        for stat in (0..=255u8).filter(|s| !raw_denom.iter().any(|d| d == s)) {
            assert!(FunctionMode::try_from(stat).is_err());
            assert_eq!(FunctionMode::from_u8(stat), FunctionMode::Reserved);
        }
    }
}
