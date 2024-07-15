use crate::{Error, Result};

const MS100: u8 = 1;
const MS200: u8 = 2;
const MS300: u8 = 3;
const MS400: u8 = 4;
const MS500: u8 = 5;
const MS600: u8 = 6;
const MS700: u8 = 7;
const MS800: u8 = 8;
const MS900: u8 = 9;
const MS1000: u8 = 10;
const MS1100: u8 = 11;
const MS1200: u8 = 12;
const MS1300: u8 = 13;
const MS1400: u8 = 14;
const MS1500: u8 = 15;

/// Represents the interval for resending device events.
///
/// Intervals are in 100ms increments.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventResendInterval {
    Ms100 = MS100,
    Ms200 = MS200,
    Ms300 = MS300,
    Ms400 = MS400,
    Ms500 = MS500,
    Ms600 = MS600,
    Ms700 = MS700,
    Ms800 = MS800,
    Ms900 = MS900,
    Ms1000 = MS1000,
    Ms1100 = MS1100,
    Ms1200 = MS1200,
    Ms1300 = MS1300,
    Ms1400 = MS1400,
    Ms1500 = MS1500,
}

impl EventResendInterval {
    /// Creates a new [EventResendInterval].
    pub const fn new() -> Self {
        Self::Ms100
    }

    /// Attempts to convert a [`u8`] into an [EventResendInterval].
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
            MS100 => Some(Self::Ms100),
            MS200 => Some(Self::Ms200),
            MS300 => Some(Self::Ms300),
            MS400 => Some(Self::Ms400),
            MS500 => Some(Self::Ms500),
            MS600 => Some(Self::Ms600),
            MS700 => Some(Self::Ms700),
            MS800 => Some(Self::Ms800),
            MS900 => Some(Self::Ms900),
            MS1000 => Some(Self::Ms1000),
            MS1100 => Some(Self::Ms1100),
            MS1200 => Some(Self::Ms1200),
            MS1300 => Some(Self::Ms1300),
            MS1400 => Some(Self::Ms1400),
            MS1500 => Some(Self::Ms1500),
            _ => None,
        }
    }

    /// Converts an [EventResendInterval] into a [`u8`].
    pub const fn into_u8(self) -> u8 {
        self as u8
    }
}

impl Default for EventResendInterval {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u8> for EventResendInterval {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        Self::from_u8(val).ok_or(Error::InvalidValue {
            field: "event_resend_interval",
            value: val as usize,
        })
    }
}

impl From<EventResendInterval> for u8 {
    fn from(val: EventResendInterval) -> Self {
        val.into_u8()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: [u8; 15] = [
        MS100, MS200, MS300, MS400, MS500, MS600, MS700, MS800, MS900, MS1000, MS1100, MS1200,
        MS1300, MS1400, MS1500,
    ];

    #[test]
    fn test_event_resend_interval() {
        RAW.into_iter()
            .zip([
                EventResendInterval::Ms100,
                EventResendInterval::Ms200,
                EventResendInterval::Ms300,
                EventResendInterval::Ms400,
                EventResendInterval::Ms500,
                EventResendInterval::Ms600,
                EventResendInterval::Ms700,
                EventResendInterval::Ms800,
                EventResendInterval::Ms900,
                EventResendInterval::Ms1000,
                EventResendInterval::Ms1100,
                EventResendInterval::Ms1200,
                EventResendInterval::Ms1300,
                EventResendInterval::Ms1400,
                EventResendInterval::Ms1500,
            ])
            .for_each(|(raw, exp)| {
                assert_eq!(EventResendInterval::from_u8(raw), Some(exp));
                assert_eq!(EventResendInterval::try_from(raw), Ok(exp));
                assert_eq!(exp.into_u8(), raw);
                assert_eq!(u8::from(exp), raw);
            });
    }

    #[test]
    fn test_event_resend_interval_invalid() {
        let field = "event_resend_interval";

        (0..=u8::MAX)
            .filter(|i| !RAW.iter().any(|raw| raw == i))
            .for_each(|invalid| {
                assert_eq!(EventResendInterval::from_u8(invalid), None);
                assert_eq!(
                    EventResendInterval::try_from(invalid),
                    Err(Error::InvalidValue {
                        field,
                        value: invalid as usize
                    })
                );
            });
    }
}
