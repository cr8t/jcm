use std::fmt;

use crate::Denomination;

/// Convenience alias for the library [`Result`](std::result::Result).
pub type Result<T> = std::result::Result<T, Error>;

/// Represents error variants for the library.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidBillAcceptorState(u8),
    InvalidDenomination((Denomination, u8)),
    InvalidFailureCode(u8),
    InvalidStatusCode(u8),
    InvalidMessageId(u8),
    InvalidConfId(u8),
    InvalidFuncId(u8),
    InvalidMessageType(u8),
    InvalidMessageLen((usize, usize)),
    InvalidMessageDataLen((usize, usize)),
    InvalidFunctionMode(u8),
    InvalidMajorMinorStatus(u16),
    InvalidDeviceStatus(u16),
    InvalidRequestCode(u16),
    InvalidEventCode(u16),
    InvalidMessageCode((u8, u16)),
    InvalidResponseCode(u8),
    InvalidResponseLen((usize, usize)),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBillAcceptorState(err) => write!(f, "invalid bill acceptor state: {err}"),
            Self::InvalidDenomination((denom, code)) => write!(
                f,
                r#"invalid denomination: {{"currency": {denom}, "code": {code}}}"#
            ),
            Self::InvalidFailureCode(err) => write!(f, "invalid failure code: {err}"),
            Self::InvalidStatusCode(err) => write!(f, "invalid status code: {err}"),
            Self::InvalidMessageId(err) => write!(f, "invalid message ID: {err}"),
            Self::InvalidConfId(err) => write!(f, "invalid conf ID: {err}"),
            Self::InvalidFuncId(err) => write!(f, "invalid func ID: {err}"),
            Self::InvalidMessageType(err) => write!(f, "invalid message type: {err}"),
            Self::InvalidMessageLen((have, exp)) => {
                write!(f, "invalid message length, have: {have}, expected: {exp}")
            }
            Self::InvalidMessageDataLen((have, exp)) => write!(
                f,
                "invalid message data length, have: {have}, expected: {exp}"
            ),
            Self::InvalidFunctionMode(err) => {
                write!(f, "invalid device status function mode: {err}")
            }
            Self::InvalidMajorMinorStatus(err) => {
                write!(f, "invalid device status major-minor status: {err}")
            }
            Self::InvalidDeviceStatus(err) => {
                write!(f, "invalid device status: {err}")
            }
            Self::InvalidRequestCode(err) => write!(f, "invalid request code: {err}"),
            Self::InvalidEventCode(err) => write!(f, "invalid event code: {err}"),
            Self::InvalidMessageCode((ty, code)) => write!(
                f,
                r#"invalid message code: {{"message_type": {ty}, "code": {code}}}"#
            ),
            Self::InvalidResponseCode(err) => write!(f, "invalid response code: {err}"),
            Self::InvalidResponseLen((have, exp)) => {
                write!(f, "invalid response length, have: {have}, expected: {exp}")
            }
        }
    }
}

impl std::error::Error for Error {}
