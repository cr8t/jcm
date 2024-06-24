use crate::{
    Error, ImageBlockNumber, Message, MessageCode, MessageData, MessageType, RequestCode,
    RequestType, Result,
};

/// Represents a `Serial Number Image` request message.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SerialNumberRequest {
    block_number: ImageBlockNumber,
}

impl SerialNumberRequest {
    /// Creates a new [SerialNumberRequest].
    pub const fn new() -> Self {
        Self {
            block_number: ImageBlockNumber::new(),
        }
    }

    /// Gets the [MessageType] for the [SerialNumberRequest].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Request(self.request_type())
    }

    /// Gets the [RequestType] for the [SerialNumberRequest].
    pub const fn request_type(&self) -> RequestType {
        RequestType::Status
    }

    /// Gets the [MessageCode] for the [SerialNumberRequest].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Request(self.request_code())
    }

    /// Gets the [RequestCode] for the [SerialNumberRequest].
    pub const fn request_code(&self) -> RequestCode {
        RequestCode::SerialNumber
    }

    /// Gets the [BlockNumber](ImageBlockNumber) for the [SerialNumberRequest].
    ///
    /// **NOTE**: block number `00h` is used to request the size and total number of blocks from
    /// the device.
    pub const fn block_number(&self) -> ImageBlockNumber {
        self.block_number
    }

    /// Sets the [BlockNumber](ImageBlockNumber) for the [SerialNumberRequest].
    ///
    /// **NOTE**: block number `00h` is used to request the size and total number of blocks from
    /// the device.
    pub fn set_block_number(&mut self, val: ImageBlockNumber) {
        self.block_number = val;
    }

    /// Builder function that sets the [BlockNumber](ImageBlockNumber) for the [SerialNumberRequest].
    ///
    /// **NOTE**: block number `00h` is used to request the size and total number of blocks from
    /// the device.
    pub const fn with_block_number(self, val: ImageBlockNumber) -> Self {
        Self { block_number: val }
    }
}

impl Default for SerialNumberRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<SerialNumberRequest> for Message {
    fn from(val: SerialNumberRequest) -> Self {
        MessageData::from(val).into()
    }
}

impl From<&SerialNumberRequest> for Message {
    fn from(val: &SerialNumberRequest) -> Self {
        (*val).into()
    }
}

impl From<SerialNumberRequest> for MessageData {
    fn from(val: SerialNumberRequest) -> Self {
        Self::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code())
            .with_additional(val.block_number().into_bytes().as_ref())
    }
}

impl From<&SerialNumberRequest> for MessageData {
    fn from(val: &SerialNumberRequest) -> Self {
        (*val).into()
    }
}

impl TryFrom<&Message> for SerialNumberRequest {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for SerialNumberRequest {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&MessageData> for SerialNumberRequest {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        let (exp_type, exp_code) = (
            MessageType::Request(RequestType::Status),
            MessageCode::Request(RequestCode::SerialNumber),
        );

        match (val.message_type(), val.message_code()) {
            (msg_type, msg_code) if msg_type == exp_type && msg_code == exp_code => Ok(Self {
                block_number: val
                    .additional()
                    .first()
                    .copied()
                    .ok_or(Error::InvalidMessageDataLen((0, ImageBlockNumber::LEN)))?
                    .into(),
            }),
            (msg_type, msg_code) => Err(Error::InvalidMessage((
                (msg_type.into(), msg_code.into()),
                (exp_type.into(), exp_code.into()),
            ))),
        }
    }
}

impl TryFrom<MessageData> for SerialNumberRequest {
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
    fn test_status_request() -> Result<()> {
        let exp_type = MessageType::Request(RequestType::Status);
        let exp_code = MessageCode::Request(RequestCode::SerialNumber);
        let exp_num = ImageBlockNumber::new();

        let msg_data = MessageData::new()
            .with_message_type(exp_type)
            .with_message_code(exp_code)
            .with_additional(exp_num.into_bytes().as_ref());
        let msg = Message::new().with_data(msg_data);

        let exp_req = SerialNumberRequest::new();

        assert_eq!(exp_req.message_type(), exp_type);
        assert_eq!(exp_type.request_type(), Ok(exp_req.request_type()));

        assert_eq!(exp_req.message_code(), exp_code);
        assert_eq!(exp_code.request_code(), Ok(exp_req.request_code()));

        assert_eq!(exp_req.block_number(), exp_num);

        assert_eq!(Message::from(exp_req), msg);
        assert_eq!(SerialNumberRequest::try_from(&msg), Ok(exp_req));

        Ok(())
    }

    #[test]
    fn test_status_request_invalid() -> Result<()> {
        let invalid_types = [MessageType::Reserved]
            .into_iter()
            .chain((0x80..=0x8f).map(|m| MessageType::Event(EventType::from_u8(m))))
            .chain(
                [
                    RequestType::Operation,
                    RequestType::SetFeature,
                    RequestType::Reserved,
                ]
                .map(MessageType::Request),
            )
            .collect::<Vec<MessageType>>();

        let invalid_codes = [
            RequestCode::Uid,
            RequestCode::ProgramSignature,
            RequestCode::Version,
            RequestCode::Status,
            RequestCode::ModelName,
            RequestCode::Reset,
            RequestCode::Stack,
            RequestCode::Inhibit,
            RequestCode::Collect,
            RequestCode::Key,
            RequestCode::EventResendInterval,
            RequestCode::Idle,
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
                    .with_message_code(SerialNumberRequest::new().message_code());

                let inval_code = MessageData::new()
                    .with_message_type(SerialNumberRequest::new().message_type())
                    .with_message_code(msg_code);

                for stack_data in [inval_data, inval_type, inval_code] {
                    assert!(SerialNumberRequest::try_from(&stack_data).is_err());
                    assert!(
                        SerialNumberRequest::try_from(Message::new().with_data(stack_data))
                            .is_err()
                    );
                }
            }
        }

        Ok(())
    }
}
