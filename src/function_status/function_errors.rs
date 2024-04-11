use std::fmt;

/// Represents whether the device unit is functional.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FunctionErrors {
    None = 0,
    ErrorOccurred = 1,
}

impl FunctionErrors {
    /// Creates a new [FunctionErrors].
    pub const fn new() -> Self {
        Self::None
    }

    /// Infallible function to convert a [`u8`] into a [FunctionErrors].
    pub const fn from_u8(val: u8) -> Self {
        match val & 0x1 {
            0 => Self::None,
            _ => Self::ErrorOccurred,
        }
    }

    /// Infallible function to convert a [FunctionErrors] into a [`u8`].
    pub const fn to_u8(&self) -> u8 {
        *self as u8
    }
}

impl Default for FunctionErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl From<FunctionErrors> for &'static str {
    fn from(val: FunctionErrors) -> Self {
        match val {
            FunctionErrors::None => "no errors",
            FunctionErrors::ErrorOccurred => "error occurred",
        }
    }
}

impl From<&FunctionErrors> for &'static str {
    fn from(val: &FunctionErrors) -> Self {
        (*val).into()
    }
}

impl fmt::Display for FunctionErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
