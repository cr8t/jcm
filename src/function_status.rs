use std::{fmt, mem};

use crate::{Error, Result};

mod function_errors;
mod unit_availability;

pub use function_errors::*;
pub use unit_availability::*;

const UNIT_AVAIL_MASK: u8 = 0x80;
const UNIT_AVAIL_SHIFT: u8 = 7;

const ERRORS_MASK: u8 = 0x40;
const ERRORS_SHIFT: u8 = 6;

const DETAIL_MASK: u8 = 0x3f;

const NORMAL: u8 = 0x00;
const NEAR_FULL: u8 = 0x01;
const FULL: u8 = 0xc1;
const BOX_REMOVED: u8 = 0xc2;
const JAM_ACCEPTOR: u8 = 0xc3;
const JAM_STACKER: u8 = 0xc4;
const CHEAT: u8 = 0xc5;
const UNIT_REMOVED: u8 = 0xc6;
const FAILURE: u8 = 0xff;
const RESERVED: u8 = 0xc0;

/// Represents the function status of device unit.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FunctionStatus {
    /// Unit is functional.
    Normal = NORMAL,
    /// Recycler Unit is almost full.
    NearFull = NEAR_FULL,
    /// Recycler Unit is full.
    Full = FULL,
    /// Cash Box has been removed.
    BoxRemoved = BOX_REMOVED,
    /// A jam detected in the Acceptor unit.
    JamAcceptor = JAM_ACCEPTOR,
    /// A jam detected in the Stacker unit.
    JamStacker = JAM_STACKER,
    /// Fraud detected.
    Cheat = CHEAT,
    /// One of the units is removed.
    UnitRemoved = UNIT_REMOVED,
    /// Unit error.
    Failure = FAILURE,
    /// Reserved value.
    Reserved = RESERVED,
}

impl FunctionStatus {
    /// Creates a new [FunctionStatus].
    pub const fn new() -> Self {
        Self::Normal
    }

    /// Gets the [UnitAvailability] of the [FunctionStatus].
    pub const fn unit_availability(&self) -> UnitAvailability {
        UnitAvailability::from_u8((self.to_u8() & UNIT_AVAIL_MASK) >> UNIT_AVAIL_SHIFT)
    }

    /// Gets the [FunctionErrors] of the [FunctionStatus].
    pub const fn errors(&self) -> FunctionErrors {
        FunctionErrors::from_u8((self.to_u8() & ERRORS_MASK) >> ERRORS_SHIFT)
    }

    /// Gets the details of the [FunctionStatus].
    pub const fn details(&self) -> u8 {
        self.to_u8() & DETAIL_MASK
    }

    /// Infallible function that converts a [`u8`] into a [FunctionStatus].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            NORMAL => Self::Normal,
            NEAR_FULL => Self::NearFull,
            FULL => Self::Full,
            BOX_REMOVED => Self::BoxRemoved,
            JAM_ACCEPTOR => Self::JamAcceptor,
            JAM_STACKER => Self::JamStacker,
            CHEAT => Self::Cheat,
            UNIT_REMOVED => Self::UnitRemoved,
            FAILURE => Self::Failure,
            _ => Self::Reserved,
        }
    }

    /// Converts the [FunctionStatus] into a [`u8`].
    pub const fn to_u8(&self) -> u8 {
        *self as u8
    }

    /// Converts the [FunctionStatus] into a [`u8`].
    pub const fn into_u8(self) -> u8 {
        self as u8
    }

    /// Gets the length of the [FunctionStatus].
    pub const fn len() -> usize {
        mem::size_of::<u8>()
    }

    /// Gets whether the [FunctionStatus] is empty.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Reserved)
    }

    /// Gets whether the [FunctionStatus] is valid.
    pub const fn is_valid(&self) -> bool {
        !self.is_empty()
    }
}

impl TryFrom<u8> for FunctionStatus {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidFunctionStatus(val)),
            fs => Ok(fs),
        }
    }
}

impl From<FunctionStatus> for u8 {
    fn from(val: FunctionStatus) -> u8 {
        val.into_u8()
    }
}

impl From<&FunctionStatus> for u8 {
    fn from(val: &FunctionStatus) -> u8 {
        val.to_u8()
    }
}

impl From<FunctionStatus> for &'static str {
    fn from(val: FunctionStatus) -> Self {
        match val {
            FunctionStatus::Normal => "normal",
            FunctionStatus::NearFull => "near full",
            FunctionStatus::Full => "full",
            FunctionStatus::BoxRemoved => "box removed",
            FunctionStatus::JamAcceptor => "jam acceptor",
            FunctionStatus::JamStacker => "jam stacker",
            FunctionStatus::Cheat => "cheat",
            FunctionStatus::UnitRemoved => "unit removed",
            FunctionStatus::Failure => "failure",
            FunctionStatus::Reserved => "reserved",
        }
    }
}

impl From<&FunctionStatus> for &'static str {
    fn from(val: &FunctionStatus) -> Self {
        (*val).into()
    }
}

impl fmt::Display for FunctionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

/// Convenience wrapper for a detailed view of a [FunctionStatus].
pub struct FunctionStatusDetails(pub FunctionStatus);

impl fmt::Display for FunctionStatusDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(
            f,
            r#""unit_availability": {}, "#,
            self.0.unit_availability()
        )?;
        write!(f, r#""errors": {}, "#, self.0.errors())?;
        write!(f, r#""details": {:#x}"#, self.0.details())?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_status() {
        let raw_vals = [
            NORMAL,
            NEAR_FULL,
            FULL,
            BOX_REMOVED,
            JAM_ACCEPTOR,
            JAM_STACKER,
            CHEAT,
            UNIT_REMOVED,
            FAILURE,
        ];

        let exp_vals = [
            FunctionStatus::Normal,
            FunctionStatus::NearFull,
            FunctionStatus::Full,
            FunctionStatus::BoxRemoved,
            FunctionStatus::JamAcceptor,
            FunctionStatus::JamStacker,
            FunctionStatus::Cheat,
            FunctionStatus::UnitRemoved,
            FunctionStatus::Failure,
        ];

        for (raw, exp) in raw_vals.into_iter().zip(exp_vals.into_iter()) {
            assert_eq!(FunctionStatus::from_u8(raw), exp);
            assert_eq!(FunctionStatus::try_from(raw), Ok(exp));

            assert!(exp.is_valid());
            assert!(!exp.is_empty());
        }

        for invalid in (0..=0xffu8).filter(|f| !raw_vals.iter().any(|v| v == f)) {
            let res_status = FunctionStatus::from_u8(invalid);

            assert_eq!(res_status, FunctionStatus::Reserved);

            assert!(!res_status.is_valid());
            assert!(res_status.is_empty());

            assert!(FunctionStatus::try_from(invalid).is_err());
        }
    }
}
