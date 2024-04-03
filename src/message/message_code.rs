use std::{fmt, mem};

use crate::{Error, FuncId, MessageType, Result};

mod event_code;
mod request_code;

pub use event_code::*;
pub use request_code::*;

const RESERVED: u16 = 0xffff;

#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageCode {
    Request(RequestCode),
    Event(EventCode),
    Reserved = RESERVED,
}

impl MessageCode {
    /// Creates a new [MessageCode].
    pub const fn new() -> Self {
        Self::Request(RequestCode::new())
    }

    /// Creates a new [MessageCode] from the provided parameters.
    pub const fn create(msg_type: MessageType, val: u16) -> Self {
        match msg_type {
            MessageType::Request(_ty) => Self::Request(RequestCode::from_u16(val)),
            MessageType::Event(_ty) => Self::Event(EventCode::from_u16(val)),
            _ => Self::Reserved,
        }
    }

    /// Gets whether the [MessageCode] contains a [RequestCode] variant.
    pub const fn is_request_code(&self) -> bool {
        matches!(self, Self::Request(_))
    }

    /// Converts the [MessageCode] into a [RequestCode].
    pub const fn request_code(&self) -> Result<RequestCode> {
        match self {
            Self::Request(code) => Ok(*code),
            _ => Err(Error::InvalidRequestCode(RESERVED)),
        }
    }

    /// Gets whether the [MessageCode] contains a [EventCode] variant.
    pub const fn is_event_code(&self) -> bool {
        matches!(self, Self::Event(_))
    }

    /// Converts the [MessageCode] into a [EventCode].
    pub const fn event_code(&self) -> Result<EventCode> {
        match self {
            Self::Event(code) => Ok(*code),
            _ => Err(Error::InvalidEventCode(RESERVED)),
        }
    }

    /// Gets the [FuncId] of the [MessageCode].
    pub const fn func_id(&self) -> FuncId {
        match self {
            Self::Request(code) => code.func_id(),
            Self::Event(code) => code.func_id(),
            _ => FuncId::Reserved,
        }
    }

    /// Converts the [MessageCode] to raw byte array.
    pub fn to_bytes(&self) -> [u8; 2] {
        u16::from(self).to_le_bytes()
    }

    /// Gets the length of the [MessageCode].
    pub const fn len() -> usize {
        mem::size_of::<u16>()
    }

    /// Gets whether the [MessageCode] contains a reserved variant.
    pub const fn is_empty(&self) -> bool {
        matches!(
            self,
            Self::Reserved
                | Self::Request(RequestCode::Reserved)
                | Self::Event(EventCode::Reserved)
        )
    }

    /// Gets whether the [MessageCode] is a valid variant.
    pub const fn is_valid(&self) -> bool {
        !self.is_empty()
    }
}

impl From<MessageCode> for u16 {
    fn from(val: MessageCode) -> Self {
        match val {
            MessageCode::Request(c) => c.into(),
            MessageCode::Event(c) => c.into(),
            _ => RESERVED,
        }
    }
}

impl From<&MessageCode> for u16 {
    fn from(val: &MessageCode) -> Self {
        (*val).into()
    }
}

impl TryFrom<RawMessageCode> for MessageCode {
    type Error = Error;

    fn try_from(val: RawMessageCode) -> Result<Self> {
        match Self::create(val.msg_type, val.code) {
            Self::Reserved
            | Self::Request(RequestCode::Reserved)
            | Self::Event(EventCode::Reserved) => {
                Err(Error::InvalidMessageCode((val.msg_type.into(), val.code)))
            }
            v => Ok(v),
        }
    }
}

impl fmt::Display for MessageCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(c) => write!(
                f,
                r#"{{"message_type": "request", "code": {:#x}, "details": {c}}}"#,
                u16::from(c)
            ),
            Self::Event(c) => write!(
                f,
                r#"{{"message_type": "event", "code": {:#x}, "details": {c}}}"#,
                u16::from(c)
            ),
            Self::Reserved => write!(
                f,
                r#"{{"message_type": "reserved", "code": {RESERVED:#x}, "details": "reserved"}}"#
            ),
        }
    }
}

/// Convenience struct for internal conversion implementation.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct RawMessageCode {
    pub msg_type: MessageType,
    pub code: u16,
}

impl RawMessageCode {
    /// Creates a new [RawMessageCode].
    pub const fn new() -> Self {
        Self {
            msg_type: MessageType::new(),
            code: 0,
        }
    }

    /// Creates a new [RawMessageCode] from the provided parameters.
    pub const fn create(msg_type: MessageType, code: u16) -> Self {
        Self { msg_type, code }
    }
}

impl Default for RawMessageCode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EventType, RequestType};

    #[test]
    fn test_message_code() {
        let req_type = MessageType::Request(RequestType::new());
        let event_type = MessageType::Event(EventType::new());

        let raw_vals = [
            RawMessageCode::create(req_type, u16::from(RequestCode::Uid)),
            RawMessageCode::create(req_type, u16::from(RequestCode::ProgramSignature)),
            RawMessageCode::create(req_type, u16::from(RequestCode::Version)),
            RawMessageCode::create(req_type, u16::from(RequestCode::SerialNumber)),
            RawMessageCode::create(req_type, u16::from(RequestCode::ModelName)),
            RawMessageCode::create(req_type, u16::from(RequestCode::Status)),
            RawMessageCode::create(req_type, u16::from(RequestCode::Reset)),
            RawMessageCode::create(req_type, u16::from(RequestCode::Inhibit)),
            RawMessageCode::create(req_type, u16::from(RequestCode::Collect)),
            RawMessageCode::create(req_type, u16::from(RequestCode::Key)),
            RawMessageCode::create(req_type, u16::from(RequestCode::EventResendInterval)),
            RawMessageCode::create(req_type, u16::from(RequestCode::Idle)),
            RawMessageCode::create(req_type, u16::from(RequestCode::Stack)),
            RawMessageCode::create(req_type, u16::from(RequestCode::Reject)),
            RawMessageCode::create(req_type, u16::from(RequestCode::Hold)),
            RawMessageCode::create(req_type, u16::from(RequestCode::AcceptorCollect)),
            RawMessageCode::create(req_type, u16::from(RequestCode::DenominationDisable)),
            RawMessageCode::create(req_type, u16::from(RequestCode::DirectionDisable)),
            RawMessageCode::create(req_type, u16::from(RequestCode::CurrencyAssign)),
            RawMessageCode::create(req_type, u16::from(RequestCode::CashBoxSize)),
            RawMessageCode::create(req_type, u16::from(RequestCode::NearFull)),
            RawMessageCode::create(req_type, u16::from(RequestCode::BarCode)),
            RawMessageCode::create(req_type, u16::from(RequestCode::Insert)),
            RawMessageCode::create(req_type, u16::from(RequestCode::ConditionalVend)),
            RawMessageCode::create(req_type, u16::from(RequestCode::Pause)),
            RawMessageCode::create(req_type, u16::from(RequestCode::NoteDataInfo)),
            RawMessageCode::create(req_type, u16::from(RequestCode::RecyclerCollect)),
            RawMessageCode::create(event_type, u16::from(EventCode::PowerUp)),
            RawMessageCode::create(event_type, u16::from(EventCode::PowerUpAcceptor)),
            RawMessageCode::create(event_type, u16::from(EventCode::PowerUpStacker)),
            RawMessageCode::create(event_type, u16::from(EventCode::Inhibit)),
            RawMessageCode::create(event_type, u16::from(EventCode::ProgramSignature)),
            RawMessageCode::create(event_type, u16::from(EventCode::Rejected)),
            RawMessageCode::create(event_type, u16::from(EventCode::Collected)),
            RawMessageCode::create(event_type, u16::from(EventCode::Clear)),
            RawMessageCode::create(event_type, u16::from(EventCode::OperationError)),
            RawMessageCode::create(event_type, u16::from(EventCode::Failure)),
            RawMessageCode::create(event_type, u16::from(EventCode::NoteStay)),
            RawMessageCode::create(event_type, u16::from(EventCode::PowerUpAcceptorAccepting)),
            RawMessageCode::create(event_type, u16::from(EventCode::PowerUpStackerAccepting)),
            RawMessageCode::create(event_type, u16::from(EventCode::Idle)),
            RawMessageCode::create(event_type, u16::from(EventCode::Escrow)),
            RawMessageCode::create(event_type, u16::from(EventCode::VendValid)),
            RawMessageCode::create(event_type, u16::from(EventCode::AcceptorRejected)),
            RawMessageCode::create(event_type, u16::from(EventCode::Returned)),
            RawMessageCode::create(event_type, u16::from(EventCode::AcceptorCollected)),
            RawMessageCode::create(event_type, u16::from(EventCode::Insert)),
            RawMessageCode::create(event_type, u16::from(EventCode::ConditionalVend)),
            RawMessageCode::create(event_type, u16::from(EventCode::Pause)),
            RawMessageCode::create(event_type, u16::from(EventCode::Resume)),
            RawMessageCode::create(event_type, u16::from(EventCode::AcceptorClear)),
            RawMessageCode::create(event_type, u16::from(EventCode::AcceptorOperationError)),
            RawMessageCode::create(event_type, u16::from(EventCode::AcceptorFailure)),
            RawMessageCode::create(event_type, u16::from(EventCode::AcceptorNoteStay)),
            RawMessageCode::create(event_type, u16::from(EventCode::FunctionAbeyance)),
        ];
        let expected = [
            MessageCode::Request(RequestCode::Uid),
            MessageCode::Request(RequestCode::ProgramSignature),
            MessageCode::Request(RequestCode::Version),
            MessageCode::Request(RequestCode::SerialNumber),
            MessageCode::Request(RequestCode::ModelName),
            MessageCode::Request(RequestCode::Status),
            MessageCode::Request(RequestCode::Reset),
            MessageCode::Request(RequestCode::Inhibit),
            MessageCode::Request(RequestCode::Collect),
            MessageCode::Request(RequestCode::Key),
            MessageCode::Request(RequestCode::EventResendInterval),
            MessageCode::Request(RequestCode::Idle),
            MessageCode::Request(RequestCode::Stack),
            MessageCode::Request(RequestCode::Reject),
            MessageCode::Request(RequestCode::Hold),
            MessageCode::Request(RequestCode::AcceptorCollect),
            MessageCode::Request(RequestCode::DenominationDisable),
            MessageCode::Request(RequestCode::DirectionDisable),
            MessageCode::Request(RequestCode::CurrencyAssign),
            MessageCode::Request(RequestCode::CashBoxSize),
            MessageCode::Request(RequestCode::NearFull),
            MessageCode::Request(RequestCode::BarCode),
            MessageCode::Request(RequestCode::Insert),
            MessageCode::Request(RequestCode::ConditionalVend),
            MessageCode::Request(RequestCode::Pause),
            MessageCode::Request(RequestCode::NoteDataInfo),
            MessageCode::Request(RequestCode::RecyclerCollect),
            MessageCode::Event(EventCode::PowerUp),
            MessageCode::Event(EventCode::PowerUpAcceptor),
            MessageCode::Event(EventCode::PowerUpStacker),
            MessageCode::Event(EventCode::Inhibit),
            MessageCode::Event(EventCode::ProgramSignature),
            MessageCode::Event(EventCode::Rejected),
            MessageCode::Event(EventCode::Collected),
            MessageCode::Event(EventCode::Clear),
            MessageCode::Event(EventCode::OperationError),
            MessageCode::Event(EventCode::Failure),
            MessageCode::Event(EventCode::NoteStay),
            MessageCode::Event(EventCode::PowerUpAcceptorAccepting),
            MessageCode::Event(EventCode::PowerUpStackerAccepting),
            MessageCode::Event(EventCode::Idle),
            MessageCode::Event(EventCode::Escrow),
            MessageCode::Event(EventCode::VendValid),
            MessageCode::Event(EventCode::AcceptorRejected),
            MessageCode::Event(EventCode::Returned),
            MessageCode::Event(EventCode::AcceptorCollected),
            MessageCode::Event(EventCode::Insert),
            MessageCode::Event(EventCode::ConditionalVend),
            MessageCode::Event(EventCode::Pause),
            MessageCode::Event(EventCode::Resume),
            MessageCode::Event(EventCode::AcceptorClear),
            MessageCode::Event(EventCode::AcceptorOperationError),
            MessageCode::Event(EventCode::AcceptorFailure),
            MessageCode::Event(EventCode::AcceptorNoteStay),
            MessageCode::Event(EventCode::FunctionAbeyance),
        ];

        for (raw, exp) in raw_vals.into_iter().zip(expected.into_iter()) {
            assert_eq!(MessageCode::try_from(raw), Ok(exp));
            assert_eq!(u16::from(exp), raw.code);

            assert!(exp.is_valid());
            assert!(!exp.is_empty());

            if exp.is_request_code() {
                assert_eq!(exp.request_code(), Ok(RequestCode::from_u16(raw.code)));
            }

            if exp.is_event_code() {
                assert_eq!(exp.event_code(), Ok(EventCode::from_u16(raw.code)));
            }
        }

        for val in (0..=0x1fffu16).filter(|s| !raw_vals.iter().any(|d| d.code == *s)) {
            let req_code = RawMessageCode::create(req_type, val);
            let event_code = RawMessageCode::create(event_type, val);

            assert!(MessageCode::try_from(req_code).is_err());
            assert!(MessageCode::try_from(event_code).is_err());

            let res_req = MessageCode::create(req_code.msg_type, req_code.code);
            let res_event = MessageCode::create(event_code.msg_type, event_code.code);

            assert_eq!(res_req, MessageCode::Request(RequestCode::Reserved));
            assert_eq!(res_event, MessageCode::Event(EventCode::Reserved));

            assert!(res_req.is_empty());
            assert!(!res_req.is_valid());

            assert!(res_event.is_empty());
            assert!(!res_event.is_valid());
        }
    }
}
