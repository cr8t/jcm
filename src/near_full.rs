use std::fmt;

use crate::{Error, Result};

mod number;
mod status;

pub use number::*;
pub use status::*;

/// Represents the length of [NearFullData].
pub const NEAR_FULL_DATA_LEN: usize = 3;

/// Represents the `Near Full` settings for the device.
///
/// When available, the `Near Full` settings indicate to send a `Near Full` event
/// to the host when the threshold number of banknotes are inserted into storage.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NearFullData {
    status: NearFullStatus,
    number: NearFullNumber,
}

impl NearFullData {
    /// Creates a new [NearFullData].
    pub const fn new() -> Self {
        Self {
            status: NearFullStatus::new(),
            number: NearFullNumber::new(),
        }
    }

    /// Gets the [NearFullStatus] of the [NearFullData].
    pub const fn status(&self) -> NearFullStatus {
        self.status
    }

    /// Sets the [NearFullStatus] of the [NearFullData].
    pub fn set_status(&mut self, status: NearFullStatus) {
        self.status = status;
    }

    /// Builder function that sets the [NearFullStatus] of the [NearFullData].
    pub const fn with_status(self, status: NearFullStatus) -> Self {
        Self {
            status,
            number: self.number,
        }
    }

    /// Gets the [NearFullNumber] of the [NearFullData].
    pub const fn number(&self) -> NearFullNumber {
        self.number
    }

    /// Sets the [NearFullNumber] of the [NearFullData].
    pub fn set_number(&mut self, number: NearFullNumber) {
        self.number = number;
    }

    /// Builder function that sets the [NearFullNumber] of the [NearFullData].
    pub const fn with_number(self, number: NearFullNumber) -> Self {
        Self {
            status: self.status,
            number,
        }
    }

    /// Attempts to convert a byte array into a [NearFullData].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        Ok(Self {
            status: buf.first().map(|&b| NearFullStatus::try_from(b)).ok_or(
                Error::InvalidNearFullDataLen((buf.len(), NEAR_FULL_DATA_LEN)),
            )??,
            number: buf.get(1..=2).map(NearFullNumber::from_bytes).ok_or(
                Error::InvalidNearFullDataLen((buf.len(), NEAR_FULL_DATA_LEN)),
            )?,
        })
    }

    /// Converts a [NearFullData] into a byte array.
    pub const fn into_bytes(self) -> [u8; NEAR_FULL_DATA_LEN] {
        let n = self.number.into_bytes();
        [self.status.into_u8(), n[0], n[1]]
    }

    /// Gets the byte length of the [NearFullData].
    pub const fn len() -> usize {
        NEAR_FULL_DATA_LEN
    }
}

impl Default for NearFullData {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for NearFullData {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, NEAR_FULL_DATA_LEN>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_bytes().into_iter()
    }
}

impl TryFrom<&[u8]> for NearFullData {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::from_bytes(val)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for NearFullData {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for NearFullData {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl fmt::Display for NearFullData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""status": {}, "#, self.status)?;
        write!(f, r#""number": {}"#, self.number)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_near_full_data() -> Result<()> {
        let exp = NearFullData::new();
        assert_eq!(exp.status(), NearFullStatus::Disabled);
        assert_eq!(exp.number(), NearFullNumber::from_u16(0));

        let raw = [1u8, 42u8, 1u8];
        let exp = NearFullData::from_bytes(raw.as_ref())?;
        let exp_stat = NearFullStatus::try_from(raw[0])?;
        let exp_num = NearFullNumber::from_bytes(&raw[1..]);

        assert_eq!(exp.status(), exp_stat);
        assert_eq!(exp.number(), exp_num);
        assert_eq!(exp.into_bytes(), raw);

        let stat = NearFullStatus::Disabled;
        let num = NearFullNumber::from_u16(42);
        assert_eq!(exp.with_status(stat).status(), stat);
        assert_eq!(exp.with_number(num).number(), num);

        Ok(())
    }

    #[test]
    fn test_near_full_data_invalid() {
        assert_eq!(
            NearFullData::from_bytes(&[]),
            Err(Error::InvalidNearFullDataLen((0, NEAR_FULL_DATA_LEN)))
        );
        assert_eq!(
            NearFullData::from_bytes(&[0]),
            Err(Error::InvalidNearFullDataLen((1, NEAR_FULL_DATA_LEN)))
        );
        assert_eq!(
            NearFullData::from_bytes(&[0, 0]),
            Err(Error::InvalidNearFullDataLen((2, NEAR_FULL_DATA_LEN)))
        );
    }
}
