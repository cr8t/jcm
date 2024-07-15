use std::{ffi, fmt};

use crate::{Error, Result};

/// Represents a `Cash Box Size`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CashBoxSize(u64);

impl CashBoxSize {
    /// Creates a new [CashBoxSize].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [CashBoxSize] from the provided parameter.
    pub const fn create(val: u64) -> Self {
        Self(val)
    }

    /// Gets the cash box size.
    pub const fn size(&self) -> u64 {
        self.0
    }

    /// Converts the [CashBoxSize] into a nul-terminated [`String`].
    pub fn into_string_with_nul(self) -> String {
        format!("{self}\0")
    }

    /// Gets the string length of the [CashBoxSize].
    pub const fn len(&self) -> usize {
        (self.0.ilog10() + 2) as usize
    }

    /// Gets whether the [CashBoxSize] is empty.
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl Default for CashBoxSize {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for CashBoxSize {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        <Vec<u8>>::from(self).into_iter()
    }
}

impl TryFrom<&[u8]> for CashBoxSize {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        ffi::CStr::from_bytes_until_nul(val)?.try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for CashBoxSize {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for CashBoxSize {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl TryFrom<&ffi::CStr> for CashBoxSize {
    type Error = Error;

    fn try_from(val: &ffi::CStr) -> Result<Self> {
        Ok(Self(
            val.to_str()?
                .parse::<u64>()
                .map_err(|_| Error::InvalidAsciiString)?,
        ))
    }
}

impl TryFrom<ffi::CString> for CashBoxSize {
    type Error = Error;

    fn try_from(val: ffi::CString) -> Result<Self> {
        val.as_c_str().try_into()
    }
}

impl TryFrom<String> for CashBoxSize {
    type Error = Error;

    fn try_from(val: String) -> Result<Self> {
        val.as_bytes().try_into()
    }
}

impl TryFrom<&str> for CashBoxSize {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        val.as_bytes().try_into()
    }
}

impl From<CashBoxSize> for String {
    fn from(val: CashBoxSize) -> Self {
        format!("{val}")
    }
}

impl fmt::Display for CashBoxSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CashBoxSize> for Vec<u8> {
    fn from(val: CashBoxSize) -> Self {
        val.into_string_with_nul().into_bytes()
    }
}

impl From<&CashBoxSize> for Vec<u8> {
    fn from(val: &CashBoxSize) -> Self {
        (*val).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cash_box_size() {
        let raw = b"500\0";
        let cstring = ffi::CString::from_vec_with_nul(raw.into()).unwrap();
        let string = String::from_utf8(raw.into()).unwrap();
        let exp = CashBoxSize::create(500);

        assert_eq!(CashBoxSize::try_from(raw), Ok(exp));
        assert_eq!(CashBoxSize::try_from(cstring.as_c_str()), Ok(exp));
        assert_eq!(CashBoxSize::try_from(cstring), Ok(exp));
        assert_eq!(CashBoxSize::try_from(string.as_str()), Ok(exp));
        assert_eq!(exp.into_string_with_nul().as_str(), string.as_str());
        assert_eq!(CashBoxSize::try_from(string), Ok(exp));

        [1, 100, 500, 1000, 2000, 10_000]
            .map(CashBoxSize::create)
            .into_iter()
            .for_each(|size| {
                // check the string length is the digits + trailing nul byte
                assert_eq!(size.into_string_with_nul().len(), size.len());
            });
    }

    #[test]
    fn test_cash_box_size_no_nul() {
        let invalid = b"500";
        let string = String::from_utf8(invalid.into()).unwrap();

        assert!(CashBoxSize::try_from(invalid).is_err());
        assert!(CashBoxSize::try_from(string.as_str()).is_err());
        assert!(CashBoxSize::try_from(string).is_err());
    }

    #[test]
    fn test_cash_box_size_not_a_number() {
        let invalid = b"50note";
        let string = String::from_utf8(invalid.into()).unwrap();
        let cstring = ffi::CString::new(string.as_bytes()).unwrap();

        assert!(CashBoxSize::try_from(invalid).is_err());
        assert!(CashBoxSize::try_from(string.as_str()).is_err());
        assert!(CashBoxSize::try_from(string).is_err());
        assert!(CashBoxSize::try_from(cstring.as_c_str()).is_err());
        assert!(CashBoxSize::try_from(cstring).is_err());
    }
}
