use crate::{
    Error, Message, MessageCode, MessageData, MessageType, NearFullData, RequestCode, RequestType,
    Result,
};

mod near_full_mode;

pub use near_full_mode::*;

/// Represents a `Near Full` request message.
///
/// This request is used to get/set the `Near Full` threshold of the device.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NearFullRequest {
    mode: NearFullMode,
    data: Option<NearFullData>,
}

impl NearFullRequest {
    /// Creates a new [NearFullRequest].
    pub const fn new() -> Self {
        Self {
            mode: NearFullMode::new(),
            data: None,
        }
    }

    /// Gets the [MessageType] for the [NearFullRequest].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Request(self.request_type())
    }

    /// Gets the [RequestType] for the [NearFullRequest].
    pub const fn request_type(&self) -> RequestType {
        self.mode.into_request_type()
    }

    /// Gets the [MessageCode] for the [NearFullRequest].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Request(self.request_code())
    }

    /// Gets the [RequestCode] for the [NearFullRequest].
    pub const fn request_code(&self) -> RequestCode {
        RequestCode::NearFull
    }

    /// Gets the [NearFullMode] for the [NearFullRequest].
    pub const fn mode(&self) -> NearFullMode {
        self.mode
    }

    /// Sets the [NearFullMode] for the [NearFullRequest].
    pub fn set_mode(&mut self, mode: NearFullMode) {
        self.mode = mode;
    }

    /// Builder function that sets the [NearFullMode] for the [NearFullRequest].
    pub const fn with_mode(self, mode: NearFullMode) -> Self {
        Self {
            mode,
            data: self.data,
        }
    }

    /// Gets the [NearFullData] for the [NearFullRequest].
    ///
    /// [NearFullData] is only set for [Set](NearFullMode::Set) requests.
    pub const fn data(&self) -> Option<NearFullData> {
        self.data
    }

    /// Sets the [NearFullData] for the [NearFullRequest].
    ///
    /// [NearFullData] is only set for [Set](NearFullMode::Set) requests.
    pub fn set_data(&mut self, data: NearFullData) {
        self.data = Some(data);
    }

    /// Unsets the [NearFullData] for the [NearFullRequest].
    pub fn unset_data(&mut self) -> Option<NearFullData> {
        self.data.take()
    }

    /// Builder function that sets the [NearFullData] for the [NearFullRequest].
    ///
    /// [NearFullData] is only set for [Set](NearFullMode::Set) requests.
    pub const fn with_data(self, data: NearFullData) -> Self {
        Self {
            mode: self.mode,
            data: Some(data),
        }
    }
}

impl Default for NearFullRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<NearFullRequest> for Message {
    fn from(val: NearFullRequest) -> Self {
        MessageData::from(val).into()
    }
}

impl From<&NearFullRequest> for Message {
    fn from(val: &NearFullRequest) -> Self {
        (*val).into()
    }
}

impl From<NearFullRequest> for MessageData {
    fn from(val: NearFullRequest) -> Self {
        match val.mode() {
            NearFullMode::Get => Self::new()
                .with_message_type(val.message_type())
                .with_message_code(val.message_code()),
            NearFullMode::Set => Self::new()
                .with_message_type(val.message_type())
                .with_message_code(val.message_code())
                .with_additional(val.data().unwrap_or_default().into_bytes().as_ref()),
        }
    }
}

impl From<&NearFullRequest> for MessageData {
    fn from(val: &NearFullRequest) -> Self {
        (*val).into()
    }
}

impl TryFrom<&Message> for NearFullRequest {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for NearFullRequest {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&MessageData> for NearFullRequest {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        // could also be NearFullMode::Set
        let exp_type = MessageType::from(NearFullMode::Get);
        let exp_code = MessageCode::Request(RequestCode::NearFull);

        match (val.message_type(), val.message_code()) {
            (msg_type, msg_code) if msg_code == exp_code => {
                let mode = NearFullMode::try_from(msg_type.request_type()?)?;
                match mode {
                    NearFullMode::Get => Ok(Self::new().with_mode(mode)),
                    NearFullMode::Set => Ok(Self::new()
                        .with_mode(mode)
                        .with_data(NearFullData::try_from(val.additional())?)),
                }
            }
            (msg_type, msg_code) => Err(Error::InvalidMessage((
                (msg_type.into(), msg_code.into()),
                (exp_type.into(), exp_code.into()),
            ))),
        }
    }
}

impl TryFrom<MessageData> for NearFullRequest {
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
    fn test_near_full_request() -> Result<()> {
        let exp_code = RequestCode::NearFull;
        let exp_data = NearFullData::new();

        for exp_type in [RequestType::Status, RequestType::SetFeature] {
            let exp_mode = exp_type.try_into()?;

            let msg_data = MessageData::new()
                .with_message_type(MessageType::Request(exp_type))
                .with_message_code(MessageCode::Request(exp_code));

            let msg = match exp_mode {
                NearFullMode::Get => Message::new().with_data(msg_data),
                NearFullMode::Set => Message::new()
                    .with_data(msg_data.with_additional(exp_data.into_bytes().as_ref())),
            };

            let exp_req = match exp_mode {
                NearFullMode::Get => NearFullRequest::new().with_mode(exp_type.try_into()?),
                NearFullMode::Set => NearFullRequest::new()
                    .with_mode(exp_type.try_into()?)
                    .with_data(exp_data),
            };

            assert_eq!(exp_req.message_type(), MessageType::Request(exp_type));
            assert_eq!(exp_req.request_type(), exp_type);

            assert_eq!(exp_req.message_code(), MessageCode::Request(exp_code));
            assert_eq!(exp_req.request_code(), exp_code);

            assert_eq!(exp_req.mode(), exp_mode);
            match exp_mode {
                NearFullMode::Get => assert_eq!(exp_req.data(), None),
                NearFullMode::Set => assert_eq!(exp_req.data(), Some(exp_data)),
            };

            assert_eq!(Message::from(exp_req), msg);
            assert_eq!(NearFullRequest::try_from(&msg), Ok(exp_req));
        }

        Ok(())
    }

    #[test]
    fn test_near_full_request_invalid() -> Result<()> {
        let invalid_types = [MessageType::Reserved]
            .into_iter()
            .chain((0x80..=0x8f).map(|m| MessageType::Event(EventType::from_u8(m))))
            .chain([RequestType::Operation, RequestType::Reserved].map(MessageType::Request))
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
            RequestCode::Collect,
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
                    .with_message_code(NearFullRequest::new().message_code());

                let inval_code = MessageData::new()
                    .with_message_type(NearFullRequest::new().message_type())
                    .with_message_code(msg_code);

                for stack_data in [inval_data, inval_type, inval_code] {
                    assert!(NearFullRequest::try_from(&stack_data).is_err());
                    assert!(
                        NearFullRequest::try_from(Message::new().with_data(stack_data)).is_err()
                    );
                }
            }
        }

        Ok(())
    }
}
