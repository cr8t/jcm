use std::{fmt, mem};

use crate::{Error, FuncId, Result};

// Common feature [EventCode]s.
const POWER_UP: u16 = 0x0000;
const POWER_UP_ACCEPTOR: u16 = 0x0001;
const POWER_UP_STACKER: u16 = 0x0002;
const INHIBIT: u16 = 0x0100;
const PROGRAM_SIGNATURE: u16 = 0x0102;
const REJECTED: u16 = 0x0104;
const COLLECTED: u16 = 0x0108;
const CLEAR: u16 = 0x0200;
const OPERATION_ERROR: u16 = 0x0201;
const FAILURE: u16 = 0x0202;
const NOTE_STAY: u16 = 0x0301;

// Acceptor feature [EventCode]s.
const POWER_UP_ACCEPTOR_ACCEPTING: u16 = 0x1011;
const POWER_UP_STACKER_ACCEPTING: u16 = 0x1012;
const IDLE: u16 = 0x1101;
const ESCROW: u16 = 0x1102;
const VEND_VALID: u16 = 0x1103;
const ACCEPTOR_REJECTED: u16 = 0x1104;
const RETURNED: u16 = 0x1105;
const ACCEPTOR_COLLECTED: u16 = 0x1108;
const INSERT: u16 = 0x110a;
const CONDITIONAL_VEND: u16 = 0x110b;
const PAUSE: u16 = 0x110c;
const RESUME: u16 = 0x110d;
const ACCEPTOR_CLEAR: u16 = 0x1200;
const ACCEPTOR_OPERATION_ERROR: u16 = 0x1201;
const ACCEPTOR_FAILURE: u16 = 0x1202;
const ACCEPTOR_NOTE_STAY: u16 = 0x1301;
const FUNCTION_ABEYANCE: u16 = 0x1302;

const RESERVED: u16 = 0xffff;

/// Represents code variants for specific request messages.
#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventCode {
    /// Normal `Power Up` status.
    PowerUp = POWER_UP,
    /// Detected a returnable note on `Power Up`.
    PowerUpAcceptor = POWER_UP_ACCEPTOR,
    /// Detected a non-returnable note on `Power Up`.
    PowerUpStacker = POWER_UP_STACKER,
    /// Device is disabled.
    Inhibit = INHIBIT,
    /// Notified hash value of the device firmware.
    ProgramSignature = PROGRAM_SIGNATURE,
    /// Note removed from transport path during `Power Up`.
    Rejected = REJECTED,
    /// Note collection completed successfully.
    Collected = COLLECTED,
    /// Resolved controversial error.
    Clear = CLEAR,
    /// Unable to operate.
    OperationError = OPERATION_ERROR,
    /// Fatal error occurred.
    Failure = FAILURE,
    /// Returned note remains in `Insertion Slot` for a certain time.
    NoteStay = NOTE_STAY,
    /// Returned note left in device at `Power Down` is returnable at `Power Up`.
    PowerUpAcceptorAccepting = POWER_UP_ACCEPTOR_ACCEPTING,
    /// Returned note left in device at `Power Down` is non-returnable at `Power Up`.
    PowerUpStackerAccepting = POWER_UP_STACKER_ACCEPTING,
    /// Device in stand-by.
    Idle = IDLE,
    /// An inserted note passed validation.
    Escrow = ESCROW,
    /// Stacking note is allowed.
    VendValid = VEND_VALID,
    /// Returning note is completed before `Escrow Status` is notified.
    AcceptorRejected = ACCEPTOR_REJECTED,
    /// Returning note is completed after `Escrow Status` is notified.
    Returned = RETURNED,
    /// Stacking a note completed.
    AcceptorCollected = ACCEPTOR_COLLECTED,
    /// Note is inserted.
    Insert = INSERT,
    /// Conditional stacking is processed.
    ConditionalVend = CONDITIONAL_VEND,
    /// Device has stopped.
    Pause = PAUSE,
    /// Device resumes and is operational.
    Resume = RESUME,
    /// Device resolved controversial error.
    AcceptorClear = ACCEPTOR_CLEAR,
    /// Unable to operate.
    AcceptorOperationError = ACCEPTOR_OPERATION_ERROR,
    /// Fatal error occurred.
    AcceptorFailure = ACCEPTOR_FAILURE,
    /// Returned note remains in `Insertion Slot` for a certain time.
    AcceptorNoteStay = ACCEPTOR_NOTE_STAY,
    /// Enabled functions cannot operate.
    FunctionAbeyance = FUNCTION_ABEYANCE,
    /// Reserved request.
    Reserved = RESERVED,
}

impl EventCode {
    /// Creates a new [].
    pub const fn new() -> Self {
        Self::PowerUp
    }

    /// Infallible conversion from a [`u16`] into a [EventCode].
    pub const fn from_u16(val: u16) -> Self {
        match val {
            POWER_UP => Self::PowerUp,
            POWER_UP_ACCEPTOR => Self::PowerUpAcceptor,
            POWER_UP_STACKER => Self::PowerUpStacker,
            INHIBIT => Self::Inhibit,
            PROGRAM_SIGNATURE => Self::ProgramSignature,
            REJECTED => Self::Rejected,
            COLLECTED => Self::Collected,
            CLEAR => Self::Clear,
            OPERATION_ERROR => Self::OperationError,
            FAILURE => Self::Failure,
            NOTE_STAY => Self::NoteStay,
            POWER_UP_ACCEPTOR_ACCEPTING => Self::PowerUpAcceptorAccepting,
            POWER_UP_STACKER_ACCEPTING => Self::PowerUpStackerAccepting,
            IDLE => Self::Idle,
            ESCROW => Self::Escrow,
            VEND_VALID => Self::VendValid,
            ACCEPTOR_REJECTED => Self::AcceptorRejected,
            RETURNED => Self::Returned,
            ACCEPTOR_COLLECTED => Self::AcceptorCollected,
            INSERT => Self::Insert,
            CONDITIONAL_VEND => Self::ConditionalVend,
            PAUSE => Self::Pause,
            RESUME => Self::Resume,
            ACCEPTOR_CLEAR => Self::AcceptorClear,
            ACCEPTOR_OPERATION_ERROR => Self::AcceptorOperationError,
            ACCEPTOR_FAILURE => Self::AcceptorFailure,
            ACCEPTOR_NOTE_STAY => Self::AcceptorNoteStay,
            FUNCTION_ABEYANCE => Self::FunctionAbeyance,
            _ => Self::Reserved,
        }
    }

    /// Gets the length of the [EventCode].
    pub const fn len() -> usize {
        mem::size_of::<u16>()
    }

    /// Extracts the [FuncId] from the [EventCode].
    pub const fn func_id(&self) -> FuncId {
        FuncId::from_u16(*self as u16)
    }

    /// Gets whether the [EventCode] contains a reserved variant.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Reserved)
    }

    /// Gets whether the [EventCode] is a valid variant.
    pub const fn is_valid(&self) -> bool {
        !self.is_empty()
    }
}

impl Default for EventCode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&EventCode> for u16 {
    fn from(val: &EventCode) -> Self {
        (*val).into()
    }
}

impl From<EventCode> for u16 {
    fn from(val: EventCode) -> Self {
        val as Self
    }
}

impl TryFrom<u16> for EventCode {
    type Error = Error;

    fn try_from(val: u16) -> Result<Self> {
        match Self::from_u16(val) {
            Self::Reserved => Err(Error::InvalidEventCode(val)),
            v => Ok(v),
        }
    }
}

impl From<EventCode> for &'static str {
    fn from(val: EventCode) -> Self {
        match val {
            EventCode::PowerUp => "PowerUp",
            EventCode::PowerUpAcceptor => "PowerUpAcceptor",
            EventCode::PowerUpStacker => "PowerUpStacker",
            EventCode::Inhibit => "Inhibit",
            EventCode::ProgramSignature => "ProgramSignature",
            EventCode::Rejected => "Rejected",
            EventCode::Collected => "Collected",
            EventCode::Clear => "Clear",
            EventCode::OperationError => "OperationError",
            EventCode::Failure => "Failure",
            EventCode::NoteStay => "NoteStay",
            EventCode::PowerUpAcceptorAccepting => "PowerUpAcceptorAccepting",
            EventCode::PowerUpStackerAccepting => "PowerUpStackerAccepting",
            EventCode::Idle => "Idle",
            EventCode::Escrow => "Escrow",
            EventCode::VendValid => "VendValid",
            EventCode::AcceptorRejected => "AcceptorRejected",
            EventCode::Returned => "Returned",
            EventCode::AcceptorCollected => "AcceptorCollected",
            EventCode::Insert => "Insert",
            EventCode::ConditionalVend => "ConditionalVend",
            EventCode::Pause => "Pause",
            EventCode::Resume => "Resume",
            EventCode::AcceptorClear => "AcceptorClear",
            EventCode::AcceptorOperationError => "AcceptorOperationError",
            EventCode::AcceptorFailure => "AcceptorFailure",
            EventCode::AcceptorNoteStay => "AcceptorNoteStay",
            EventCode::FunctionAbeyance => "FunctionAbeyance",
            EventCode::Reserved => "Reserved",
        }
    }
}

impl From<&EventCode> for &'static str {
    fn from(val: &EventCode) -> Self {
        (*val).into()
    }
}

impl fmt::Display for EventCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

/// Convenience struct to display details for [EventCode].
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventCodeDetails(pub EventCode);

impl From<EventCodeDetails> for &'static str {
    fn from(val: EventCodeDetails) -> Self {
        match val.0 {
            EventCode::PowerUp => "normal `Power Up` status",
            EventCode::PowerUpAcceptor => "detected a returnable note on `Power Up`",
            EventCode::PowerUpStacker => "detected a non-returnable note on `Power Up`",
            EventCode::Inhibit => "device is disabled",
            EventCode::ProgramSignature => "notified hash value of the device firmware",
            EventCode::Rejected => "note removed from transport path during `Power Up`",
            EventCode::Collected => "note collection completed successfully",
            EventCode::Clear => "resolved controversial error",
            EventCode::OperationError => "unable to operate",
            EventCode::Failure => "fatal error occurred",
            EventCode::NoteStay => "returned note remains in `Insertion Slot` for a certain time",
            EventCode::PowerUpAcceptorAccepting => {
                "returned note left in device at `Power Down` is returnable at `Power Up`"
            }
            EventCode::PowerUpStackerAccepting => {
                "returned note left in device at `Power Down` is non-returnable at `Power Up`"
            }
            EventCode::Idle => "stand-by",
            EventCode::Escrow => "an inserted note passed validation",
            EventCode::VendValid => "stacking note is allowed",
            EventCode::AcceptorRejected => {
                "returning note is completed before `Escrow Status` is notified"
            }
            EventCode::Returned => "returning note is completed after `Escrow Status` is notified",
            EventCode::AcceptorCollected => "stacking a note completed",
            EventCode::Insert => "note is inserted",
            EventCode::ConditionalVend => "conditional stacking is processed",
            EventCode::Pause => "device has stopped",
            EventCode::Resume => "device resumes and is operational",
            EventCode::AcceptorClear => "device resolved controversial error",
            EventCode::AcceptorOperationError => "unable to operate",
            EventCode::AcceptorFailure => "fatal error occurred",
            EventCode::AcceptorNoteStay => {
                "returned note remains in `Insertion Slot` for a certain time"
            }
            EventCode::FunctionAbeyance => "enabled functions cannot operate",
            EventCode::Reserved => "reserved",
        }
    }
}

impl From<&EventCodeDetails> for &'static str {
    fn from(val: &EventCodeDetails) -> Self {
        (*val).into()
    }
}

impl fmt::Display for EventCodeDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_code() {
        let raw_vals = [
            POWER_UP,
            POWER_UP_ACCEPTOR,
            POWER_UP_STACKER,
            INHIBIT,
            PROGRAM_SIGNATURE,
            REJECTED,
            COLLECTED,
            CLEAR,
            OPERATION_ERROR,
            FAILURE,
            NOTE_STAY,
            POWER_UP_ACCEPTOR_ACCEPTING,
            POWER_UP_STACKER_ACCEPTING,
            IDLE,
            ESCROW,
            VEND_VALID,
            ACCEPTOR_REJECTED,
            RETURNED,
            ACCEPTOR_COLLECTED,
            INSERT,
            CONDITIONAL_VEND,
            PAUSE,
            RESUME,
            ACCEPTOR_CLEAR,
            ACCEPTOR_OPERATION_ERROR,
            ACCEPTOR_FAILURE,
            ACCEPTOR_NOTE_STAY,
            FUNCTION_ABEYANCE,
        ];

        let expected = [
            EventCode::PowerUp,
            EventCode::PowerUpAcceptor,
            EventCode::PowerUpStacker,
            EventCode::Inhibit,
            EventCode::ProgramSignature,
            EventCode::Rejected,
            EventCode::Collected,
            EventCode::Clear,
            EventCode::OperationError,
            EventCode::Failure,
            EventCode::NoteStay,
            EventCode::PowerUpAcceptorAccepting,
            EventCode::PowerUpStackerAccepting,
            EventCode::Idle,
            EventCode::Escrow,
            EventCode::VendValid,
            EventCode::AcceptorRejected,
            EventCode::Returned,
            EventCode::AcceptorCollected,
            EventCode::Insert,
            EventCode::ConditionalVend,
            EventCode::Pause,
            EventCode::Resume,
            EventCode::AcceptorClear,
            EventCode::AcceptorOperationError,
            EventCode::AcceptorFailure,
            EventCode::AcceptorNoteStay,
            EventCode::FunctionAbeyance,
        ];

        for (raw, exp) in raw_vals.into_iter().zip(expected.into_iter()) {
            assert_eq!(EventCode::try_from(raw), Ok(exp));
            assert_eq!(EventCode::from_u16(raw), exp);
            assert_eq!(FuncId::from_u16(raw), exp.func_id());
        }

        for val in (0..=0x1fffu16).filter(|s| !raw_vals.iter().any(|d| d == s)) {
            assert!(
                EventCode::try_from(val).is_err(),
                "expected failure on value: {val:#x}"
            );
            assert_eq!(EventCode::from_u16(val), EventCode::Reserved);
        }
    }
}
