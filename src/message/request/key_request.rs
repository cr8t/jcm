use crate::{
    Error, KeySetting, KeySettingList, Message, MessageCode, MessageData, MessageType, RequestCode,
    RequestType, Result,
};

mod mode;

pub use mode::*;

/// Represents a `Status` request message.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyRequest {
    settings: KeySettingList,
    mode: KeyMode,
}

impl KeyRequest {
    /// Creates a new [KeyRequest].
    pub const fn new() -> Self {
        Self {
            settings: KeySettingList::new(),
            mode: KeyMode::new(),
        }
    }

    /// Gets the [KeySetting]s for the [KeyRequest].
    pub fn settings(&self) -> &[KeySetting] {
        self.settings.items()
    }

    /// Sets the [KeySetting]s for the [KeyRequest].
    pub fn set_settings(&mut self, settings: &[KeySetting]) {
        self.settings = settings.into();
    }

    /// Builder function that sets the [KeySetting]s for the [KeyRequest].
    pub fn with_settings(mut self, settings: &[KeySetting]) -> Self {
        self.set_settings(settings);
        self
    }

    /// Gets the [KeyMode] for the [KeyRequest].
    pub const fn request_mode(&self) -> KeyMode {
        self.mode
    }

    /// Sets the [KeyMode] for the [KeyRequest].
    pub fn set_request_mode(&mut self, mode: KeyMode) {
        self.mode = mode;
    }

    /// Builder function that sets the [KeyMode] for the [KeyRequest].
    pub fn with_request_mode(mut self, mode: KeyMode) -> Self {
        self.set_request_mode(mode);
        self
    }

    /// Gets the [MessageType] for the [KeyRequest].
    pub const fn message_type(&self) -> MessageType {
        self.mode.into_message_type()
    }

    /// Gets the [RequestType] for the [KeyRequest].
    pub const fn request_type(&self) -> RequestType {
        self.mode.into_request_type()
    }

    /// Gets the [MessageCode] for the [KeyRequest].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Request(self.request_code())
    }

    /// Gets the [RequestCode] for the [KeyRequest].
    pub const fn request_code(&self) -> RequestCode {
        RequestCode::Key
    }
}

impl Default for KeyRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&KeyRequest> for Message {
    fn from(val: &KeyRequest) -> Self {
        MessageData::from(val).into()
    }
}

impl From<KeyRequest> for Message {
    fn from(val: KeyRequest) -> Self {
        (&val).into()
    }
}

impl From<&KeyRequest> for MessageData {
    fn from(val: &KeyRequest) -> Self {
        let ret = Self::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code());

        match val.mode {
            KeyMode::Get => ret,
            KeyMode::Set => {
                ret.emplace_additional(val.settings.clone().into_iter_bytes().collect())
            }
        }
    }
}

impl From<KeyRequest> for MessageData {
    fn from(val: KeyRequest) -> Self {
        (&val).into()
    }
}

impl TryFrom<&Message> for KeyRequest {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for KeyRequest {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&MessageData> for KeyRequest {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        const EXP_GET_TYPE: MessageType = MessageType::Request(RequestType::Status);
        const EXP_SET_TYPE: MessageType = MessageType::Request(RequestType::SetFeature);
        const EXP_CODE: MessageCode = MessageCode::Request(RequestCode::Key);

        match (val.message_type(), val.message_code()) {
            (EXP_GET_TYPE, EXP_CODE) => Ok(Self::new()),
            (EXP_SET_TYPE, EXP_CODE) => Ok(Self::new()
                .with_request_mode(KeyMode::Set)
                .with_settings(KeySettingList::try_from(val.additional())?.items())),
            (msg_type, msg_code) => Err(Error::InvalidMessage((
                (msg_type.into(), msg_code.into()),
                (EXP_GET_TYPE.into(), EXP_CODE.into()),
            ))),
        }
    }
}

impl TryFrom<MessageData> for KeyRequest {
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
    fn test_key_request() -> Result<()> {
        let exp_modes = [KeyMode::Get, KeyMode::Set];
        let exp_types = [
            MessageType::Request(RequestType::Status),
            MessageType::Request(RequestType::SetFeature),
        ];
        let exp_code = MessageCode::Request(RequestCode::Key);

        for (mode, exp_type) in exp_modes.into_iter().zip(exp_types) {
            let msg_data = MessageData::new()
                .with_message_type(exp_type)
                .with_message_code(exp_code);

            let (exp_req, msg) = match mode {
                KeyMode::Get => (KeyRequest::new(), msg_data.into()),
                KeyMode::Set => (
                    KeyRequest::new()
                        .with_request_mode(KeyMode::Set)
                        .with_settings(&[KeySetting::Enabled]),
                    msg_data
                        .with_additional(&[KeySetting::Enabled.into_u8()])
                        .into(),
                ),
            };

            assert_eq!(exp_req.message_type(), exp_type);
            assert_eq!(exp_type.request_type(), Ok(exp_req.request_type()));

            assert_eq!(exp_req.message_code(), exp_code);
            assert_eq!(exp_code.request_code(), Ok(exp_req.request_code()));

            assert_eq!(Message::from(&exp_req), msg);
            assert_eq!(KeyRequest::try_from(&msg), Ok(exp_req));
        }

        Ok(())
    }

    #[test]
    fn test_key_request_invalid() -> Result<()> {
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
            RequestCode::Uid,
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
                    .with_message_code(KeyRequest::new().message_code());

                let inval_code = MessageData::new()
                    .with_message_type(KeyRequest::new().message_type())
                    .with_message_code(msg_code);

                for stack_data in [inval_data, inval_type, inval_code] {
                    assert!(KeyRequest::try_from(&stack_data).is_err());
                    assert!(KeyRequest::try_from(Message::new().with_data(stack_data)).is_err());
                }
            }
        }

        Ok(())
    }
}
