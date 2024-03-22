use std::fmt;

use crate::{Error, Result};

const TWENTY_MXN: u8 = 0x02;
const FIFTY_MXN: u8 = 0x05;
const HUNDRED_MXN: u8 = 0x0a;
const TWO_HUNDRED_MXN: u8 = 0x14;
const FIVE_HUNDRED_MXN: u8 = 0x32;
const THOUSAND_MXN: u8 = 0x64;
const RESERVED_MXN: u8 = 0xff;

const ONE_USD: u8 = 0x01;
const TWO_USD: u8 = 0x02;
const FIVE_USD: u8 = 0x05;
const TEN_USD: u8 = 0x0a;
const TWENTY_USD: u8 = 0x14;
const FIFTY_USD: u8 = 0x32;
const HUNDRED_USD: u8 = 0x64;
const RESERVED_USD: u8 = 0xff;

/// Represents device currency denomination countries.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Denomination {
    Mxn,
    Usd,
}

impl From<&Denomination> for &'static str {
    fn from(val: &Denomination) -> Self {
        match val {
            Denomination::Mxn => "MXN",
            Denomination::Usd => "USD",
        }
    }
}

impl From<Denomination> for &'static str {
    fn from(val: Denomination) -> Self {
        (&val).into()
    }
}

impl fmt::Display for Denomination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

/// Represents currency denominations: Mexican Peso.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DenominationMxn {
    Twenty = TWENTY_MXN,
    Fifty = FIFTY_MXN,
    Hundred = HUNDRED_MXN,
    TwoHundred = TWO_HUNDRED_MXN,
    FiveHundred = FIVE_HUNDRED_MXN,
    Thousand = THOUSAND_MXN,
    Reserved = RESERVED_MXN,
}

impl DenominationMxn {
    /// Creates a new [DenominationMxn].
    pub const fn new() -> Self {
        Self::Twenty
    }

    /// Infallible conversion from a [`u8`] into a [DenominationMxn].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            TWENTY_MXN => Self::Twenty,
            FIFTY_MXN => Self::Fifty,
            HUNDRED_MXN => Self::Hundred,
            TWO_HUNDRED_MXN => Self::TwoHundred,
            FIVE_HUNDRED_MXN => Self::FiveHundred,
            THOUSAND_MXN => Self::Thousand,
            _ => Self::Reserved,
        }
    }
}

impl Default for DenominationMxn {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&DenominationMxn> for &'static str {
    fn from(val: &DenominationMxn) -> Self {
        match val {
            DenominationMxn::Twenty => "20 MXN",
            DenominationMxn::Fifty => "50 MXN",
            DenominationMxn::Hundred => "100 MXN",
            DenominationMxn::TwoHundred => "200 MXN",
            DenominationMxn::FiveHundred => "500 MXN",
            DenominationMxn::Thousand => "1000 MXN",
            DenominationMxn::Reserved => "Reserved MXN",
        }
    }
}

impl From<DenominationMxn> for &'static str {
    fn from(val: DenominationMxn) -> Self {
        (&val).into()
    }
}

impl fmt::Display for DenominationMxn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl TryFrom<u8> for DenominationMxn {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidDenomination((Denomination::Mxn, val))),
            cur => Ok(cur),
        }
    }
}

/// Represents currency denominations: US Dollar.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DenominationUsd {
    One = ONE_USD,
    Two = TWO_USD,
    Five = FIVE_USD,
    Ten = TEN_USD,
    Twenty = TWENTY_USD,
    Fifty = FIFTY_USD,
    Hundred = HUNDRED_USD,
    Reserved = RESERVED_USD,
}

impl DenominationUsd {
    /// Creates a new [DenominationUsd].
    pub const fn new() -> Self {
        Self::One
    }

    /// Infallible conversion from a [`u8`] into a [DenominationUsd].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            ONE_USD => Self::One,
            TWO_USD => Self::Two,
            FIVE_USD => Self::Five,
            TEN_USD => Self::Ten,
            TWENTY_USD => Self::Twenty,
            FIFTY_USD => Self::Fifty,
            HUNDRED_USD => Self::Hundred,
            _ => Self::Reserved,
        }
    }
}

impl Default for DenominationUsd {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&DenominationUsd> for &'static str {
    fn from(val: &DenominationUsd) -> Self {
        match val {
            DenominationUsd::One => "1 USD",
            DenominationUsd::Two => "2 USD",
            DenominationUsd::Five => "5 USD",
            DenominationUsd::Ten => "10 USD",
            DenominationUsd::Twenty => "20 USD",
            DenominationUsd::Fifty => "50 USD",
            DenominationUsd::Hundred => "100 USD",
            DenominationUsd::Reserved => "Reserved USD",
        }
    }
}

impl From<DenominationUsd> for &'static str {
    fn from(val: DenominationUsd) -> Self {
        (&val).into()
    }
}

impl fmt::Display for DenominationUsd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl TryFrom<u8> for DenominationUsd {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidDenomination((Denomination::Usd, val))),
            cur => Ok(cur),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_denomination_mxn() {
        let raw_denom = [
            TWENTY_MXN,
            FIFTY_MXN,
            HUNDRED_MXN,
            TWO_HUNDRED_MXN,
            FIVE_HUNDRED_MXN,
            THOUSAND_MXN,
        ];
        let expected = [
            DenominationMxn::Twenty,
            DenominationMxn::Fifty,
            DenominationMxn::Hundred,
            DenominationMxn::TwoHundred,
            DenominationMxn::FiveHundred,
            DenominationMxn::Thousand,
        ];

        for (raw, exp) in raw_denom.into_iter().zip(expected.into_iter()) {
            assert_eq!(DenominationMxn::try_from(raw), Ok(exp));
            assert_eq!(DenominationMxn::from_u8(raw), exp);
        }


        for stat in (0..=255u8)
            .filter(|s| raw_denom.iter().find(|d| d == &s).is_none())
        {
            assert!(DenominationMxn::try_from(stat).is_err());
            assert_eq!(DenominationMxn::from_u8(stat), DenominationMxn::Reserved);
        }
    }

    #[test]
    fn test_denomination_usd() {
        let raw_denom = [
            ONE_USD,
            TWO_USD,
            FIVE_USD,
            TEN_USD,
            TWENTY_USD,
            FIFTY_USD,
            HUNDRED_USD,
        ];
        let expected = [
            DenominationUsd::One,
            DenominationUsd::Two,
            DenominationUsd::Five,
            DenominationUsd::Ten,
            DenominationUsd::Twenty,
            DenominationUsd::Fifty,
            DenominationUsd::Hundred,
        ];

        for (raw, exp) in raw_denom.into_iter().zip(expected.into_iter()) {
            assert_eq!(DenominationUsd::try_from(raw), Ok(exp));
        }

        for stat in (0..=255u8)
            .filter(|s| raw_denom.iter().find(|d| d == &s).is_none())
        {
            assert!(DenominationUsd::try_from(stat).is_err());
            assert_eq!(DenominationUsd::from_u8(stat), DenominationUsd::Reserved);
        }
    }
}
