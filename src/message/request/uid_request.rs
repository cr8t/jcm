use crate::{
    Error, Message, MessageCode, MessageData, MessageType, RequestCode, RequestMode, RequestType,
    Result,
};

/// Represents a `Status` request message.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UidRequest {
    uid: u8,
    mode: RequestMode,
}

impl UidRequest {
    /// Creates a new [UidRequest].
    pub const fn new() -> Self {
        Self {
            uid: 0,
            mode: RequestMode::new(),
        }
    }

    /// Creates a new [UidRequest] to [get](RequestMode::Get) the UID.
    pub const fn new_get() -> Self {
        Self {
            uid: 0,
            mode: RequestMode::Get,
        }
    }

    /// Creates a new [UidRequest] to [set](RequestMode::Set) the UID.
    pub const fn new_set(uid: u8) -> Self {
        Self {
            uid,
            mode: RequestMode::Set,
        }
    }

    /// Gets the UID for the [UidRequest].
    pub const fn uid(&self) -> u8 {
        self.uid
    }

    /// Sets the UID for the [UidRequest].
    pub fn set_uid(&mut self, uid: u8) {
        self.uid = uid;
    }

    /// Builder function that sets the UID for the [UidRequest].
    pub fn with_uid(mut self, uid: u8) -> Self {
        self.set_uid(uid);
        self
    }

    /// Gets the [RequestMode] for the [UidRequest].
    pub const fn request_mode(&self) -> RequestMode {
        self.mode
    }

    /// Sets the [RequestMode] for the [UidRequest].
    pub fn set_request_mode(&mut self, mode: RequestMode) {
        self.mode = mode;
    }

    /// Builder function that sets the [RequestMode] for the [UidRequest].
    pub fn with_request_mode(mut self, mode: RequestMode) -> Self {
        self.set_request_mode(mode);
        self
    }

    /// Gets the [MessageType] for the [UidRequest].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Request(self.request_type())
    }

    /// Gets the [RequestType] for the [UidRequest].
    pub const fn request_type(&self) -> RequestType {
        match self.mode {
            RequestMode::Get => RequestType::Status,
            RequestMode::Set => RequestType::SetFeature,
        }
    }

    /// Gets the [MessageCode] for the [UidRequest].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Request(self.request_code())
    }

    /// Gets the [RequestCode] for the [UidRequest].
    pub const fn request_code(&self) -> RequestCode {
        RequestCode::Uid
    }
}

impl Default for UidRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<UidRequest> for Message {
    fn from(val: UidRequest) -> Self {
        MessageData::from(val).into()
    }
}

impl From<&UidRequest> for Message {
    fn from(val: &UidRequest) -> Self {
        (*val).into()
    }
}

impl From<UidRequest> for MessageData {
    fn from(val: UidRequest) -> Self {
        let ret = Self::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code());

        match val.mode {
            RequestMode::Get => ret,
            RequestMode::Set => ret.with_additional(&[val.uid]),
        }
    }
}

impl From<&UidRequest> for MessageData {
    fn from(val: &UidRequest) -> Self {
        (*val).into()
    }
}

impl TryFrom<&Message> for UidRequest {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for UidRequest {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&MessageData> for UidRequest {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        let (exp_get_type, exp_set_type, exp_code) = (
            MessageType::Request(RequestType::Status),
            MessageType::Request(RequestType::SetFeature),
            MessageCode::Request(RequestCode::Uid),
        );

        match (val.message_type(), val.message_code()) {
            (msg_type, msg_code) if msg_type == exp_get_type && msg_code == exp_code => {
                Ok(Self::new())
            }
            (msg_type, msg_code) if msg_type == exp_set_type && msg_code == exp_code => {
                Ok(Self::new_set(
                    val.additional()
                        .first()
                        .cloned()
                        .ok_or(Error::InvalidMessageDataLen((0, 1)))?,
                ))
            }
            (msg_type, msg_code) => Err(Error::InvalidMessage((
                (msg_type.into(), msg_code.into()),
                (exp_get_type.into(), exp_code.into()),
            ))),
        }
    }
}

impl TryFrom<MessageData> for UidRequest {
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
    fn test_uid_request() -> Result<()> {
        let exp_modes = [RequestMode::Get, RequestMode::Set];
        let exp_types = [
            MessageType::Request(RequestType::Status),
            MessageType::Request(RequestType::SetFeature),
        ];
        let exp_code = MessageCode::Request(RequestCode::Uid);

        let uid = 1;

        for (mode, exp_type) in exp_modes.into_iter().zip(exp_types) {
            let msg_data = MessageData::new()
                .with_message_type(exp_type)
                .with_message_code(exp_code);

            let (exp_req, msg) = match mode {
                RequestMode::Get => (UidRequest::new_get(), msg_data.into()),
                RequestMode::Set => (
                    UidRequest::new_set(uid),
                    msg_data.with_additional(&[uid]).into(),
                ),
            };

            assert_eq!(exp_req.message_type(), exp_type);
            assert_eq!(exp_type.request_type(), Ok(exp_req.request_type()));

            assert_eq!(exp_req.message_code(), exp_code);
            assert_eq!(exp_code.request_code(), Ok(exp_req.request_code()));

            assert_eq!(Message::from(exp_req), msg);
            assert_eq!(UidRequest::try_from(&msg), Ok(exp_req));
        }

        Ok(())
    }

    #[test]
    fn test_uid_request_invalid() -> Result<()> {
        let invalid_types = [MessageType::Reserved]
            .into_iter()
            .chain((0x80..=0x8f).map(|m| MessageType::Event(EventType::from_u8(m))))
            .chain([RequestType::Operation, RequestType::Reserved].map(MessageType::Request))
            .collect::<Vec<MessageType>>();

        let invalid_codes = [
            RequestCode::ProgramSignature,
            RequestCode::Version,
            RequestCode::SerialNumber,
            RequestCode::ModelName,
            RequestCode::Reset,
            RequestCode::Stack,
            RequestCode::Status,
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
                    .with_message_code(UidRequest::new().message_code());

                let inval_code = MessageData::new()
                    .with_message_type(UidRequest::new().message_type())
                    .with_message_code(msg_code);

                for stack_data in [inval_data, inval_type, inval_code] {
                    assert!(UidRequest::try_from(&stack_data).is_err());
                    assert!(UidRequest::try_from(Message::new().with_data(stack_data)).is_err());
                }
            }
        }

        Ok(())
    }
}
