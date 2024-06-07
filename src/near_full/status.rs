use crate::{Error, Result};

/// Represents the length of [NearFullStatus].
pub const NEAR_FULL_STATUS_LEN: usize = 1;

const DISABLED: u8 = 0;
const ENABLED: u8 = 1;

/// Represents whether the `Near Full` feature is enabled.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NearFullStatus {
    Disabled = DISABLED,
    Enabled = ENABLED,
}

impl NearFullStatus {
    /// Creates a new [NearFullStatus].
    pub const fn new() -> Self {
        Self::Disabled
    }

    /// Attempts to convert a [`u8`] into a [NearFullStatus].
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
            DISABLED => Some(Self::Disabled),
            ENABLED => Some(Self::Enabled),
            _ => None,
        }
    }

    /// Converts a [NearFullStatus] into a [`u8`].
    pub const fn into_u8(self) -> u8 {
        self as u8
    }
}

impl Default for NearFullStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u8> for NearFullStatus {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        Self::from_u8(val).ok_or(Error::InvalidNearFullStatus(val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: [u8; 2] = [DISABLED, ENABLED];

    #[test]
    fn test_near_full_status() {
        RAW.into_iter()
            .zip([NearFullStatus::Disabled, NearFullStatus::Enabled])
            .for_each(|(raw, exp)| {
                assert_eq!(NearFullStatus::from_u8(raw), Some(exp));
                assert_eq!(NearFullStatus::try_from(raw), Ok(exp));
                assert_eq!(exp.into_u8(), raw);
            });
    }

    #[test]
    fn test_near_full_status_invalid() {
        (0..=u8::MAX)
            .filter(|i| !RAW.iter().any(|r| r == i))
            .for_each(|invalid| {
                assert_eq!(NearFullStatus::from_u8(invalid), None);
                assert_eq!(
                    NearFullStatus::try_from(invalid),
                    Err(Error::InvalidNearFullStatus(invalid))
                );
            });
    }
}
