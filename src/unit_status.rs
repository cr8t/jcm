use std::fmt;

use crate::{Error, FunctionStatus, Result, UnitNumber};

/// Represents the status of a JCM device unit, e.g. `Acceptor`, `Stacker`, `Recycler`, etc.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnitStatus {
    unit_number: UnitNumber,
    function_status: FunctionStatus,
}

impl UnitStatus {
    /// Creates a new [UnitStatus].
    pub const fn new() -> Self {
        Self {
            unit_number: UnitNumber::new(),
            function_status: FunctionStatus::new(),
        }
    }

    /// Gets the [UnitNumber] of the [UnitStatus].
    pub const fn unit_number(&self) -> UnitNumber {
        self.unit_number
    }

    /// Sets the [UnitNumber] of the [UnitStatus].
    pub fn set_unit_number(&mut self, val: UnitNumber) {
        self.unit_number = val;
    }

    /// Builder function that sets the [UnitNumber] of the [UnitStatus].
    pub fn with_unit_number(mut self, val: UnitNumber) -> Self {
        self.set_unit_number(val);
        self
    }

    /// Gets the [FunctionStatus] of the [UnitStatus].
    pub const fn function_status(&self) -> FunctionStatus {
        self.function_status
    }

    /// Sets the [FunctionStatus] of the [UnitStatus].
    pub fn set_function_status(&mut self, val: FunctionStatus) {
        self.function_status = val;
    }

    /// Builder function that sets the [FunctionStatus] of the [UnitStatus].
    pub fn with_function_status(mut self, val: FunctionStatus) -> Self {
        self.set_function_status(val);
        self
    }

    /// Gets the length of the [UnitStatus].
    pub const fn len() -> usize {
        UnitNumber::len() + FunctionStatus::len()
    }

    /// Gets whether the [UnitStatus] is empty.
    pub const fn is_empty(&self) -> bool {
        self.unit_number.is_empty() && self.function_status.is_empty()
    }

    /// Gets whether the [UnitStatus] is valid.
    pub const fn is_valid(&self) -> bool {
        self.unit_number.is_valid() && self.function_status.is_valid()
    }

    /// Infallible function that converts a byte array into a [UnitStatus].
    pub const fn from_bytes(val: &[u8]) -> Self {
        match val.len() {
            0 => Self::new(),
            1 => Self {
                unit_number: UnitNumber::from_u8(val[0]),
                function_status: FunctionStatus::new(),
            },
            _ => Self {
                unit_number: UnitNumber::from_u8(val[0]),
                function_status: FunctionStatus::from_u8(val[1]),
            },
        }
    }

    /// Infallible function that converts a [UnitStatus] into a byte array.
    pub const fn to_bytes(&self) -> [u8; 2] {
        [self.unit_number.to_u8(), self.function_status.to_u8()]
    }

    /// Infallible function that converts a [UnitStatus] into a byte array.
    pub const fn into_bytes(self) -> [u8; 2] {
        [self.unit_number.into_u8(), self.function_status.into_u8()]
    }
}

impl TryFrom<&[u8]> for UnitStatus {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<UnitStatus> {
        let len = Self::len();
        let val_len = val.len();

        if val_len < len {
            Err(Error::InvalidUnitStatusLen((val_len, len)))
        } else {
            Ok(Self {
                unit_number: UnitNumber::try_from(val[0])?,
                function_status: FunctionStatus::try_from(val[1])?,
            })
        }
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for UnitStatus {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<UnitStatus> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for UnitStatus {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<UnitStatus> {
        val.as_ref().try_into()
    }
}

impl fmt::Display for UnitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""unit_number": {}, "#, self.unit_number)?;
        write!(f, r#""function_status": {}"#, self.function_status)?;
        write!(f, "}}")
    }
}

/// Convenience container for a list of [UnitStatus] items.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitStatusList(pub Vec<UnitStatus>);

impl UnitStatusList {
    /// Creates a new [UnitStatusList].
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Gets a reference to the list of [UnitStatus] items.
    pub fn items(&self) -> &[UnitStatus] {
        self.0.as_ref()
    }

    /// Converts the [UnitStatusList] into its inner representation.
    pub fn into_inner(self) -> Vec<UnitStatus> {
        self.0
    }

    /// Converts the [UnitStatusList] into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.iter().flat_map(|c| c.to_bytes()).collect()
    }

    /// Converts the [UnitStatusList] into a byte vector.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0.into_iter().flat_map(|c| c.into_bytes()).collect()
    }

    /// Gets the byte length of the [UnitStatusList].
    pub fn len(&self) -> usize {
        self.0.len().saturating_mul(UnitStatus::len())
    }

    /// Gets whether the [UnitStatusList] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty() || self.0.iter().all(|u| u.is_empty())
    }
}

impl TryFrom<&[u8]> for UnitStatusList {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        let unit_len = UnitStatus::len();
        let val_len = val.len();

        if val_len % unit_len == 0 {
            Ok(Self(
                val.chunks_exact(UnitStatus::len())
                    .filter_map(|c| match UnitStatus::try_from(c) {
                        Ok(u) => Some(u),
                        Err(err) => {
                            log::warn!("invalid UnitStatus: {err}");
                            None
                        }
                    })
                    .collect(),
            ))
        } else {
            Err(Error::InvalidUnitStatusLen((val_len, unit_len)))
        }
    }
}

impl fmt::Display for UnitStatusList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, unit) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{unit}")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_status() -> Result<()> {
        const FUNC_STATS: [FunctionStatus; 9] = [
            FunctionStatus::Normal,
            FunctionStatus::NearFull,
            FunctionStatus::Full,
            FunctionStatus::BoxRemoved,
            FunctionStatus::JamAcceptor,
            FunctionStatus::JamStacker,
            FunctionStatus::Cheat,
            FunctionStatus::UnitRemoved,
            FunctionStatus::Failure,
        ];

        assert!(UnitStatus::try_from(&[]).is_err());
        assert!(UnitStatus::try_from(&[0u8]).is_err());
        assert!(UnitStatus::try_from(&[0u8, 0u8]).is_err());
        assert!(UnitStatus::try_from(&[1u8, 0u8]).is_ok());
        assert!(UnitStatus::try_from(&[1u8, 0u8, 0u8]).is_ok());

        let func_vals = FUNC_STATS.map(|f| f.to_u8());
        for num in 0x1..=0xf {
            for (i, func) in func_vals.into_iter().enumerate() {
                let raw = [num, func];
                let status = UnitStatus::try_from(&raw)?;
                let exp = UnitStatus::new()
                    .with_unit_number(UnitNumber::from_u8(num))
                    .with_function_status(FUNC_STATS[i]);

                assert_eq!(status.to_bytes(), raw);
                assert_eq!(status, exp);

                assert!(!status.is_empty());
                assert!(status.is_valid());
            }
        }

        Ok(())
    }
}
