use std::fmt;

use crate::{Error, Result};

const COMPLETED_RESET_REQ: u8 = 1;
const RECEIVED_RESET_REQ: u8 = 2;
const RECEIVED_IDLE_REQ: u8 = 3;
const RECEIVED_INHIBIT_REQ: u8 = 4;
const INSERTED_BANKNOTE: u8 = 5;
const COMPLETED_VALIDATION: u8 = 6;
const RECEIVED_STACK_RETURN_HOLD: u8 = 7;
const COMPLETED_PAPER_TRANSPORT: u8 = 8;
const RECEIVED_VEND_VALID_EVENT_ACK: u8 = 9;
const BANKNOTE_STACKED: u8 = 10;
const COMPLETED_SCAN_COLLECT_PAPER_STACK: u8 = 11;
const RETURNED_COLLECT_EVENT_ACK: u8 = 12;
const TRANSPORT_REJECT_PAPER: u8 = 13;
const TRANSPORT_RETURN_PAPER: u8 = 14;
const REMOVED_REJECTED_PAPER: u8 = 15;
const REMOVED_RETURNED_PAPER: u8 = 16;
const COMPLETED_STACK_REQ: u8 = 17;
const REMOVED_PAPER: u8 = 18;
const REJECTED_10S: u8 = 19;
const RETURNED_10S: u8 = 20;
const ERROR: u8 = 21;
const ERROR_CONDITIONAL: u8 = 22;
const RECEIVED_RESET_REQ_ALT: u8 = 23;
const RESERVED: u8 = 0xff;

/// Represents status codes returned by JCM devices.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusCode {
    CompletedResetReq = COMPLETED_RESET_REQ,
    ReceivedResetReq = RECEIVED_RESET_REQ,
    ReceivedIdleReq = RECEIVED_IDLE_REQ,
    ReceivedInhibitReq = RECEIVED_INHIBIT_REQ,
    InsertedBanknote = INSERTED_BANKNOTE,
    CompletedValidation = COMPLETED_VALIDATION,
    ReceivedStackReturnHold = RECEIVED_STACK_RETURN_HOLD,
    CompletedPaperTransport = COMPLETED_PAPER_TRANSPORT,
    ReceivedVendValidEventAck = RECEIVED_VEND_VALID_EVENT_ACK,
    BanknoteStacked = BANKNOTE_STACKED,
    CompletedScanCollectPaperStack = COMPLETED_SCAN_COLLECT_PAPER_STACK,
    ReturnedCollectEventAck = RETURNED_COLLECT_EVENT_ACK,
    TransportRejectPaper = TRANSPORT_REJECT_PAPER,
    TransportReturnPaper = TRANSPORT_RETURN_PAPER,
    RemovedRejectedPaper = REMOVED_REJECTED_PAPER,
    RemovedReturnedPaper = REMOVED_RETURNED_PAPER,
    CompletedStackReq = COMPLETED_STACK_REQ,
    RemovedPaper = REMOVED_PAPER,
    Rejected10s = REJECTED_10S,
    Returned10s = RETURNED_10S,
    Error = ERROR,
    ErrorConditional = ERROR_CONDITIONAL,
    ReceivedResetReqAlt = RECEIVED_RESET_REQ_ALT,
    Reserved = RESERVED,
}

impl StatusCode {
    /// Creates a new [StatusCode].
    pub const fn new() -> Self {
        Self::CompletedResetReq
    }

    /// Infallible conversion from a [`u8`] into a [StatusCode].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            COMPLETED_RESET_REQ => Self::CompletedResetReq,
            RECEIVED_RESET_REQ => Self::ReceivedResetReq,
            RECEIVED_IDLE_REQ => Self::ReceivedIdleReq,
            RECEIVED_INHIBIT_REQ => Self::ReceivedInhibitReq,
            INSERTED_BANKNOTE => Self::InsertedBanknote,
            COMPLETED_VALIDATION => Self::CompletedValidation,
            RECEIVED_STACK_RETURN_HOLD => Self::ReceivedStackReturnHold,
            COMPLETED_PAPER_TRANSPORT => Self::CompletedPaperTransport,
            RECEIVED_VEND_VALID_EVENT_ACK => Self::ReceivedVendValidEventAck,
            BANKNOTE_STACKED => Self::BanknoteStacked,
            COMPLETED_SCAN_COLLECT_PAPER_STACK => Self::CompletedScanCollectPaperStack,
            RETURNED_COLLECT_EVENT_ACK => Self::ReturnedCollectEventAck,
            TRANSPORT_REJECT_PAPER => Self::TransportRejectPaper,
            TRANSPORT_RETURN_PAPER => Self::TransportReturnPaper,
            REMOVED_REJECTED_PAPER => Self::RemovedRejectedPaper,
            REMOVED_RETURNED_PAPER => Self::RemovedReturnedPaper,
            COMPLETED_STACK_REQ => Self::CompletedStackReq,
            REMOVED_PAPER => Self::RemovedPaper,
            REJECTED_10S => Self::Rejected10s,
            RETURNED_10S => Self::Returned10s,
            ERROR => Self::Error,
            ERROR_CONDITIONAL => Self::ErrorConditional,
            RECEIVED_RESET_REQ_ALT => Self::ReceivedResetReqAlt,
            _ => Self::Reserved,
        }
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u8> for StatusCode {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidStatusCode(val)),
            code => Ok(code),
        }
    }
}

impl From<&StatusCode> for &'static str {
    fn from(val: &StatusCode) -> Self {
        match val {
            StatusCode::CompletedResetReq => "completed handling reset request successfully",
            StatusCode::ReceivedResetReq => "received reset request",
            StatusCode::ReceivedIdleReq => "received idle request",
            StatusCode::ReceivedInhibitReq => "received inhibit request",
            StatusCode::InsertedBanknote => "inserted a banknote",
            StatusCode::CompletedValidation => "completed validation",
            StatusCode::ReceivedStackReturnHold => {
                "received stack or return rquest, or neither during a hold"
            }
            StatusCode::CompletedPaperTransport => "completed paper transport",
            StatusCode::ReceivedVendValidEventAck => "received vend valid event ACK",
            StatusCode::BanknoteStacked => "banknote stacked",
            StatusCode::CompletedScanCollectPaperStack => "completed scan collect paper stack",
            StatusCode::ReturnedCollectEventAck => "returned collect event ACK",
            StatusCode::TransportRejectPaper => "transport reject paper",
            StatusCode::TransportReturnPaper => "transport return paper",
            StatusCode::RemovedRejectedPaper => "removed rejected paper",
            StatusCode::RemovedReturnedPaper => "removed returned paper",
            StatusCode::CompletedStackReq => "completed stack req",
            StatusCode::RemovedPaper => "removed paper",
            StatusCode::Rejected10s => "stayed in rejected status for 10s",
            StatusCode::Returned10s => "stayed in returned status for 10s",
            StatusCode::Error => "error occurred",
            StatusCode::ErrorConditional => "error occurred, potentially in conditional vend mode",
            StatusCode::ReceivedResetReqAlt => "received reset request (depending on model)",
            StatusCode::Reserved => "reserved",
        }
    }
}

impl From<StatusCode> for &'static str {
    fn from(val: StatusCode) -> Self {
        (&val).into()
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code() {
        let raw_denom = [
            COMPLETED_RESET_REQ,
            RECEIVED_RESET_REQ,
            RECEIVED_IDLE_REQ,
            RECEIVED_INHIBIT_REQ,
            INSERTED_BANKNOTE,
            COMPLETED_VALIDATION,
            RECEIVED_STACK_RETURN_HOLD,
            COMPLETED_PAPER_TRANSPORT,
            RECEIVED_VEND_VALID_EVENT_ACK,
            BANKNOTE_STACKED,
            COMPLETED_SCAN_COLLECT_PAPER_STACK,
            RETURNED_COLLECT_EVENT_ACK,
            TRANSPORT_REJECT_PAPER,
            TRANSPORT_RETURN_PAPER,
            REMOVED_REJECTED_PAPER,
            REMOVED_RETURNED_PAPER,
            COMPLETED_STACK_REQ,
            REMOVED_PAPER,
            REJECTED_10S,
            RETURNED_10S,
            ERROR,
            ERROR_CONDITIONAL,
            RECEIVED_RESET_REQ_ALT,
        ];
        let expected = [
            StatusCode::CompletedResetReq,
            StatusCode::ReceivedResetReq,
            StatusCode::ReceivedIdleReq,
            StatusCode::ReceivedInhibitReq,
            StatusCode::InsertedBanknote,
            StatusCode::CompletedValidation,
            StatusCode::ReceivedStackReturnHold,
            StatusCode::CompletedPaperTransport,
            StatusCode::ReceivedVendValidEventAck,
            StatusCode::BanknoteStacked,
            StatusCode::CompletedScanCollectPaperStack,
            StatusCode::ReturnedCollectEventAck,
            StatusCode::TransportRejectPaper,
            StatusCode::TransportReturnPaper,
            StatusCode::RemovedRejectedPaper,
            StatusCode::RemovedReturnedPaper,
            StatusCode::CompletedStackReq,
            StatusCode::RemovedPaper,
            StatusCode::Rejected10s,
            StatusCode::Returned10s,
            StatusCode::Error,
            StatusCode::ErrorConditional,
            StatusCode::ReceivedResetReqAlt,
        ];

        for (raw, exp) in raw_denom.into_iter().zip(expected.into_iter()) {
            assert_eq!(StatusCode::try_from(raw), Ok(exp));
            assert_eq!(StatusCode::from_u8(raw), exp);
        }

        for stat in (0..=255u8).filter(|s| !raw_denom.iter().any(|d| d == s)) {
            assert!(StatusCode::try_from(stat).is_err());
            assert_eq!(StatusCode::from_u8(stat), StatusCode::Reserved);
        }
    }
}
