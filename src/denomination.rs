use std::{cmp, fmt, mem};

use crate::{Error, Result};

const DENOM_INT_SHIFT: u8 = 8;
const DENOM_EXP_MAX: u32 = 19;
const DENOM_BASE: u64 = 10;

/// Represents the currency denomination.
///
/// ## Format
///
/// Field  | Integer | Exponent
/// -------|---------|---------
/// Length | 1 byte  | 1 byte
///
/// Denominations representable by a [`u8`] will have a zero exponent, e.g. `100 = 100 * 10^0`.
///
/// Any denomination above [`u8::MAX`] will have a non-zero exponent, e.g. `500 = 50 * 10^1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Denomination(u16);

impl Denomination {
    /// Creates a new [Denomination].
    pub const fn new() -> Self {
        Self(0x0100)
    }

    /// Gets the `integer` field of the [Denomination].
    pub const fn integer(&self) -> u8 {
        (self.0 >> DENOM_INT_SHIFT) as u8
    }

    /// Gets the `exponent` field of the [Denomination].
    pub const fn exponent(&self) -> u8 {
        self.0 as u8
    }

    /// Gets the value of the [Denomination].
    ///
    /// # Example
    ///
    /// ```
    /// use jcm::Denomination;
    ///
    /// let denom = Denomination::new();
    /// assert_eq!(denom.integer(), 1);
    /// assert_eq!(denom.exponent(), 0);
    /// assert_eq!(denom.value(), 1);
    ///
    /// let denom = Denomination::from_value(500);
    /// assert_eq!(denom.integer(), 50);
    /// assert_eq!(denom.exponent(), 1);
    /// assert_eq!(denom.value(), 500);
    /// ```
    pub fn value(&self) -> u64 {
        let exp = cmp::min(self.exponent() as u32, DENOM_EXP_MAX);
        (self.integer() as u64).saturating_mul(DENOM_BASE.pow(exp))
    }

    /// Infallible function that converts a value into a [Denomination].
    ///
    /// # Example
    ///
    /// ```
    /// use jcm::Denomination;
    ///
    /// let denom = Denomination::from_value(2000);
    /// assert_eq!(denom.integer(), 200);
    /// assert_eq!(denom.exponent(), 1);
    /// assert_eq!(denom.value(), 2000);
    /// ```
    pub fn from_value(val: u64) -> Self {
        match val {
            v if v <= u8::MAX as u64 => Self((val << 8) as u16),
            v if v % 10 == 0 => {
                let exp = (val as f64).log10().floor() as u32;
                let (int, exp) = match val.saturating_div(10u64.pow(exp)) {
                    i if i == 1 || i == 2 => (i * 100, exp - 2),
                    i if i == 5 || i == 25 => (i * 10, exp - 1),
                    i => (i, exp),
                };

                Self(((int << 8) as u16) | exp as u16)
            }
            _ => Self(0),
        }
    }

    /// Gets the length of the [Denomination].
    pub const fn len() -> usize {
        mem::size_of::<u16>()
    }

    /// Gets whether the [Denomination] is empty.
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Gets whether the [Denomination] is valid.
    pub const fn is_valid(&self) -> bool {
        matches!(self.integer(), 1 | 2 | 5 | 10 | 20 | 50 | 100 | 200 | 250)
    }

    /// Converts the [Denomination] to a [`u16`].
    pub const fn to_u16(&self) -> u16 {
        self.0
    }

    /// Converts the [Denomination] to a [`u16`].
    pub const fn into_u16(self) -> u16 {
        self.0
    }

    /// Infallible function to convert a byte buffer into a [Denomination].
    pub fn from_bytes(val: &[u8]) -> Self {
        match val.len() {
            0 => Self(0),
            1 => Self((val[0] as u16) << DENOM_INT_SHIFT),
            _ => Self(((val[0] as u16) << DENOM_INT_SHIFT) | val[1] as u16),
        }
    }

    /// Gets whether the value is a valid [Denomination].
    pub fn valid_value(val: u64) -> bool {
        [1, 2, 5, 10, 20, 50, 100, 200, 250]
            .into_iter()
            .any(|v| val % v == 0 && (val <= 10 || val % 10 == 0))
    }

    /// Writes the [Denomination] to a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = Self::len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidDenominationLen((buf_len, len)))
        } else {
            buf.copy_from_slice(self.to_u16().to_be_bytes().as_ref());
            Ok(())
        }
    }
}

impl TryFrom<u64> for Denomination {
    type Error = Error;

    fn try_from(val: u64) -> Result<Self> {
        match Self::from_value(val) {
            d if d.is_valid() => Ok(d),
            d => Err(Error::InvalidDenomination((d.integer(), d.exponent()))),
        }
    }
}

impl TryFrom<u32> for Denomination {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        (val as u64).try_into()
    }
}

impl TryFrom<u16> for Denomination {
    type Error = Error;

    fn try_from(val: u16) -> Result<Self> {
        (val as u64).try_into()
    }
}

impl TryFrom<u8> for Denomination {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        (val as u64).try_into()
    }
}

impl TryFrom<&[u8]> for Denomination {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        match Self::from_bytes(val) {
            d if d.is_valid() => Ok(d),
            d => Err(Error::InvalidDenomination((d.integer(), d.exponent()))),
        }
    }
}

impl<const N: usize> TryFrom<[u8; N]> for Denomination {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for Denomination {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl Default for Denomination {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Denomination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""integer": {:#x}, "#, self.integer())?;
        write!(f, r#""exponent": {:#x}, "#, self.exponent())?;
        write!(f, r#""value": {:#x}"#, self.value())?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_denomination() {
        let raw_vals = [1, 2, 5, 10, 20, 50, 100, 250, 500, 1000, 10_000u64];
        let exp_ints = [1, 2, 5, 10, 20, 50, 100, 250, 50, 100, 100];
        let exp_exps = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2];
        let exp_denoms = [
            Denomination(0x0100),
            Denomination(0x0200),
            Denomination(0x0500),
            Denomination(0x0a00),
            Denomination(0x1400),
            Denomination(0x3200),
            Denomination(0x6400),
            Denomination(0xfa00),
            Denomination(0x3201),
            Denomination(0x6401),
            Denomination(0x6402),
        ];

        raw_vals.into_iter().enumerate().for_each(|(i, val)| {
            assert_eq!(Denomination::try_from(val), Ok(exp_denoms[i]));
            assert_eq!(
                Denomination::try_from([exp_ints[i], exp_exps[i]]),
                Ok(exp_denoms[i])
            );

            let denom = Denomination::from_value(val);

            assert_eq!(denom, exp_denoms[i]);
            assert_eq!(denom.integer(), exp_ints[i]);
            assert_eq!(denom.exponent(), exp_exps[i]);
            assert_eq!(denom.value(), val);

            assert!(denom.is_valid());
            assert!(!denom.is_empty());
        });
    }

    #[test]
    fn test_denomination_invalid() {
        let zero_denom = Denomination::from_value(0);

        assert!(zero_denom.is_empty());
        assert!(!zero_denom.is_valid());

        (0..=u16::MAX)
            .filter(|&v| !Denomination::valid_value(v as u64))
            .for_each(|val| {
                assert!(!Denomination::from_value(val as u64).is_valid());
                assert!(Denomination::try_from(val).is_err());
            });
    }
}
