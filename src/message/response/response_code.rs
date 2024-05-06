use std::{fmt, mem};

use crate::{Error, Result};

const ACK: u8 = 0x06;
const NAK: u8 = 0x15;
const BUSY: u8 = 0xe1;
const UNSUPPORTED: u8 = 0xe2;
const UNAVAILABLE: u8 = 0xe3;
const COLLISION: u8 = 0xe4;
const RESERVED: u8 = 0xff;

/// Represents response code variants for reponse messages.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseCode {
    /// Affirmative response.
    Ack = ACK,
    /// Negative response.
    Nak = NAK,
    /// Device is handling another Message, no request acceptable.
    Busy = BUSY,
    /// Unsupported `Function Code`, `UID`, and `Request Message`.
    Unsupported = UNSUPPORTED,
    /// Device is not accepting requests.
    Unavailable = UNAVAILABLE,
    /// Message conflict, e.g. receiving a message in the middle of sending a message.
    Collision = COLLISION,
    /// Reserved response code.
    Reserved = RESERVED,
}

impl ResponseCode {
    /// Creates a new [ResponseCode].
    pub const fn new() -> Self {
        Self::Ack
    }

    /// Infallible conversion from a [`u8`] into a [ResponseCode].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            ACK => Self::Ack,
            NAK => Self::Nak,
            BUSY => Self::Busy,
            UNSUPPORTED => Self::Unsupported,
            UNAVAILABLE => Self::Unavailable,
            COLLISION => Self::Collision,
            _ => Self::Reserved,
        }
    }

    /// Gets the length of the [ResponseCode].
    pub const fn len() -> usize {
        mem::size_of::<u8>()
    }

    /// Gets whether the [ResponseCode] contains a reserved variant.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Reserved)
    }

    /// Gets whether the [ResponseCode] is a valid variant.
    pub const fn is_valid(&self) -> bool {
        !self.is_empty()
    }
}

impl From<ResponseCode> for u8 {
    fn from(val: ResponseCode) -> Self {
        val as Self
    }
}

impl From<&ResponseCode> for u8 {
    fn from(val: &ResponseCode) -> Self {
        (*val).into()
    }
}

impl TryFrom<u8> for ResponseCode {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidResponseCode(val)),
            v => Ok(v),
        }
    }
}

impl From<&ResponseCode> for &'static str {
    fn from(val: &ResponseCode) -> Self {
        match val {
            ResponseCode::Ack => "affirmative response",
            ResponseCode::Nak => "negative response",
            ResponseCode::Busy => "device is handling another Message, no request acceptable",
            ResponseCode::Unsupported => {
                "unsupported `Function Code`, `UID`, and `Request Message`"
            }
            ResponseCode::Unavailable => "device is not accepting requests",
            ResponseCode::Collision => {
                "message conflict, e.g. receiving a message in the middle of sending a message"
            }
            ResponseCode::Reserved => "reserved",
        }
    }
}

impl From<ResponseCode> for &'static str {
    fn from(val: ResponseCode) -> Self {
        (&val).into()
    }
}

impl Default for ResponseCode {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_code() {
        let raw_vals = [ACK, NAK, BUSY, UNSUPPORTED, UNAVAILABLE, COLLISION];
        let expected = [
            ResponseCode::Ack,
            ResponseCode::Nak,
            ResponseCode::Busy,
            ResponseCode::Unsupported,
            ResponseCode::Unavailable,
            ResponseCode::Collision,
        ];

        for (raw, exp) in raw_vals.into_iter().zip(expected.into_iter()) {
            assert_eq!(ResponseCode::try_from(raw), Ok(exp));
            assert_eq!(ResponseCode::from_u8(raw), exp);

            assert!(!exp.is_empty());
            assert!(exp.is_valid());
        }

        for val in (0..=255u8).filter(|s| !raw_vals.iter().any(|d| d == s)) {
            assert!(ResponseCode::try_from(val).is_err());
            assert_eq!(ResponseCode::from_u8(val), ResponseCode::Reserved);
        }
    }
}
