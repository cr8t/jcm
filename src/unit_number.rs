use std::{fmt, mem};

use crate::{Error, FuncId, Result};

const FUNC_ID_SHIFT: u8 = 4;
const FUNC_ID_MASK: u8 = 0xf0;
const UNIT_NUM_MASK: u8 = 0x0f;

/// Represents the unit number for a device function module.
///
/// # Format
///
/// Field   | [FuncId] | Unit Number
/// --------|----------|-----------------
/// Bitmask | 0xf0     | 0x0f (0x1 - 0xf, zero is invalid)
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnitNumber(u8);

impl UnitNumber {
    /// Creates a new [UnitNumber].
    pub const fn new() -> Self {
        Self(((FuncId::new() as u8) << FUNC_ID_SHIFT) | 1)
    }

    /// Gets the [FuncId] field of the [UnitNumber].
    pub const fn func_id(&self) -> FuncId {
        FuncId::from_u8(self.0 >> FUNC_ID_SHIFT)
    }

    /// Sets the [FuncId] field of the [UnitNumber].
    pub fn set_func_id(&mut self, func_id: FuncId) {
        self.0 &= !FUNC_ID_MASK;
        self.0 |= (func_id as u8) << FUNC_ID_SHIFT
    }

    /// Builder function that sets the [FuncId] field of the [UnitNumber].
    pub fn with_func_id(mut self, func_id: FuncId) -> Self {
        self.set_func_id(func_id);
        self
    }

    /// Gets the unit number field of the [UnitNumber].
    pub const fn unit_number(&self) -> u8 {
        self.0 & UNIT_NUM_MASK
    }

    /// Sets the unit number field of the [UnitNumber].
    pub fn set_unit_number(&mut self, num: u8) {
        let unit_num = UNIT_NUM_MASK & num;
        if Self::valid_unit_number(unit_num) {
            self.0 &= !UNIT_NUM_MASK;
            self.0 |= unit_num
        }
    }

    /// Builder function that sets the unit number field of the [UnitNumber].
    pub fn with_unit_number(mut self, num: u8) -> Self {
        self.set_unit_number(num);
        self
    }

    /// Infallible function to convert a [`u8`] into a [UnitNumber].
    pub const fn from_u8(val: u8) -> Self {
        Self(val)
    }

    /// Infallible function to convert a [UnitNumber] into a [`u8`].
    pub const fn to_u8(&self) -> u8 {
        self.0
    }

    /// Infallible function to convert a [UnitNumber] into a [`u8`].
    pub const fn into_u8(self) -> u8 {
        self.0
    }

    /// Gets the length of the [UnitNumber].
    pub const fn len() -> usize {
        mem::size_of::<u8>()
    }

    /// Gets whether the [UnitNumber] is empty.
    pub const fn is_empty(&self) -> bool {
        self.func_id().is_empty() || self.unit_number() == 0
    }

    /// Gets whether the [UnitNumber] is valid according to the specification format.
    pub const fn is_valid(&self) -> bool {
        !self.func_id().is_empty() && Self::valid_unit_number(self.0)
    }

    /// Gets whether the supplied number is a valid [UnitNumber].
    pub(crate) const fn valid_unit_number(num: u8) -> bool {
        (UNIT_NUM_MASK & num) != 0
    }
}

impl TryFrom<u8> for UnitNumber {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        let res = Self(val);

        if res.is_valid() {
            Ok(res)
        } else {
            Err(Error::InvalidUnitNumber(val))
        }
    }
}

impl From<UnitNumber> for u8 {
    fn from(val: UnitNumber) -> Self {
        val.to_u8()
    }
}

impl From<&UnitNumber> for u8 {
    fn from(val: &UnitNumber) -> Self {
        val.to_u8()
    }
}

impl fmt::Display for UnitNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""func_id": {}, "#, self.func_id())?;
        write!(f, r#""unit_number": {}"#, self.unit_number())?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FUNC_IDS: [u8; 4] = [
        FuncId::Common as u8,
        FuncId::Acceptor as u8,
        FuncId::Recycler as u8,
        FuncId::Escrow as u8,
    ];

    #[test]
    fn test_unit_number() {
        for id in FUNC_IDS.into_iter() {
            let func_id = id << FUNC_ID_SHIFT;
            for num in 0x1..=0xf {
                let good_num = UnitNumber::from_u8(func_id | num);

                assert!(!good_num.is_empty());
                assert!(good_num.is_valid());

                assert!(UnitNumber::try_from(func_id | num).is_ok());
            }
        }
    }

    #[test]
    fn test_unit_number_invalid() {
        for id in FUNC_IDS.into_iter() {
            let func_id = id << FUNC_ID_SHIFT;
            let bad_num = UnitNumber::from_u8(func_id);

            assert!(bad_num.is_empty());
            assert!(!bad_num.is_valid());

            assert!(UnitNumber::try_from(func_id).is_err(), "bad num: {bad_num}");
        }

        for num in 0x1..=0xf {
            let res_id = (FuncId::Reserved as u8) << FUNC_ID_SHIFT;
            let bad_id = UnitNumber::from_u8(res_id | num);

            assert!(bad_id.is_empty());
            assert!(!bad_id.is_valid());

            assert!(UnitNumber::try_from(res_id | num).is_err());
        }
    }
}
