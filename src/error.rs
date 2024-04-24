use std::fmt;

/// Convenience alias for the library [`Result`](std::result::Result).
pub type Result<T> = std::result::Result<T, Error>;

/// Represents error variants for the library.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidBillAcceptorState(u8),
    InvalidDenomination((u8, u8)),
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
    InvalidDeviceStatusLen((usize, usize)),
    InvalidDeviceStatus(u16),
    InvalidRequestCode(u16),
    InvalidEventCode(u16),
    InvalidMessageCode((u8, u16)),
    InvalidResponseCode(u8),
    InvalidResponseLen((usize, usize)),
    InvalidUnitNumber(u8),
    InvalidFunctionStatus(u8),
    InvalidUnitStatusLen((usize, usize)),
    InvalidUnitStatusListLen((usize, usize)),
    InvalidStackRequestDataLen((usize, usize)),
    InvalidEventLen((usize, usize)),
    InvalidRequestLen((usize, usize)),
    InvalidStackStatusChange(u8),
    InvalidRejectCode(u8),
    InvalidRequestType(u8),
    InvalidEventType(u8),
    #[cfg(feature = "usb")]
    Usb(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBillAcceptorState(err) => write!(f, "invalid bill acceptor state: {err}"),
            Self::InvalidDenomination((int, exp)) => write!(
                f,
                r#"invalid denomination: {{"integer": {int}, "exponent": {exp}}}"#
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
            Self::InvalidDeviceStatusLen((have, exp)) => {
                write!(
                    f,
                    "invalid device status length, have: {have}, expected: {exp}"
                )
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
            Self::InvalidUnitNumber(err) => write!(f, "invalid unit number: {err:#x}"),
            Self::InvalidFunctionStatus(err) => {
                write!(f, "invalid function status: {err:#x}")
            }
            Self::InvalidUnitStatusLen((have, exp)) => {
                write!(
                    f,
                    "invalid unit status length, have: {have}, expected: {exp}"
                )
            }
            Self::InvalidUnitStatusListLen((have, exp)) => {
                write!(
                    f,
                    "invalid unit status list length, have: {have}, expected a multiple of: {exp}"
                )
            }
            Self::InvalidStackRequestDataLen((have, exp)) => {
                write!(
                    f,
                    "invalid stack request data length, have: {have}, expected: {exp}"
                )
            }
            Self::InvalidEventLen((have, exp)) => {
                write!(f, "invalid event length, have: {have}, expected: {exp}")
            }
            Self::InvalidRequestLen((have, exp)) => {
                write!(f, "invalid request length, have: {have}, expected: {exp}")
            }
            Self::InvalidStackStatusChange(err) => {
                write!(f, "invalid stack status change: {err:#x}")
            }
            Self::InvalidRejectCode(err) => {
                write!(f, "invalid reject code: {err:#x}")
            }
            Self::InvalidRequestType(err) => {
                write!(f, "invalid request type: {err:#x}")
            }
            Self::InvalidEventType(err) => {
                write!(f, "invalid event type: {err:#x}")
            }
            #[cfg(feature = "usb")]
            Self::Usb(err) => write!(f, "USB error: {err}"),
        }
    }
}

impl std::error::Error for Error {}
