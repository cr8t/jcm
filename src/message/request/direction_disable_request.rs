use crate::{
    Error, Message, MessageCode, MessageData, MessageType, RequestCode, RequestType, Result,
};

mod direction_disable_mode;
mod direction_inhibit;
mod inhibit_direction;

pub use direction_disable_mode::*;
pub use direction_inhibit::*;
pub use inhibit_direction::*;

/// Represents a `Denomination Disable` request message.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectionDisableRequest {
    mode: DirectionDisableMode,
    direction: InhibitDirection,
}

impl DirectionDisableRequest {
    /// Creates a new [DirectionDisableRequest].
    pub const fn new() -> Self {
        Self {
            mode: DirectionDisableMode::new(),
            direction: InhibitDirection::new(),
        }
    }

    /// Gets the [MessageType] for the [DirectionDisableRequest].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Request(self.request_type())
    }

    /// Gets the [RequestType] for the [DirectionDisableRequest].
    pub const fn request_type(&self) -> RequestType {
        self.mode.to_request_type()
    }

    /// Gets the [DirectionDisableMode] for the [DirectionDisableRequest].
    ///
    /// Indirection type for setting the [RequestType].
    pub const fn mode(&self) -> DirectionDisableMode {
        self.mode
    }

    /// Sets the [DirectionDisableMode] for the [DirectionDisableRequest].
    ///
    /// Indirection type for setting the [RequestType].
    pub fn set_mode(&mut self, mode: DirectionDisableMode) {
        self.mode = mode;
    }

    /// Sets the [DirectionDisableMode] for the [DirectionDisableRequest].
    ///
    /// Indirection type for setting the [RequestType].
    pub fn with_mode(mut self, mode: DirectionDisableMode) -> Self {
        self.set_mode(mode);
        self
    }

    /// Gets the [MessageCode] for the [DirectionDisableRequest].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Request(self.request_code())
    }

    /// Gets the [RequestCode] for the [DirectionDisableRequest].
    pub const fn request_code(&self) -> RequestCode {
        RequestCode::DirectionDisable
    }

    /// Gets a reference to the list of [DirectionDisable] items.
    pub fn direction(&self) -> InhibitDirection {
        self.direction
    }

    /// Sets the list of [DirectionDisable] items.
    pub fn set_direction(&mut self, direction: InhibitDirection) {
        self.direction = direction;
    }

    /// Builder function that sets the list of [DirectionDisable] items.
    pub fn with_direction(mut self, direction: InhibitDirection) -> Self {
        self.set_direction(direction);
        self
    }
}

impl Default for DirectionDisableRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&DirectionDisableRequest> for Message {
    fn from(val: &DirectionDisableRequest) -> Self {
        MessageData::from(val).into()
    }
}

impl From<DirectionDisableRequest> for Message {
    fn from(val: DirectionDisableRequest) -> Self {
        (&val).into()
    }
}

impl From<&DirectionDisableRequest> for MessageData {
    fn from(val: &DirectionDisableRequest) -> Self {
        let data = Self::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code());

        match val.mode() {
            DirectionDisableMode::Set => data.with_additional(&[val.direction.bits()]),
            _ => data,
        }
    }
}

impl From<DirectionDisableRequest> for MessageData {
    fn from(val: DirectionDisableRequest) -> Self {
        (&val).into()
    }
}

impl TryFrom<&Message> for DirectionDisableRequest {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for DirectionDisableRequest {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&MessageData> for DirectionDisableRequest {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        let (exp_type, exp_code) = (
            MessageType::Request(RequestType::Status),
            MessageCode::Request(RequestCode::DirectionDisable),
        );

        match (val.message_type().request_type(), val.message_code()) {
            (Ok(msg_type), msg_code)
                if matches!(msg_type, RequestType::Status | RequestType::SetFeature)
                    && msg_code == exp_code =>
            {
                Ok(Self {
                    mode: msg_type.try_into()?,
                    direction: InhibitDirection::create(
                        val.additional().first().cloned().unwrap_or(0),
                    ),
                })
            }
            (_, msg_code) => Err(Error::InvalidMessage((
                (val.message_type().into(), msg_code.into()),
                (exp_type.into(), exp_code.into()),
            ))),
        }
    }
}

impl TryFrom<MessageData> for DirectionDisableRequest {
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
        let exp_code = RequestCode::DirectionDisable;
        let exp_direction = InhibitDirection::new();

        for exp_type in [RequestType::Status, RequestType::SetFeature] {
            let (msg_data, exp_req) = if exp_type == RequestType::SetFeature {
                (
                    MessageData::new()
                        .with_message_type(MessageType::Request(exp_type))
                        .with_message_code(MessageCode::Request(exp_code))
                        .with_additional(&[exp_direction.bits()]),
                    DirectionDisableRequest::new()
                        .with_mode(DirectionDisableMode::from_request_type(exp_type))
                        .with_direction(exp_direction),
                )
            } else {
                (
                    MessageData::new()
                        .with_message_type(MessageType::Request(exp_type))
                        .with_message_code(MessageCode::Request(exp_code)),
                    DirectionDisableRequest::new()
                        .with_mode(DirectionDisableMode::from_request_type(exp_type)),
                )
            };

            let msg = Message::new().with_data(msg_data);

            assert_eq!(exp_req.message_type().request_type(), Ok(exp_type));
            assert_eq!(exp_req.request_type(), exp_type);

            assert_eq!(exp_req.message_code().request_code(), Ok(exp_code));
            assert_eq!(exp_req.request_code(), exp_code);

            assert_eq!(Message::from(&exp_req), msg);
            assert_eq!(
                DirectionDisableRequest::try_from(&msg).as_ref(),
                Ok(&exp_req)
            );
        }

        Ok(())
    }

    #[test]
    fn test_status_request_invalid() -> Result<()> {
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
            RequestCode::Collect,
            RequestCode::Key,
            RequestCode::EventResendInterval,
            RequestCode::Idle,
            RequestCode::Reject,
            RequestCode::Hold,
            RequestCode::AcceptorCollect,
            RequestCode::Status,
            RequestCode::DenominationDisable,
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
                    .with_message_code(DirectionDisableRequest::new().message_code());

                let inval_code = MessageData::new()
                    .with_message_type(DirectionDisableRequest::new().message_type())
                    .with_message_code(msg_code);

                for stack_data in [inval_data, inval_type, inval_code] {
                    assert!(DirectionDisableRequest::try_from(&stack_data).is_err());
                    assert!(DirectionDisableRequest::try_from(
                        Message::new().with_data(stack_data)
                    )
                    .is_err());
                }
            }
        }

        Ok(())
    }
}
