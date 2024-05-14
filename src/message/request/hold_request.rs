use crate::{
    Error, Message, MessageCode, MessageData, MessageType, RequestCode, RequestType, Result,
};

mod hold_timeout;

pub use hold_timeout::*;

/// Represents a `Hold` request message.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HoldRequest {
    timeout: HoldTimeout,
}

impl HoldRequest {
    /// Creates a new [HoldRequest].
    pub const fn new() -> Self {
        Self {
            timeout: HoldTimeout::new(),
        }
    }

    /// Creates a new [HoldRequest] from the provided parameters.
    ///
    /// - `timeout`: number of seconds to hold a note in escrow.
    pub const fn create(timeout: u16) -> Self {
        Self {
            timeout: HoldTimeout::from_u16(timeout),
        }
    }

    /// Gets the [MessageType] for the [HoldRequest].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Request(self.request_type())
    }

    /// Gets the [RequestType] for the [HoldRequest].
    pub const fn request_type(&self) -> RequestType {
        RequestType::Operation
    }

    /// Gets the [MessageCode] for the [HoldRequest].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Request(self.request_code())
    }

    /// Gets the [RequestCode] for the [HoldRequest].
    pub const fn request_code(&self) -> RequestCode {
        RequestCode::Hold
    }

    /// Gets the timeout in seconds to hold a note in escrow.
    pub const fn timeout(&self) -> HoldTimeout {
        self.timeout
    }

    /// Sets the timeout in seconds to hold a note in escrow.
    pub fn set_timeout(&mut self, val: HoldTimeout) {
        self.timeout = val;
    }

    /// Builder function that sets the timeout in seconds to hold a note in escrow.
    pub fn with_timeout(mut self, val: HoldTimeout) -> Self {
        self.set_timeout(val);
        self
    }
}

impl Default for HoldRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<HoldRequest> for Message {
    fn from(val: HoldRequest) -> Self {
        MessageData::from(val).into()
    }
}

impl From<&HoldRequest> for Message {
    fn from(val: &HoldRequest) -> Self {
        (*val).into()
    }
}

impl From<HoldRequest> for MessageData {
    fn from(val: HoldRequest) -> Self {
        Self::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code())
            .with_additional(val.timeout().to_bytes().as_ref())
    }
}

impl From<&HoldRequest> for MessageData {
    fn from(val: &HoldRequest) -> Self {
        (*val).into()
    }
}

impl TryFrom<&Message> for HoldRequest {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for HoldRequest {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&MessageData> for HoldRequest {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        let (exp_type, exp_code) = (
            MessageType::Request(RequestType::Operation),
            MessageCode::Request(RequestCode::Hold),
        );

        let timeout = HoldTimeout::try_from(val.additional())?;

        match (val.message_type(), val.message_code()) {
            (msg_type, msg_code) if msg_type == exp_type && msg_code == exp_code => {
                Ok(Self { timeout })
            }
            (msg_type, msg_code) => Err(Error::InvalidMessage((
                (msg_type.into(), msg_code.into()),
                (exp_type.into(), exp_code.into()),
            ))),
        }
    }
}

impl TryFrom<MessageData> for HoldRequest {
    type Error = Error;

    fn try_from(val: MessageData) -> Result<Self> {
        (&val).try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EventCode, EventType};

    #[test]
    fn test_hold_request() -> Result<()> {
        let exp_type = MessageType::Request(RequestType::Operation);
        let exp_code = MessageCode::Request(RequestCode::Hold);
        let exp_timeout = HoldTimeout::new();

        let msg_data = MessageData::new()
            .with_message_type(exp_type)
            .with_message_code(exp_code)
            .with_additional(exp_timeout.to_bytes().as_ref());
        let msg = Message::new().with_data(msg_data);

        let exp_req = HoldRequest::new();

        assert_eq!(exp_req.message_type(), exp_type);
        assert_eq!(exp_type.request_type(), Ok(exp_req.request_type()));

        assert_eq!(exp_req.message_code(), exp_code);
        assert_eq!(exp_code.request_code(), Ok(exp_req.request_code()));

        assert_eq!(exp_req.timeout(), exp_timeout);

        assert_eq!(Message::from(exp_req), msg);
        assert_eq!(HoldRequest::try_from(&msg), Ok(exp_req));

        Ok(())
    }

    #[test]
    fn test_hold_request_invalid() -> Result<()> {
        let invalid_types = [MessageType::Reserved]
            .into_iter()
            .chain((0x80..=0x8f).map(|m| MessageType::Event(EventType::from_u8(m))))
            .chain(
                [
                    RequestType::SetFeature,
                    RequestType::Status,
                    RequestType::Reserved,
                ]
                .map(MessageType::Request),
            )
            .collect::<Vec<MessageType>>();

        let invalid_codes = [
            RequestCode::Uid,
            RequestCode::ProgramSignature,
            RequestCode::Version,
            RequestCode::SerialNumber,
            RequestCode::ModelName,
            RequestCode::Reset,
            RequestCode::Status,
            RequestCode::Stack,
            RequestCode::Inhibit,
            RequestCode::Collect,
            RequestCode::Key,
            RequestCode::EventResendInterval,
            RequestCode::Idle,
            RequestCode::Reject,
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
            RequestCode::Reserved,
        ]
        .map(MessageCode::Request)
        .into_iter()
        .chain(
            [
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
                EventCode::Reserved,
            ]
            .map(MessageCode::Event),
        )
        .collect::<Vec<MessageCode>>();

        let val_data = [0u8; 2];

        for &msg_type in invalid_types.iter() {
            for &msg_code in invalid_codes.iter() {
                let inval_data = MessageData::new()
                    .with_message_type(msg_type)
                    .with_message_code(msg_code);

                let inval_type = MessageData::new()
                    .with_message_type(msg_type)
                    .with_message_code(HoldRequest::new().message_code())
                    .with_additional(val_data.as_ref());

                let inval_code = MessageData::new()
                    .with_message_type(HoldRequest::new().message_type())
                    .with_message_code(msg_code)
                    .with_additional(val_data.as_ref());

                for stack_data in [inval_data, inval_type, inval_code] {
                    assert!(HoldRequest::try_from(&stack_data).is_err());
                    assert!(HoldRequest::try_from(Message::new().with_data(stack_data)).is_err());
                }
            }
        }

        Ok(())
    }
}
