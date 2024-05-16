/// Represents variants for inhibiting a denomination direction.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectionInhibit {
    Accept = 0,
    Inhibit = 1,
}

impl DirectionInhibit {
    /// Creates a new [DirectionInhibit].
    pub const fn new() -> Self {
        Self::Accept
    }

    /// Converts a [`bool`] into a [DirectionInhibit].
    pub const fn from_bool(val: bool) -> Self {
        match val {
            false => Self::Accept,
            true => Self::Inhibit,
        }
    }
}

impl From<u8> for DirectionInhibit {
    fn from(val: u8) -> Self {
        match val & 0x1 {
            0 => Self::Accept,
            1 => Self::Inhibit,
            // technically unreachable, but make the compiler happy without panic
            _ => Self::Accept,
        }
    }
}

impl Default for DirectionInhibit {
    fn default() -> Self {
        Self::new()
    }
}

impl From<bool> for DirectionInhibit {
    fn from(val: bool) -> Self {
        Self::from_bool(val)
    }
}

impl From<DirectionInhibit> for u8 {
    fn from(val: DirectionInhibit) -> Self {
        val as Self
    }
}

impl From<&DirectionInhibit> for u8 {
    fn from(val: &DirectionInhibit) -> Self {
        (*val).into()
    }
}

impl From<DirectionInhibit> for bool {
    fn from(val: DirectionInhibit) -> Self {
        matches!(val, DirectionInhibit::Inhibit)
    }
}

impl From<&DirectionInhibit> for bool {
    fn from(val: &DirectionInhibit) -> Self {
        (*val).into()
    }
}
