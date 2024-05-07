use crate::{
    Error, Message, MessageCode, MessageData, MessageType, RequestCode, RequestType, Result,
};

/// Represents a `Idle` request message.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdleRequest;

impl IdleRequest {
    /// Creates a new [IdleRequest].
    pub const fn new() -> Self {
        Self
    }

    /// Gets the [MessageType] for the [IdleRequest].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Request(self.request_type())
    }

    /// Gets the [RequestType] for the [IdleRequest].
    pub const fn request_type(&self) -> RequestType {
        RequestType::Operation
    }

    /// Gets the [MessageCode] for the [IdleRequest].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Request(self.request_code())
    }

    /// Gets the [RequestCode] for the [IdleRequest].
    pub const fn request_code(&self) -> RequestCode {
        RequestCode::Idle
    }
}

impl Default for IdleRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<IdleRequest> for Message {
    fn from(val: IdleRequest) -> Self {
        Message::new().with_data(val.into())
    }
}

impl From<&IdleRequest> for Message {
    fn from(val: &IdleRequest) -> Self {
        (*val).into()
    }
}

impl From<IdleRequest> for MessageData {
    fn from(val: IdleRequest) -> Self {
        Self::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code())
    }
}

impl From<&IdleRequest> for MessageData {
    fn from(val: &IdleRequest) -> Self {
        (*val).into()
    }
}

impl TryFrom<&Message> for IdleRequest {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for IdleRequest {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&MessageData> for IdleRequest {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        let (exp_type, exp_code) = (
            MessageType::Request(RequestType::Operation),
            MessageCode::Request(RequestCode::Idle),
        );

        match (val.message_type(), val.message_code()) {
            (msg_type, msg_code) if msg_type == exp_type && msg_code == exp_code => Ok(Self),
            (msg_type, msg_code) => Err(Error::InvalidMessage((
                (msg_type.into(), msg_code.into()),
                (exp_type.into(), exp_code.into()),
            ))),
        }
    }
}

impl TryFrom<MessageData> for IdleRequest {
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
    fn test_idle_request() -> Result<()> {
        let exp_type = MessageType::Request(RequestType::Operation);
        let exp_code = MessageCode::Request(RequestCode::Idle);

        let msg_data = MessageData::new()
            .with_message_type(exp_type)
            .with_message_code(exp_code);
        let msg = Message::new().with_data(msg_data);

        let exp_req = IdleRequest::new();

        assert_eq!(exp_req.message_type(), exp_type);
        assert_eq!(exp_type.request_type(), Ok(exp_req.request_type()));

        assert_eq!(exp_req.message_code(), exp_code);
        assert_eq!(exp_code.request_code(), Ok(exp_req.request_code()));

        assert_eq!(Message::from(exp_req), msg);
        assert_eq!(IdleRequest::try_from(&msg), Ok(exp_req));

        Ok(())
    }

    #[test]
    fn test_idle_request_invalid() -> Result<()> {
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
            RequestCode::Status,
            RequestCode::Stack,
            RequestCode::Inhibit,
            RequestCode::Collect,
            RequestCode::Key,
            RequestCode::EventResendInterval,
            RequestCode::Reset,
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

        for &msg_type in invalid_types.iter() {
            for &msg_code in invalid_codes.iter() {
                let inval_data = MessageData::new()
                    .with_message_type(msg_type)
                    .with_message_code(msg_code);

                let inval_type = MessageData::new()
                    .with_message_type(msg_type)
                    .with_message_code(IdleRequest::new().message_code());

                let inval_code = MessageData::new()
                    .with_message_type(IdleRequest::new().message_type())
                    .with_message_code(msg_code);

                for stack_data in [inval_data, inval_type, inval_code] {
                    assert!(IdleRequest::try_from(&stack_data).is_err());
                    assert!(IdleRequest::try_from(Message::new().with_data(stack_data)).is_err());
                }
            }
        }

        Ok(())
    }
}
