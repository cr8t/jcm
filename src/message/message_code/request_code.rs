use std::{fmt, mem};

use crate::{Error, FuncId, Result};

// Common request codes
const UID: u16 = 0x0001;
const PROGRAM_SIGNATURE: u16 = 0x0002;
const VERSION: u16 = 0x0003;
const SERIAL_NUMBER: u16 = 0x0004;
const MODEL_NAME: u16 = 0x0005;
const STATUS: u16 = 0x0010;
const RESET: u16 = 0x0011;
const INHIBIT: u16 = 0x0012;
const COLLECT: u16 = 0x0017;
const KEY: u16 = 0x0019;
const EVENT_RESEND_INTERVAL: u16 = 0x002c;

// Acceptor request codes
const IDLE: u16 = 0x1013;
const STACK: u16 = 0x1014;
const REJECT: u16 = 0x1015;
const HOLD: u16 = 0x1016;
const ACCEPTOR_COLLECT: u16 = 0x1017;
const DENOMINATION_DISABLE: u16 = 0x1021;
const DIRECTION_DISABLE: u16 = 0x1022;
const CURRENCY_ASSIGN: u16 = 0x1023;
const CASH_BOX_SIZE: u16 = 0x1024;
const NEAR_FULL: u16 = 0x1025;
const BAR_CODE: u16 = 0x1026;
const INSERT: u16 = 0x1028;
const CONDITIONAL_VEND: u16 = 0x1029;
const PAUSE: u16 = 0x102a;
const NOTE_DATA_INFO: u16 = 0x102f;

// Recycler request codes
const RECYCLER_COLLECT: u16 = 0x2017;

const RESERVED: u16 = 0xffff;

/// Represents code variants for specific request messages.
#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestCode {
    /// Request to get/set UID information.
    Uid = UID,
    /// Request hash value of the firmware, or the supported hash algorithm.
    ProgramSignature = PROGRAM_SIGNATURE,
    /// Request the version of the device firmware.
    Version = VERSION,
    /// Request the device serial number.
    SerialNumber = SERIAL_NUMBER,
    /// Request the device product name.
    ModelName = MODEL_NAME,
    /// Request the device condition.
    Status = STATUS,
    /// Request to reset the device.
    Reset = RESET,
    /// Request to disable the device.
    Inhibit = INHIBIT,
    /// Request at `Power Up` to collect any note left in the device transport path.
    Collect = COLLECT,
    /// Request to send the key input accept/reject information (status or setting).
    Key = KEY,
    /// Request to send or set/change the `Event Re-sending Interval` setting.
    EventResendInterval = EVENT_RESEND_INTERVAL,
    /// Request to accept the operation request.
    Idle = IDLE,
    /// Request to accept the note.
    Stack = STACK,
    /// Request to reject the note.
    Reject = REJECT,
    /// Request to hold the note in escrow.
    Hold = HOLD,
    /// Request at `Power Up` to collect any note left in the device transport path.
    AcceptorCollect = ACCEPTOR_COLLECT,
    /// Request to send or set information of notes to disable by denomination.
    DenominationDisable = DENOMINATION_DISABLE,
    /// Request to send or set information of notes to disable by direction.
    DirectionDisable = DIRECTION_DISABLE,
    /// Request to send the acceptable denominnation information.
    CurrencyAssign = CURRENCY_ASSIGN,
    /// Request send the `Cash Box` capacity information.
    CashBoxSize = CASH_BOX_SIZE,
    /// Request to send or set the `Near Full` settings information.
    NearFull = NEAR_FULL,
    /// Request to send or set the `Bar Code` features information.
    BarCode = BAR_CODE,
    /// Request to send or set the `Insert Notification Function` settings information.
    Insert = INSERT,
    /// Request to send or set the `Conditional Vend Function` settings information.
    ConditionalVend = CONDITIONAL_VEND,
    /// Request to send or set the `Pause` duration, and `Status and Event Message` enabled/disabled settings information.
    Pause = PAUSE,
    /// Request to send information of an inserted note.
    NoteDataInfo = NOTE_DATA_INFO,
    /// Request for retrieving.
    RecyclerCollect = RECYCLER_COLLECT,
    /// Reserved request.
    Reserved = RESERVED,
}

impl RequestCode {
    /// Creates a new [RequestCode].
    pub const fn new() -> Self {
        Self::Uid
    }

    /// Infallible conversion from a [`u16`] into a [RequestCode].
    pub const fn from_u16(val: u16) -> Self {
        match val {
            UID => Self::Uid,
            PROGRAM_SIGNATURE => Self::ProgramSignature,
            VERSION => Self::Version,
            SERIAL_NUMBER => Self::SerialNumber,
            MODEL_NAME => Self::ModelName,
            STATUS => Self::Status,
            RESET => Self::Reset,
            INHIBIT => Self::Inhibit,
            COLLECT => Self::Collect,
            KEY => Self::Key,
            EVENT_RESEND_INTERVAL => Self::EventResendInterval,
            IDLE => Self::Idle,
            STACK => Self::Stack,
            REJECT => Self::Reject,
            HOLD => Self::Hold,
            ACCEPTOR_COLLECT => Self::AcceptorCollect,
            DENOMINATION_DISABLE => Self::DenominationDisable,
            DIRECTION_DISABLE => Self::DirectionDisable,
            CURRENCY_ASSIGN => Self::CurrencyAssign,
            CASH_BOX_SIZE => Self::CashBoxSize,
            NEAR_FULL => Self::NearFull,
            BAR_CODE => Self::BarCode,
            INSERT => Self::Insert,
            CONDITIONAL_VEND => Self::ConditionalVend,
            PAUSE => Self::Pause,
            NOTE_DATA_INFO => Self::NoteDataInfo,
            RECYCLER_COLLECT => Self::RecyclerCollect,
            _ => Self::Reserved,
        }
    }

    /// Extracts the [FuncId] from the [RequestCode].
    pub const fn func_id(&self) -> FuncId {
        FuncId::from_u16(*self as u16)
    }

    /// Gets the length of the [RequestCode].
    pub const fn len() -> usize {
        mem::size_of::<u16>()
    }

    /// Gets whether the [RequestCode] contains a reserved variant.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Reserved)
    }

    /// Gets whether the [RequestCode] is a valid variant.
    pub const fn is_valid(&self) -> bool {
        !self.is_empty()
    }
}

impl Default for RequestCode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&RequestCode> for u16 {
    fn from(val: &RequestCode) -> Self {
        (*val).into()
    }
}

impl From<RequestCode> for u16 {
    fn from(val: RequestCode) -> Self {
        val as Self
    }
}

impl TryFrom<u16> for RequestCode {
    type Error = Error;

    fn try_from(val: u16) -> Result<Self> {
        match Self::from_u16(val) {
            Self::Reserved => Err(Error::InvalidRequestCode(val)),
            v => Ok(v),
        }
    }
}

impl From<&RequestCode> for &'static str {
    fn from(val: &RequestCode) -> Self {
        match val {
            RequestCode::Uid => "request to get/set UID information",
            RequestCode::ProgramSignature => "request hash value of the firmware, or the supported hash algorithm",
            RequestCode::Version => "request the version of the device firmware",
            RequestCode::SerialNumber => "request the device serial number",
            RequestCode::ModelName => "request the device product name",
            RequestCode::Status => "request the device condition",
            RequestCode::Reset => "request to reset the device",
            RequestCode::Inhibit => "request to disable the device",
            RequestCode::Collect => "request at `Power Up` to collect any note left in the device transport path",
            RequestCode::Key => "request to send the key input accept/reject information (status or setting)",
            RequestCode::EventResendInterval => "request to send or set/change the `Event Re-sending Interval` setting",
            RequestCode::Idle => "request to accept the operation request",
            RequestCode::Stack => "request to accept the note",
            RequestCode::Reject => "request to reject the note",
            RequestCode::Hold => "request to hold the note in escrow",
            RequestCode::AcceptorCollect => "request at `Power Up` to collect any note left in the device transport path",
            RequestCode::DenominationDisable => "request to send or set information of notes to disable by denomination",
            RequestCode::DirectionDisable => "request to send or set information of notes to disable by direction",
            RequestCode::CurrencyAssign => "request to send the acceptable denominnation information",
            RequestCode::CashBoxSize => "request send the `Cash Box` capacity information",
            RequestCode::NearFull => "request to send or set the `Near Full` settings information",
            RequestCode::BarCode => "request to send or set the `Bar Code` features information",
            RequestCode::Insert => "request to send or set the `Insert Notification Function` settings information",
            RequestCode::ConditionalVend => "request to send or set the `Conditional Vend Function` settings information",
            RequestCode::Pause => "request to send or set the `Pause` duration, and `Status and Event Message` enabled/disabled settings information",
            RequestCode::NoteDataInfo => "request to send information of an inserted note",
            RequestCode::RecyclerCollect => "request for retrieving",
            RequestCode::Reserved => "reserved",
        }
    }
}

impl From<RequestCode> for &'static str {
    fn from(val: RequestCode) -> Self {
        (&val).into()
    }
}

impl fmt::Display for RequestCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_code() {
        let raw_vals = [
            UID,
            PROGRAM_SIGNATURE,
            VERSION,
            SERIAL_NUMBER,
            MODEL_NAME,
            STATUS,
            RESET,
            INHIBIT,
            COLLECT,
            KEY,
            EVENT_RESEND_INTERVAL,
            IDLE,
            STACK,
            REJECT,
            HOLD,
            ACCEPTOR_COLLECT,
            DENOMINATION_DISABLE,
            DIRECTION_DISABLE,
            CURRENCY_ASSIGN,
            CASH_BOX_SIZE,
            NEAR_FULL,
            BAR_CODE,
            INSERT,
            CONDITIONAL_VEND,
            PAUSE,
            NOTE_DATA_INFO,
            RECYCLER_COLLECT,
        ];
        let expected = [
            RequestCode::Uid,
            RequestCode::ProgramSignature,
            RequestCode::Version,
            RequestCode::SerialNumber,
            RequestCode::ModelName,
            RequestCode::Status,
            RequestCode::Reset,
            RequestCode::Inhibit,
            RequestCode::Collect,
            RequestCode::Key,
            RequestCode::EventResendInterval,
            RequestCode::Idle,
            RequestCode::Stack,
            RequestCode::Reject,
            RequestCode::Hold,
            RequestCode::AcceptorCollect,
            RequestCode::DenominationDisable,
            RequestCode::DirectionDisable,
            RequestCode::CurrencyAssign,
            RequestCode::CashBoxSize,
            RequestCode::NearFull,
            RequestCode::BarCode,
            RequestCode::Insert,
            RequestCode::ConditionalVend,
            RequestCode::Pause,
            RequestCode::NoteDataInfo,
            RequestCode::RecyclerCollect,
        ];

        for (raw, exp) in raw_vals.into_iter().zip(expected.into_iter()) {
            assert_eq!(RequestCode::try_from(raw), Ok(exp));
            assert_eq!(RequestCode::from_u16(raw), exp);
            assert_eq!(FuncId::from_u16(raw), exp.func_id());
        }

        for val in (0..=0x1fffu16).filter(|s| !raw_vals.iter().any(|d| d == s)) {
            assert!(RequestCode::try_from(val).is_err());
            assert_eq!(RequestCode::from_u16(val), RequestCode::Reserved);
        }
    }
}
