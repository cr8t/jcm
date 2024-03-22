use std::fmt;

use crate::{Error, Result};

const INIT_STATE: u8 = 1;
const INHIBITED_STATE: u8 = 2;
const IDLE_STATE: u8 = 3;
const ESCROWED_STATE: u8 = 4;
const VEND_VALID_STATE: u8 = 5;
const RESERVED_STATE: u8 = 0xff;

/// Represents the state of the bill acceptor.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BillAcceptorState {
    Initializing = INIT_STATE,
    Inhibited = INHIBITED_STATE,
    Idle = IDLE_STATE,
    Escrowed = ESCROWED_STATE,
    VendValid = VEND_VALID_STATE,
    Reserved = RESERVED_STATE,
}

impl BillAcceptorState {
    /// Creates a new [BillAcceptorState].
    pub const fn new() -> Self {
        Self::Initializing
    }

    /// Infallible conversion from a [`u8`] into a [BillAcceptorState].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            INIT_STATE => Self::Initializing,
            INHIBITED_STATE => Self::Inhibited,
            IDLE_STATE => Self::Idle,
            ESCROWED_STATE => Self::Escrowed,
            VEND_VALID_STATE => Self::VendValid,
            _ => Self::Reserved,
        }
    }
}

impl Default for BillAcceptorState {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&BillAcceptorState> for &'static str {
    fn from(val: &BillAcceptorState) -> Self {
        match val {
            BillAcceptorState::Initializing => "initializing",
            BillAcceptorState::Inhibited => "inhibited",
            BillAcceptorState::Idle => "idle",
            BillAcceptorState::Escrowed => "escrowed",
            BillAcceptorState::VendValid => "vend valid",
            BillAcceptorState::Reserved => "reserved",
        }
    }
}

impl From<BillAcceptorState> for &'static str {
    fn from(val: BillAcceptorState) -> Self {
        (&val).into()
    }
}

impl fmt::Display for BillAcceptorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

impl TryFrom<u8> for BillAcceptorState {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidBillAcceptorState(val)),
            state => Ok(state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bill_acceptor_state() {
        let raw_denom = [
            INIT_STATE,
            INHIBITED_STATE,
            IDLE_STATE,
            ESCROWED_STATE,
            VEND_VALID_STATE,
        ];
        let expected = [
            BillAcceptorState::Initializing,
            BillAcceptorState::Inhibited,
            BillAcceptorState::Idle,
            BillAcceptorState::Escrowed,
            BillAcceptorState::VendValid,
        ];

        for (raw, exp) in raw_denom.into_iter().zip(expected.into_iter()) {
            assert_eq!(BillAcceptorState::try_from(raw), Ok(exp));
            assert_eq!(BillAcceptorState::from_u8(raw), exp);
        }

        for stat in (0..=255u8)
            .filter(|s| raw_denom.iter().find(|d| d == &s).is_none())
        {
            assert!(BillAcceptorState::try_from(stat).is_err());
            assert_eq!(BillAcceptorState::from_u8(stat), BillAcceptorState::Reserved);
        }
    }
}
