use std::fmt;

use crate::{Error, Result};

const ABNORMAL_INSERTION: u8 = 0x71;
const ABNORMAL_SENSOR: u8 = 0x72;
const RETURNED_REMAINING: u8 = 0x73;
const ABNORMAL_MAGNIFICATION: u8 = 0x74;
const TRANSPORTATION: u8 = 0x75;
const INHIBITED: u8 = 0x76;
const PHOTO_PATTERN1: u8 = 0x77;
const PHOTO_LEVEL: u8 = 0x78;
const INHIBIT_BEFORE_ESCROW: u8 = 0x79;
const RETURN: u8 = 0x7a;
const TRANSPORT_STACKER: u8 = 0x7b;
const TRANSPORT_FRAUD: u8 = 0x7c;
const NOTE_LENGTH: u8 = 0x7d;
const PHOTO_PATTERN2: u8 = 0x7e;
const TRUE_BILL_FEATURE: u8 = 0x7f;
const VALIDATE_BARCODE: u8 = 0x82;
const BARCODE_DIGITS: u8 = 0x83;
const BARCODE_START_BIT: u8 = 0x84;
const BARCODE_STOP_BIT: u8 = 0x85;
const DOUBLE_TICKET: u8 = 0x88;
const TICKET_WRONG_SIDE_UP: u8 = 0x8b;
const TICKET_LENGTH: u8 = 0x8d;
const RESERVED: u8 = 0xff;

/// Represents note rejection codes.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RejectCode {
    /// Abnormal note insertion.
    AbnormalInsertion = ABNORMAL_INSERTION,
    /// Abnormal magnetic/UV sensor detected.
    AbnormalSensor = ABNORMAL_SENSOR,
    /// Returned remaining amount from Stacker.
    ReturnedRemaining = RETURNED_REMAINING,
    /// Abnormal note magnification detected.
    AbnormalMagnification = ABNORMAL_MAGNIFICATION,
    /// Tranportation error.
    Transportation = TRANSPORTATION,
    /// Inhibited note detected.
    Inhibited = INHIBITED,
    /// Photo pattern (1) error.
    PhotoPattern1 = PHOTO_PATTERN1,
    /// Photo level error.
    PhotoLevel = PHOTO_LEVEL,
    /// Inhibited note detected before Escrow.
    InhibitBeforeEscrow = INHIBIT_BEFORE_ESCROW,
    /// Return requested by host.
    Return = RETURN,
    /// Transport error from the Stacker.
    TransportStacker = TRANSPORT_STACKER,
    /// Transport error, fraud detected.
    TransportFraud = TRANSPORT_FRAUD,
    /// Invalid note length.
    NoteLength = NOTE_LENGTH,
    /// Photo pattern (2) error.
    PhotoPattern2 = PHOTO_PATTERN2,
    /// True Bill Feature enabled.
    TrueBillFeature = TRUE_BILL_FEATURE,
    /// Validate barcode error.
    ValidateBarcode = VALIDATE_BARCODE,
    /// Invalid barcode digits.
    BarcodeDigits = BARCODE_DIGITS,
    /// Invalid/missing barcode start bit.
    BarcodeStartBit = BARCODE_START_BIT,
    /// Invalid/missing barcode stop bit.
    BarcodeStopBit = BARCODE_STOP_BIT,
    /// Double ticket detected.
    DoubleTicket = DOUBLE_TICKET,
    /// Ticket inserted wrong side facing up.
    TicketWrongSideUp = TICKET_WRONG_SIDE_UP,
    /// Invalid ticket length.
    TicketLength = TICKET_LENGTH,
    /// Reserved code.
    Reserved = RESERVED,
}

impl RejectCode {
    /// Creates a new [RejectCode].
    pub const fn new() -> Self {
        Self::AbnormalInsertion
    }

    /// Infallible function that converts a [`u8`] into a [RejectCode].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            ABNORMAL_INSERTION => Self::AbnormalInsertion,
            ABNORMAL_SENSOR => Self::AbnormalSensor,
            RETURNED_REMAINING => Self::ReturnedRemaining,
            ABNORMAL_MAGNIFICATION => Self::AbnormalMagnification,
            TRANSPORTATION => Self::Transportation,
            INHIBITED => Self::Inhibited,
            PHOTO_PATTERN1 => Self::PhotoPattern1,
            PHOTO_LEVEL => Self::PhotoLevel,
            INHIBIT_BEFORE_ESCROW => Self::InhibitBeforeEscrow,
            RETURN => Self::Return,
            TRANSPORT_STACKER => Self::TransportStacker,
            TRANSPORT_FRAUD => Self::TransportFraud,
            NOTE_LENGTH => Self::NoteLength,
            PHOTO_PATTERN2 => Self::PhotoPattern2,
            TRUE_BILL_FEATURE => Self::TrueBillFeature,
            VALIDATE_BARCODE => Self::ValidateBarcode,
            BARCODE_DIGITS => Self::BarcodeDigits,
            BARCODE_START_BIT => Self::BarcodeStartBit,
            BARCODE_STOP_BIT => Self::BarcodeStopBit,
            DOUBLE_TICKET => Self::DoubleTicket,
            TICKET_WRONG_SIDE_UP => Self::TicketWrongSideUp,
            TICKET_LENGTH => Self::TicketLength,
            _ => Self::Reserved,
        }
    }

    /// Converts a [RejectCode] into a [`u8`].
    pub const fn to_u8(&self) -> u8 {
        *self as u8
    }

    /// Converts a [RejectCode] into a [`u8`].
    pub const fn into_u8(self) -> u8 {
        self as u8
    }
}

impl Default for RejectCode {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u8> for RejectCode {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidRejectCode(val)),
            code => Ok(code),
        }
    }
}

impl From<RejectCode> for u8 {
    fn from(val: RejectCode) -> Self {
        val.into_u8()
    }
}

impl From<&RejectCode> for u8 {
    fn from(val: &RejectCode) -> Self {
        val.to_u8()
    }
}

impl From<RejectCode> for &'static str {
    fn from(val: RejectCode) -> Self {
        match val {
            RejectCode::AbnormalInsertion => "AbnormalInsertion",
            RejectCode::AbnormalSensor => "AbnormalSensor",
            RejectCode::ReturnedRemaining => "ReturnedRemaining",
            RejectCode::AbnormalMagnification => "AbnormalMagnification",
            RejectCode::Transportation => "Transportation",
            RejectCode::Inhibited => "Inhibited",
            RejectCode::PhotoPattern1 => "PhotoPattern1",
            RejectCode::PhotoLevel => "PhotoLevel",
            RejectCode::InhibitBeforeEscrow => "InhibitBeforeEscrow",
            RejectCode::Return => "Return",
            RejectCode::TransportStacker => "TransportStacker",
            RejectCode::TransportFraud => "TransportFraud",
            RejectCode::NoteLength => "NoteLength",
            RejectCode::PhotoPattern2 => "PhotoPattern2",
            RejectCode::TrueBillFeature => "TrueBillFeature",
            RejectCode::ValidateBarcode => "ValidateBarcode",
            RejectCode::BarcodeDigits => "BarcodeDigits",
            RejectCode::BarcodeStartBit => "BarcodeStartBit",
            RejectCode::BarcodeStopBit => "BarcodeStopBit",
            RejectCode::DoubleTicket => "DoubleTicket",
            RejectCode::TicketWrongSideUp => "TicketWrongSideUp",
            RejectCode::TicketLength => "TicketLength",
            RejectCode::Reserved => "Reserved",
        }
    }
}

impl From<&RejectCode> for &'static str {
    fn from(val: &RejectCode) -> Self {
        (*val).into()
    }
}

impl fmt::Display for RejectCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

/// Convenience struct to display detailed [RejectCode] message.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RejectCodeDetails(pub RejectCode);

impl From<RejectCodeDetails> for &'static str {
    fn from(val: RejectCodeDetails) -> Self {
        match val.0 {
            RejectCode::AbnormalInsertion => "abnormal note insertion",
            RejectCode::AbnormalSensor => "abnormal magnetic/UV sensor detected",
            RejectCode::ReturnedRemaining => "returned remaining amount from Stacker",
            RejectCode::AbnormalMagnification => "abnormal note magnification detected",
            RejectCode::Transportation => "transportation error",
            RejectCode::Inhibited => "inhibited note detected",
            RejectCode::PhotoPattern1 => "photo pattern (1) error",
            RejectCode::PhotoLevel => "photo level error",
            RejectCode::InhibitBeforeEscrow => "inhibited note detected before Escrow",
            RejectCode::Return => "return requested from the host",
            RejectCode::TransportStacker => "transport error from the Stacker",
            RejectCode::TransportFraud => "transport error, fraud detected",
            RejectCode::NoteLength => "invalid note length",
            RejectCode::PhotoPattern2 => "photo pattern (2) error",
            RejectCode::TrueBillFeature => "true Bill Feature enabled",
            RejectCode::ValidateBarcode => "validate barcode error",
            RejectCode::BarcodeDigits => "invalid barcode digits",
            RejectCode::BarcodeStartBit => "invalid/missing barcode start bit",
            RejectCode::BarcodeStopBit => "invalid/missing barcode stop bit",
            RejectCode::DoubleTicket => "double ticket detected",
            RejectCode::TicketWrongSideUp => "ticket inserted wrong side facing up",
            RejectCode::TicketLength => "invalid ticket length",
            RejectCode::Reserved => "reserved code",
        }
    }
}

impl From<&RejectCodeDetails> for &'static str {
    fn from(val: &RejectCodeDetails) -> Self {
        (*val).into()
    }
}

impl fmt::Display for RejectCodeDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reject_code() {
        let raw_vals = [
            ABNORMAL_INSERTION,
            ABNORMAL_SENSOR,
            RETURNED_REMAINING,
            ABNORMAL_MAGNIFICATION,
            TRANSPORTATION,
            INHIBITED,
            PHOTO_PATTERN1,
            PHOTO_LEVEL,
            INHIBIT_BEFORE_ESCROW,
            RETURN,
            TRANSPORT_STACKER,
            TRANSPORT_FRAUD,
            NOTE_LENGTH,
            PHOTO_PATTERN2,
            TRUE_BILL_FEATURE,
            VALIDATE_BARCODE,
            BARCODE_DIGITS,
            BARCODE_START_BIT,
            BARCODE_STOP_BIT,
            DOUBLE_TICKET,
            TICKET_WRONG_SIDE_UP,
            TICKET_LENGTH,
        ];

        let expected = [
            RejectCode::AbnormalInsertion,
            RejectCode::AbnormalSensor,
            RejectCode::ReturnedRemaining,
            RejectCode::AbnormalMagnification,
            RejectCode::Transportation,
            RejectCode::Inhibited,
            RejectCode::PhotoPattern1,
            RejectCode::PhotoLevel,
            RejectCode::InhibitBeforeEscrow,
            RejectCode::Return,
            RejectCode::TransportStacker,
            RejectCode::TransportFraud,
            RejectCode::NoteLength,
            RejectCode::PhotoPattern2,
            RejectCode::TrueBillFeature,
            RejectCode::ValidateBarcode,
            RejectCode::BarcodeDigits,
            RejectCode::BarcodeStartBit,
            RejectCode::BarcodeStopBit,
            RejectCode::DoubleTicket,
            RejectCode::TicketWrongSideUp,
            RejectCode::TicketLength,
        ];

        for (raw, exp) in raw_vals.into_iter().zip(expected.into_iter()) {
            assert_eq!(RejectCode::try_from(raw), Ok(exp));
            assert_eq!(RejectCode::from_u8(raw), exp);
        }

        for val in (0..=255u8).filter(|s| !raw_vals.iter().any(|d| d == s)) {
            assert!(RejectCode::try_from(val).is_err());
            assert_eq!(RejectCode::from_u8(val), RejectCode::Reserved);
        }
    }
}
