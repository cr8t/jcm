use crate::{
    Error, Message, MessageCode, MessageData, MessageType, RequestCode, RequestType, Result,
};

/// Represents a `Idle` request message.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CashBoxSizeRequest;

impl CashBoxSizeRequest {
    /// Creates a new [CashBoxSizeRequest].
    pub const fn new() -> Self {
        Self
    }

    /// Gets the [MessageType] for the [CashBoxSizeRequest].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Request(self.request_type())
    }

    /// Gets the [RequestType] for the [CashBoxSizeRequest].
    pub const fn request_type(&self) -> RequestType {
        RequestType::Status
    }

    /// Gets the [MessageCode] for the [CashBoxSizeRequest].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Request(self.request_code())
    }

    /// Gets the [RequestCode] for the [CashBoxSizeRequest].
    pub const fn request_code(&self) -> RequestCode {
        RequestCode::CashBoxSize
    }
}

impl Default for CashBoxSizeRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<CashBoxSizeRequest> for Message {
    fn from(val: CashBoxSizeRequest) -> Self {
        Message::new().with_data(val.into())
    }
}

impl From<&CashBoxSizeRequest> for Message {
    fn from(val: &CashBoxSizeRequest) -> Self {
        (*val).into()
    }
}

impl From<CashBoxSizeRequest> for MessageData {
    fn from(val: CashBoxSizeRequest) -> Self {
        Self::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code())
    }
}

impl From<&CashBoxSizeRequest> for MessageData {
    fn from(val: &CashBoxSizeRequest) -> Self {
        (*val).into()
    }
}

impl TryFrom<&Message> for CashBoxSizeRequest {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for CashBoxSizeRequest {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&MessageData> for CashBoxSizeRequest {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        const EXP_TYPE: MessageType = MessageType::Request(RequestType::Status);
        const EXP_CODE: MessageCode = MessageCode::Request(RequestCode::CashBoxSize);

        match (val.message_type(), val.message_code()) {
            (EXP_TYPE, EXP_CODE) => Ok(Self),
            (msg_type, msg_code) => Err(Error::InvalidMessage((
                (msg_type.into(), msg_code.into()),
                (EXP_TYPE.into(), EXP_CODE.into()),
            ))),
        }
    }
}

impl TryFrom<MessageData> for CashBoxSizeRequest {
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
    fn test_cash_box_size_request() -> Result<()> {
        let exp_type = MessageType::Request(RequestType::Status);
        let exp_code = MessageCode::Request(RequestCode::CashBoxSize);

        let msg_data = MessageData::new()
            .with_message_type(exp_type)
            .with_message_code(exp_code);
        let msg = Message::new().with_data(msg_data);

        let exp_req = CashBoxSizeRequest::new();

        assert_eq!(exp_req.message_type(), exp_type);
        assert_eq!(exp_type.request_type(), Ok(exp_req.request_type()));

        assert_eq!(exp_req.message_code(), exp_code);
        assert_eq!(exp_code.request_code(), Ok(exp_req.request_code()));

        assert_eq!(Message::from(exp_req), msg);
        assert_eq!(CashBoxSizeRequest::try_from(&msg), Ok(exp_req));

        Ok(())
    }

    #[test]
    fn test_cash_box_size_request_invalid() -> Result<()> {
        let invalid_types = [MessageType::Reserved]
            .into_iter()
            .chain((0x80..=0x8f).map(|m| MessageType::Event(EventType::from_u8(m))))
            .chain(
                [
                    RequestType::SetFeature,
                    RequestType::Operation,
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
            RequestCode::Idle,
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
                    .with_message_code(CashBoxSizeRequest::new().message_code());

                let inval_code = MessageData::new()
                    .with_message_type(CashBoxSizeRequest::new().message_type())
                    .with_message_code(msg_code);

                for stack_data in [inval_data, inval_type, inval_code] {
                    assert!(CashBoxSizeRequest::try_from(&stack_data).is_err());
                    assert!(
                        CashBoxSizeRequest::try_from(Message::new().with_data(stack_data)).is_err()
                    );
                }
            }
        }

        Ok(())
    }
}
