use std::fmt;

use super::*;
use crate::{Error, Result};

const EVENT_SEQUENCE0: u8 = 0x80;
const EVENT_SEQUENCE1: u8 = 0x81;
const EVENT_SEQUENCE2: u8 = 0x82;
const EVENT_SEQUENCE3: u8 = 0x83;
const EVENT_SEQUENCE4: u8 = 0x84;
const EVENT_SEQUENCE5: u8 = 0x85;
const EVENT_SEQUENCE6: u8 = 0x86;
const EVENT_SEQUENCE7: u8 = 0x87;
const EVENT_SEQUENCE8: u8 = 0x88;
const EVENT_SEQUENCE9: u8 = 0x89;
const EVENT_SEQUENCE10: u8 = 0x8a;
const EVENT_SEQUENCE11: u8 = 0x8b;
const EVENT_SEQUENCE12: u8 = 0x8c;
const EVENT_SEQUENCE13: u8 = 0x8d;
const EVENT_SEQUENCE14: u8 = 0x8e;
const EVENT_SEQUENCE15: u8 = 0x8f;

/// Represents the sequence number of the device-to-host event status message.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventType {
    Sequence0 = EVENT_SEQUENCE0,
    Sequence1 = EVENT_SEQUENCE1,
    Sequence2 = EVENT_SEQUENCE2,
    Sequence3 = EVENT_SEQUENCE3,
    Sequence4 = EVENT_SEQUENCE4,
    Sequence5 = EVENT_SEQUENCE5,
    Sequence6 = EVENT_SEQUENCE6,
    Sequence7 = EVENT_SEQUENCE7,
    Sequence8 = EVENT_SEQUENCE8,
    Sequence9 = EVENT_SEQUENCE9,
    Sequence10 = EVENT_SEQUENCE10,
    Sequence11 = EVENT_SEQUENCE11,
    Sequence12 = EVENT_SEQUENCE12,
    Sequence13 = EVENT_SEQUENCE13,
    Sequence14 = EVENT_SEQUENCE14,
    Sequence15 = EVENT_SEQUENCE15,
    Reserved = RESERVED,
}

impl EventType {
    /// Creates a new [EventType].
    pub const fn new() -> Self {
        Self::Sequence0
    }

    /// Infallible conversion from a [`u8`] into a [EventType].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            EVENT_SEQUENCE0 => Self::Sequence0,
            EVENT_SEQUENCE1 => Self::Sequence1,
            EVENT_SEQUENCE2 => Self::Sequence2,
            EVENT_SEQUENCE3 => Self::Sequence3,
            EVENT_SEQUENCE4 => Self::Sequence4,
            EVENT_SEQUENCE5 => Self::Sequence5,
            EVENT_SEQUENCE6 => Self::Sequence6,
            EVENT_SEQUENCE7 => Self::Sequence7,
            EVENT_SEQUENCE8 => Self::Sequence8,
            EVENT_SEQUENCE9 => Self::Sequence9,
            EVENT_SEQUENCE10 => Self::Sequence10,
            EVENT_SEQUENCE11 => Self::Sequence11,
            EVENT_SEQUENCE12 => Self::Sequence12,
            EVENT_SEQUENCE13 => Self::Sequence13,
            EVENT_SEQUENCE14 => Self::Sequence14,
            EVENT_SEQUENCE15 => Self::Sequence15,
            _ => Self::Reserved,
        }
    }
}

impl TryFrom<u8> for EventType {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidMessageType(val)),
            v => Ok(v),
        }
    }
}

impl From<&EventType> for &'static str {
    fn from(val: &EventType) -> Self {
        match val {
            EventType::Sequence0 => "event sequence 0",
            EventType::Sequence1 => "event sequence 1",
            EventType::Sequence2 => "event sequence 2",
            EventType::Sequence3 => "event sequence 3",
            EventType::Sequence4 => "event sequence 4",
            EventType::Sequence5 => "event sequence 5",
            EventType::Sequence6 => "event sequence 6",
            EventType::Sequence7 => "event sequence 7",
            EventType::Sequence8 => "event sequence 8",
            EventType::Sequence9 => "event sequence 9",
            EventType::Sequence10 => "event sequence 10",
            EventType::Sequence11 => "event sequence 11",
            EventType::Sequence12 => "event sequence 12",
            EventType::Sequence13 => "event sequence 13",
            EventType::Sequence14 => "event sequence 14",
            EventType::Sequence15 => "event sequence 15",
            EventType::Reserved => "reserved",
        }
    }
}

impl From<EventType> for &'static str {
    fn from(val: EventType) -> Self {
        (&val).into()
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type() {
        let raw_denom = [
            EVENT_SEQUENCE0,
            EVENT_SEQUENCE1,
            EVENT_SEQUENCE2,
            EVENT_SEQUENCE3,
            EVENT_SEQUENCE4,
            EVENT_SEQUENCE5,
            EVENT_SEQUENCE6,
            EVENT_SEQUENCE7,
            EVENT_SEQUENCE8,
            EVENT_SEQUENCE9,
            EVENT_SEQUENCE10,
            EVENT_SEQUENCE11,
            EVENT_SEQUENCE12,
            EVENT_SEQUENCE13,
            EVENT_SEQUENCE14,
            EVENT_SEQUENCE15,
        ];
        let expected = [
            EventType::Sequence0,
            EventType::Sequence1,
            EventType::Sequence2,
            EventType::Sequence3,
            EventType::Sequence4,
            EventType::Sequence5,
            EventType::Sequence6,
            EventType::Sequence7,
            EventType::Sequence8,
            EventType::Sequence9,
            EventType::Sequence10,
            EventType::Sequence11,
            EventType::Sequence12,
            EventType::Sequence13,
            EventType::Sequence14,
            EventType::Sequence15,
        ];

        for (raw, exp) in raw_denom.into_iter().zip(expected.into_iter()) {
            assert_eq!(EventType::try_from(raw), Ok(exp));
            assert_eq!(EventType::from_u8(raw), exp);
        }

        for stat in (0..=255u8).filter(|s| raw_denom.iter().find(|d| d == &s).is_none()) {
            assert!(EventType::try_from(stat).is_err());
            assert_eq!(EventType::from_u8(stat), EventType::Reserved);
        }
    }
}
