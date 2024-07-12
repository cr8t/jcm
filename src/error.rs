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
    InvalidStackRequestLen((usize, usize)),
    InvalidEventLen((usize, usize)),
    InvalidRequestLen((usize, usize)),
    InvalidStackStatusChange(u8),
    InvalidRejectCode(u8),
    InvalidRequestType(u8),
    InvalidEventType(u8),
    InvalidMessage(((u8, u16), (u8, u16))),
    InvalidEscrowData,
    InvalidCurrency((u32, u16)),
    InvalidEscrowDataLen((usize, usize)),
    InvalidCurrencyLen((usize, usize)),
    InvalidDenominationLen((usize, usize)),
    InvalidTicketLen((usize, usize)),
    InvalidFirmwareVersionLen((usize, usize)),
    InvalidVersionResponseLen((usize, usize)),
    InvalidCurrencyAssignLen((usize, usize)),
    InvalidAlgorithmNumber(u8),
    InvalidNearFullStatus(u8),
    InvalidNearFullDataLen((usize, usize)),
    InvalidNearFullNumberLen((usize, usize)),
    InvalidNearFullMode(u8),
    InvalidImageSizeLen((usize, usize)),
    InvalidCString,
    InvalidAsciiString,
    InvalidUtf8String,
    InvalidFirmwareVersion,
    InvalidValue {
        field: &'static str,
        value: usize,
    },
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
            Self::InvalidStackRequestLen((have, exp)) => {
                write!(
                    f,
                    "invalid stack request length, have: {have}, expected: {exp}"
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
            Self::InvalidMessage(((have_type, have_code), (exp_type, exp_code))) => {
                write!(f, "invalid message, have: {have_type} - {have_code}, expected: {exp_type} - {exp_code}")
            }
            Self::InvalidEscrowData => write!(f, "invalid escrow data"),
            Self::InvalidCurrency((code, denom)) => {
                write!(
                    f,
                    r#"invalid currency: {{"code": {code}, "denomination": {denom}}}"#
                )
            }
            Self::InvalidEscrowDataLen((have, exp)) => {
                write!(
                    f,
                    "invalid escrow data length, have: {have}, expected: {exp}"
                )
            }
            Self::InvalidCurrencyLen((have, exp)) => {
                write!(f, "invalid currency length, have: {have}, expected: {exp}")
            }
            Self::InvalidDenominationLen((have, exp)) => {
                write!(
                    f,
                    "invalid denomination length, have: {have}, expected: {exp}"
                )
            }
            Self::InvalidTicketLen((have, exp)) => {
                write!(f, "invalid ticket length, have: {have}, expected: {exp}")
            }
            Self::InvalidFirmwareVersionLen((have, exp)) => {
                write!(
                    f,
                    "invalid firmware version length, have: {have}, expected: {exp}"
                )
            }
            Self::InvalidVersionResponseLen((have, exp)) => {
                write!(
                    f,
                    "invalid version response length, have: {have}, expected: {exp}"
                )
            }
            Self::InvalidCurrencyAssignLen((have, exp)) => {
                write!(
                    f,
                    "invalid currency assign length, have: {have}, expected: {exp}"
                )
            }
            Self::InvalidAlgorithmNumber(err) => {
                write!(f, "invalid algorithm number: {err:#x}")
            }
            Self::InvalidNearFullStatus(err) => {
                write!(f, "invalid near full status: {err:#x}")
            }
            Self::InvalidNearFullDataLen((have, exp)) => {
                write!(
                    f,
                    "invalid near full data length, have: {have}, expected: {exp}"
                )
            }
            Self::InvalidNearFullNumberLen((have, exp)) => {
                write!(
                    f,
                    "invalid near full number length, have: {have}, expected: {exp}"
                )
            }
            Self::InvalidNearFullMode(err) => {
                write!(f, "invalid near full mode: {err:#x}")
            }
            Self::InvalidImageSizeLen((have, exp)) => {
                write!(
                    f,
                    "invalid serial number size total length, have: {have}, expected: {exp}"
                )
            }
            Self::InvalidAsciiString => write!(f, "invalid ASCII encoded string"),
            Self::InvalidCString => write!(f, "invalid null-terminated C string"),
            Self::InvalidUtf8String => write!(f, "invalid UTF-8 encoded string"),
            Self::InvalidFirmwareVersion => write!(f, "invalid firmware version"),
            Self::InvalidValue { field, value } => {
                write!(f, "invalid {field} value: {value}")
            }
            #[cfg(feature = "usb")]
            Self::Usb(err) => write!(f, "USB error: {err}"),
        }
    }
}

impl From<std::ffi::FromBytesUntilNulError> for Error {
    fn from(_err: std::ffi::FromBytesUntilNulError) -> Self {
        Self::InvalidCString
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(_err: std::str::Utf8Error) -> Self {
        Self::InvalidUtf8String
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_err: std::string::FromUtf8Error) -> Self {
        Self::InvalidUtf8String
    }
}

impl std::error::Error for Error {}
