use crate::{Error, Result};

/// Represents the length of [NearFullNumber].
pub const NEAR_FULL_NUMBER_LEN: usize = 2;

/// Represents the threshold number of banknotes to send a `Near Full` event message.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NearFullNumber(u16);

impl NearFullNumber {
    /// Creates a new [NearFullNumber].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Converts the [NearFullNumber] into a [`u16`].
    pub const fn from_u16(val: u16) -> Self {
        Self(val)
    }

    /// Converts the [NearFullNumber] into a [`u16`].
    pub const fn into_u16(self) -> u16 {
        self.0
    }

    /// Infallible function that converts a byte array into a [NearFullNumber].
    pub const fn from_bytes(val: &[u8]) -> Self {
        match val.len() {
            0 => Self(0),
            1 => Self(u16::from_le_bytes([val[0], 0])),
            _ => Self(u16::from_le_bytes([val[0], val[1]])),
        }
    }

    /// Converts the [NearFullNumber] into a byte array.
    pub const fn into_bytes(self) -> [u8; NEAR_FULL_NUMBER_LEN] {
        self.0.to_le_bytes()
    }
}

impl Default for NearFullNumber {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for NearFullNumber {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, NEAR_FULL_NUMBER_LEN>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_bytes().into_iter()
    }
}

impl TryFrom<&[u8]> for NearFullNumber {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        match val.len() {
            0 | 1 => Err(Error::InvalidNearFullNumberLen((
                val.len(),
                NEAR_FULL_NUMBER_LEN,
            ))),
            _ => Ok(Self::from_bytes(val)),
        }
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for NearFullNumber {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for NearFullNumber {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_near_full_number() {
        assert_eq!(NearFullNumber::from_u16(u16::MIN).into_u16(), u16::MIN);
        assert_eq!(NearFullNumber::from_u16(u16::MAX).into_u16(), u16::MAX);

        assert_eq!(NearFullNumber::from_bytes(&[]).into_u16(), 0);
        assert_eq!(NearFullNumber::from_bytes(&[42]).into_u16(), 42);
        assert_eq!(
            NearFullNumber::from_bytes(&[42, 1]).into_u16(),
            (1 << 8) | 42
        );

        assert_eq!(NearFullNumber::from_bytes(&[]).into_bytes(), [0, 0]);
        assert_eq!(NearFullNumber::from_bytes(&[42]).into_bytes(), [42, 0]);
        assert_eq!(NearFullNumber::from_bytes(&[42, 1]).into_bytes(), [42, 1]);

        assert_eq!(
            NearFullNumber::try_from(&[42u8, 1u8]),
            Ok(NearFullNumber::from_u16(1 << 8 | 42))
        );

        // Only considers the first `NEAR_FULL_NUMBER_LEN` amount of bytes
        assert_eq!(
            NearFullNumber::from_bytes(&[42, 1, 2]).into_bytes(),
            [42, 1]
        );
        assert_eq!(
            NearFullNumber::try_from(&[42, 1, 2]),
            Ok(NearFullNumber::from_u16(1 << 8 | 42))
        );
    }

    #[test]
    fn test_near_full_number_invalid() {
        assert_eq!(
            NearFullNumber::try_from(&[]),
            Err(Error::InvalidNearFullNumberLen((0, NEAR_FULL_NUMBER_LEN)))
        );
        assert_eq!(
            NearFullNumber::try_from(&[42]),
            Err(Error::InvalidNearFullNumberLen((1, NEAR_FULL_NUMBER_LEN)))
        );
    }
}
