use std::{fmt, mem};

use super::*;
use crate::{Error, Result};

const OPERATION_REQ: u8 = 0b0000_0000;
const STATUS_REQ: u8 = 0b0001_0000;
const SET_FEATURE_REQ: u8 = 0b0010_0000;

/// Represents the type of host-to-device request message.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestType {
    Operation = OPERATION_REQ,
    Status = STATUS_REQ,
    SetFeature = SET_FEATURE_REQ,
    Reserved = RESERVED,
}

impl RequestType {
    /// Creates a new [RequestType].
    pub const fn new() -> Self {
        Self::Operation
    }

    /// Infallible conversion from a [`u8`] into a [RequestType].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            OPERATION_REQ => Self::Operation,
            STATUS_REQ => Self::Status,
            SET_FEATURE_REQ => Self::SetFeature,
            _ => Self::Reserved,
        }
    }

    /// Converts the [RequestType] to a [`u8`].
    pub const fn to_u8(&self) -> u8 {
        *self as u8
    }

    /// Gets the length of the [RequestType].
    pub const fn len() -> usize {
        mem::size_of::<u8>()
    }

    /// Gets whether the [RequestType] contains a reserved variant.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Reserved)
    }

    /// Gets whether the [RequestType] is a valid variant.
    pub const fn is_valid(&self) -> bool {
        !self.is_empty()
    }
}

impl TryFrom<u8> for RequestType {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidMessageType(val)),
            v => Ok(v),
        }
    }
}

impl From<&RequestType> for &'static str {
    fn from(val: &RequestType) -> Self {
        match val {
            RequestType::Operation => "operation",
            RequestType::Status => "status",
            RequestType::SetFeature => "set feature",
            RequestType::Reserved => "reserved",
        }
    }
}

impl From<RequestType> for &'static str {
    fn from(val: RequestType) -> Self {
        (&val).into()
    }
}

impl fmt::Display for RequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_type() {
        let raw_denom = [OPERATION_REQ, STATUS_REQ, SET_FEATURE_REQ];
        let expected = [
            RequestType::Operation,
            RequestType::Status,
            RequestType::SetFeature,
        ];

        for (raw, exp) in raw_denom.into_iter().zip(expected.into_iter()) {
            assert_eq!(RequestType::try_from(raw), Ok(exp));
            assert_eq!(RequestType::from_u8(raw), exp);
        }

        for stat in (0..=255u8).filter(|s| !raw_denom.iter().any(|d| d == s)) {
            assert!(RequestType::try_from(stat).is_err());
            assert_eq!(RequestType::from_u8(stat), RequestType::Reserved);
        }
    }
}
