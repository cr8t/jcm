use std::fmt;

/// Represents whether the device unit is functional.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnitAvailability {
    Available = 0,
    NotFunctional = 1,
}

impl UnitAvailability {
    /// Creates a new [UnitAvailability].
    pub const fn new() -> Self {
        Self::Available
    }

    /// Infallible function to convert a [`u8`] into a [UnitAvailability].
    pub const fn from_u8(val: u8) -> Self {
        match val & 0x1 {
            0 => Self::Available,
            _ => Self::NotFunctional,
        }
    }

    /// Infallible function to convert a [UnitAvailability] into a [`u8`].
    pub const fn to_u8(&self) -> u8 {
        *self as u8
    }
}

impl Default for UnitAvailability {
    fn default() -> Self {
        Self::new()
    }
}

impl From<UnitAvailability> for &'static str {
    fn from(val: UnitAvailability) -> Self {
        match val {
            UnitAvailability::Available => "available",
            UnitAvailability::NotFunctional => "not functional",
        }
    }
}

impl From<&UnitAvailability> for &'static str {
    fn from(val: &UnitAvailability) -> Self {
        (*val).into()
    }
}

impl fmt::Display for UnitAvailability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
