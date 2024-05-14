use crate::{
    Error, Message, MessageCode, MessageData, MessageType, RequestCode, RequestType, Result,
};

mod collect_mode;

pub use collect_mode::*;

/// Represents a `Collect` request message.
///
/// This request is used to collect notes from the device.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CollectRequest {
    mode: CollectMode,
}

impl CollectRequest {
    /// Creates a new [CollectRequest].
    pub const fn new() -> Self {
        Self {
            mode: CollectMode::new(),
        }
    }

    /// Creates a new [CollectRequest] from the provided parameter.
    pub const fn create(mode: CollectMode) -> Self {
        Self { mode }
    }

    /// Gets the [MessageType] for the [CollectRequest].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Request(self.request_type())
    }

    /// Gets the [RequestType] for the [CollectRequest].
    pub const fn request_type(&self) -> RequestType {
        RequestType::Operation
    }

    /// Gets the [MessageCode] for the [CollectRequest].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Request(self.request_code())
    }

    /// Gets the [RequestCode] for the [CollectRequest].
    pub const fn request_code(&self) -> RequestCode {
        self.mode.to_request_code()
    }
}

impl Default for CollectRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<CollectRequest> for Message {
    fn from(val: CollectRequest) -> Self {
        MessageData::from(val).into()
    }
}

impl From<&CollectRequest> for Message {
    fn from(val: &CollectRequest) -> Self {
        (*val).into()
    }
}

impl From<CollectRequest> for MessageData {
    fn from(val: CollectRequest) -> Self {
        Self::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code())
    }
}

impl From<&CollectRequest> for MessageData {
    fn from(val: &CollectRequest) -> Self {
        (*val).into()
    }
}

impl TryFrom<&Message> for CollectRequest {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for CollectRequest {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&MessageData> for CollectRequest {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        let (exp_type, exp_code) = (
            MessageType::Request(RequestType::Operation),
            MessageCode::Request(RequestCode::Collect),
        );

        match (val.message_type(), val.message_code()) {
            (msg_type, MessageCode::Request(code))
                if msg_type == exp_type
                    && matches!(
                        code,
                        RequestCode::Collect
                            | RequestCode::AcceptorCollect
                            | RequestCode::RecyclerCollect
                    ) =>
            {
                Ok(Self::create(CollectMode::from_request_code(code)))
            }
            (msg_type, msg_code) => Err(Error::InvalidMessage((
                (msg_type.into(), msg_code.into()),
                (exp_type.into(), exp_code.into()),
            ))),
        }
    }
}

impl TryFrom<MessageData> for CollectRequest {
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
    fn test_collect_request() -> Result<()> {
        let exp_type = MessageType::Request(RequestType::Operation);

        [
            RequestCode::Collect,
            RequestCode::AcceptorCollect,
            RequestCode::RecyclerCollect,
        ]
        .into_iter()
        .for_each(|exp_code| {
            let msg_data = MessageData::new()
                .with_message_type(exp_type)
                .with_message_code(MessageCode::Request(exp_code));
            let msg = Message::new().with_data(msg_data);

            let exp_req = CollectRequest::create(CollectMode::from_request_code(exp_code));

            assert_eq!(exp_req.message_type(), exp_type);
            assert_eq!(exp_type.request_type(), Ok(exp_req.request_type()));

            assert_eq!(exp_req.message_code(), MessageCode::Request(exp_code));
            assert_eq!(exp_req.request_code(), exp_code);

            assert_eq!(Message::from(exp_req), msg);
            assert_eq!(CollectRequest::try_from(&msg), Ok(exp_req));
        });

        Ok(())
    }

    #[test]
    fn test_collect_request_invalid() -> Result<()> {
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
            RequestCode::Stack,
            RequestCode::Inhibit,
            RequestCode::Status,
            RequestCode::Key,
            RequestCode::EventResendInterval,
            RequestCode::Idle,
            RequestCode::Reject,
            RequestCode::Hold,
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

        for &msg_type in invalid_types.iter() {
            for &msg_code in invalid_codes.iter() {
                let inval_data = MessageData::new()
                    .with_message_type(msg_type)
                    .with_message_code(msg_code);

                let inval_type = MessageData::new()
                    .with_message_type(msg_type)
                    .with_message_code(CollectRequest::new().message_code());

                let inval_code = MessageData::new()
                    .with_message_type(CollectRequest::new().message_type())
                    .with_message_code(msg_code);

                for stack_data in [inval_data, inval_type, inval_code] {
                    assert!(CollectRequest::try_from(&stack_data).is_err());
                    assert!(
                        CollectRequest::try_from(Message::new().with_data(stack_data)).is_err()
                    );
                }
            }
        }

        Ok(())
    }
}
