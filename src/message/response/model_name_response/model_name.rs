use std::fmt;

use crate::{Error, Result};

/// Represents the device model name.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelName(String);

impl ModelName {
    /// Creates a new [ModelName].
    pub const fn new() -> Self {
        Self(String::new())
    }

    /// Gets a reference to the [ModelName] string.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Converts a string into a [ModelName].
    pub fn from_string(val: &str) -> Self {
        Self(val.split('\0').next().unwrap_or_default().into())
    }

    /// Attempts to convert a byte buffer into a [ModelName].
    pub fn from_bytes_until_nul(buf: &[u8]) -> Result<Self> {
        let pos = buf.iter().position(|c| c == &b'\0').unwrap_or(buf.len());

        Ok(Self(String::from_utf8(buf[..pos].into())?))
    }

    /// Gets the byte length of the [ModelName].
    pub fn len(&self) -> usize {
        self.0.as_bytes().len()
    }

    /// Gets whether the [ModelName] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets a iterator over the [ModelName] bytes.
    pub fn iter(&self) -> impl Iterator<Item = &u8> {
        self.0.as_bytes().iter()
    }

    /// Gets a iterator over the [ModelName] bytes.
    pub fn iter_with_nul(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.as_bytes().iter().copied().chain([b'\0'])
    }

    /// Converts the [ModelName] into a string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Converts the [ModelName] into a byte vector.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0.into_bytes()
    }

    /// Converts the [ModelName] into a byte vector with a nul terminator.
    pub fn into_bytes_with_nul(self) -> Vec<u8> {
        self.into_iter().chain([b'\0']).collect()
    }
}

impl Default for ModelName {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for ModelName {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> std::vec::IntoIter<u8> {
        self.0.into_bytes().into_iter()
    }
}

impl std::str::FromStr for ModelName {
    type Err = Error;

    fn from_str(val: &str) -> Result<Self> {
        Ok(Self::from_string(val))
    }
}

impl TryFrom<&[u8]> for ModelName {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::from_bytes_until_nul(val)
    }
}

impl fmt::Display for ModelName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, self.0)
    }
}
