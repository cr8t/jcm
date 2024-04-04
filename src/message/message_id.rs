use std::{fmt, mem};

use crate::{Error, Result};

const MESSAGE: u8 = 0x12;
const RESERVED: u8 = 0xff;

/// Represents the `ID` field of a message.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageId {
    Message = MESSAGE,
    Reserved = RESERVED,
}

impl MessageId {
    /// Creates a new [MessageId].
    pub const fn new() -> Self {
        Self::Message
    }

    /// Infallible conversion from a [`u8`] into a [MessageId].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            MESSAGE => Self::Message,
            _ => Self::Reserved,
        }
    }

    /// Gets the length of the [MessageId].
    pub const fn len() -> usize {
        mem::size_of::<u8>()
    }

    /// Gets whether the [MessageId] contains a reserved variant.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Reserved)
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MessageId> for u8 {
    fn from(val: MessageId) -> Self {
        val as Self
    }
}

impl From<&MessageId> for u8 {
    fn from(val: &MessageId) -> Self {
        (*val).into()
    }
}

impl TryFrom<u8> for MessageId {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidMessageId(val)),
            v => Ok(v),
        }
    }
}

impl From<&MessageId> for &'static str {
    fn from(val: &MessageId) -> Self {
        match val {
            MessageId::Message => "message",
            MessageId::Reserved => "reserved",
        }
    }
}

impl From<MessageId> for &'static str {
    fn from(val: MessageId) -> Self {
        (&val).into()
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_id() {
        let raw_denom = [MESSAGE];
        let expected = [MessageId::Message];

        for (raw, exp) in raw_denom.into_iter().zip(expected.into_iter()) {
            assert_eq!(MessageId::try_from(raw), Ok(exp));
            assert_eq!(MessageId::from_u8(raw), exp);
        }

        for stat in (0..=255u8).filter(|s| !raw_denom.iter().any(|d| d == s)) {
            assert!(MessageId::try_from(stat).is_err());
            assert_eq!(MessageId::from_u8(stat), MessageId::Reserved);
        }
    }
}
