use std::{fmt, mem};

use crate::{Error, Result};

const ACCEPTOR: u8 = 0x10;
const ACCEPTOR_RECYCLER: u8 = 0x11;
const ACCEPTOR_ESCROW: u8 = 0x12;
const ACCEPTOR_RECYCLER_ESCROW: u8 = 0x18;
const RESERVED: u8 = 0xff;

/// Represents the JCM device configuration ID.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfId {
    /// Only the primary `acceptor` feature.
    Acceptor = ACCEPTOR,
    /// Primary acceptor feature, and secondary recycler feature.
    AcceptorRecycler = ACCEPTOR_RECYCLER,
    /// Primary acceptor feature, and secondary escrow feature.
    AcceptorEscrow = ACCEPTOR_ESCROW,
    /// Primary acceptor feature, and secondary recycler + escrow features.
    AcceptorRecyclerEscrow = ACCEPTOR_RECYCLER_ESCROW,
    /// Reserved configuration.
    Reserved = RESERVED,
}

impl ConfId {
    /// Creates a new [ConfId].
    pub const fn new() -> Self {
        Self::Acceptor
    }

    /// Infallible conversion from a [`u8`] into a [ConfId].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            ACCEPTOR => Self::Acceptor,
            ACCEPTOR_RECYCLER => Self::AcceptorRecycler,
            ACCEPTOR_ESCROW => Self::AcceptorEscrow,
            ACCEPTOR_RECYCLER_ESCROW => Self::AcceptorRecyclerEscrow,
            _ => Self::Reserved,
        }
    }

    /// Gets the length of the [ConfId].
    pub const fn len() -> usize {
        mem::size_of::<u8>()
    }

    /// Gets whether the [ConfId] contains a reserved variant.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Reserved)
    }
}

impl Default for ConfId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ConfId> for u8 {
    fn from(val: ConfId) -> Self {
        val as Self
    }
}

impl From<&ConfId> for u8 {
    fn from(val: &ConfId) -> Self {
        (*val).into()
    }
}

impl TryFrom<u8> for ConfId {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidConfId(val)),
            v => Ok(v),
        }
    }
}

impl From<&ConfId> for &'static str {
    fn from(val: &ConfId) -> Self {
        match val {
            ConfId::Acceptor => "acceptor",
            ConfId::AcceptorRecycler => "acceptor, recycler",
            ConfId::AcceptorEscrow => "acceptor, escrow",
            ConfId::AcceptorRecyclerEscrow => "acceptor, recyler, escrow",
            ConfId::Reserved => "reserved",
        }
    }
}

impl From<ConfId> for &'static str {
    fn from(val: ConfId) -> Self {
        (&val).into()
    }
}

impl fmt::Display for ConfId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_id() {
        let raw_denom = [
            ACCEPTOR,
            ACCEPTOR_RECYCLER,
            ACCEPTOR_ESCROW,
            ACCEPTOR_RECYCLER_ESCROW,
        ];
        let expected = [
            ConfId::Acceptor,
            ConfId::AcceptorRecycler,
            ConfId::AcceptorEscrow,
            ConfId::AcceptorRecyclerEscrow,
        ];

        for (raw, exp) in raw_denom.into_iter().zip(expected.into_iter()) {
            assert_eq!(ConfId::try_from(raw), Ok(exp));
            assert_eq!(ConfId::from_u8(raw), exp);
        }

        for stat in (0..=255u8).filter(|s| !raw_denom.iter().any(|d| d == s)) {
            assert!(ConfId::try_from(stat).is_err());
            assert_eq!(ConfId::from_u8(stat), ConfId::Reserved);
        }
    }
}
