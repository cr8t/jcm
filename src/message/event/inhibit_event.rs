use crate::{Error, EventCode, EventType, Message, MessageCode, MessageData, MessageType, Result};

/// Represents an inhibit event.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InhibitEvent {
    event_type: EventType,
}

impl InhibitEvent {
    /// Creates a new [InhibitEvent].
    pub const fn new() -> Self {
        Self {
            event_type: EventType::new(),
        }
    }

    /// Creates a new [InhibitEvent] from the provided parameter.
    pub const fn create(event_type: EventType) -> Self {
        Self { event_type }
    }

    /// Gets the [MessageType] of the [InhibitEvent].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Event(self.event_type())
    }

    /// Gets the [EventType] of the [InhibitEvent].
    pub const fn event_type(&self) -> EventType {
        self.event_type
    }

    /// Sets the [EventType] of the [InhibitEvent].
    pub fn set_event_type(&mut self, event_type: EventType) {
        self.event_type = event_type;
    }

    /// Builder function that sets the [EventType] of the [InhibitEvent].
    pub fn with_event_type(mut self, event_type: EventType) -> Self {
        self.set_event_type(event_type);
        self
    }

    /// Gets the [MessageCode] of the [InhibitEvent].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Event(self.event_code())
    }

    /// Gets the [EventCode] of the [InhibitEvent].
    pub const fn event_code(&self) -> EventCode {
        EventCode::Inhibit
    }
}

impl From<&InhibitEvent> for MessageData {
    fn from(val: &InhibitEvent) -> Self {
        MessageData::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code())
    }
}

impl From<InhibitEvent> for MessageData {
    fn from(val: InhibitEvent) -> Self {
        (&val).into()
    }
}

impl From<&InhibitEvent> for Message {
    fn from(val: &InhibitEvent) -> Self {
        MessageData::from(val).into()
    }
}

impl From<InhibitEvent> for Message {
    fn from(val: InhibitEvent) -> Self {
        MessageData::from(val).into()
    }
}

impl TryFrom<&MessageData> for InhibitEvent {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        match val.message_code().event_code()? {
            EventCode::Inhibit => Ok(Self {
                event_type: val.message_type().event_type()?,
            }),
            code => Err(Error::InvalidEventCode(code.into())),
        }
    }
}

impl TryFrom<MessageData> for InhibitEvent {
    type Error = Error;

    fn try_from(val: MessageData) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&Message> for InhibitEvent {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for InhibitEvent {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        val.data().try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RequestCode, RequestType};

    #[test]
    fn test_inhibit_event() -> Result<()> {
        let exp_types = (0x80..=0x8fu8)
            .into_iter()
            .map(|e| MessageType::Event(EventType::from_u8(e)));
        let exp_code = MessageCode::Event(EventCode::Inhibit);

        for exp_type in exp_types {
            let msg: Message = MessageData::new()
                .with_message_type(exp_type)
                .with_message_code(exp_code)
                .into();

            let exp_req = InhibitEvent::create(exp_type.event_type()?);

            assert_eq!(exp_req.message_type(), exp_type);
            assert_eq!(exp_type.event_type(), Ok(exp_req.event_type()));

            assert_eq!(exp_req.message_code(), exp_code);
            assert_eq!(exp_code.event_code(), Ok(exp_req.event_code()));

            assert_eq!(Message::from(exp_req), msg);
            assert_eq!(InhibitEvent::try_from(&msg), Ok(exp_req));
        }

        Ok(())
    }

    #[test]
    fn test_inhibit_event_invalid() -> Result<()> {
        let invalid_types = [MessageType::Reserved]
            .into_iter()
            .chain(
                [
                    RequestType::Operation,
                    RequestType::Status,
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
                    .with_message_code(InhibitEvent::new().message_code());

                let inval_code = MessageData::new()
                    .with_message_type(InhibitEvent::new().message_type())
                    .with_message_code(msg_code);

                for data in [inval_data, inval_type, inval_code] {
                    assert!(InhibitEvent::try_from(&data).is_err());
                    assert!(InhibitEvent::try_from(Message::new().with_data(data)).is_err());
                }
            }
        }

        Ok(())
    }
}
