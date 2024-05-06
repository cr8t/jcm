use crate::{
    Currency, Error, EventCode, EventType, Message, MessageCode, MessageData, MessageType, Result,
    Ticket,
};

mod escrow_data;

pub use escrow_data::*;

/// Represents an inhibit event.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct EscrowEvent {
    event_type: EventType,
    data: EscrowData,
}

impl EscrowEvent {
    /// Creates a new [EscrowEvent].
    pub const fn new() -> Self {
        Self {
            event_type: EventType::new(),
            data: EscrowData::new(),
        }
    }

    /// Creates a new [EscrowEvent] from the provided parameters.
    pub const fn create(event_type: EventType, data: EscrowData) -> Self {
        Self { event_type, data }
    }

    /// Gets the [MessageType] of the [EscrowEvent].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Event(self.event_type())
    }

    /// Gets the [EventType] of the [EscrowEvent].
    pub const fn event_type(&self) -> EventType {
        self.event_type
    }

    /// Sets the [EventType] of the [EscrowEvent].
    pub fn set_event_type(&mut self, event_type: EventType) {
        self.event_type = event_type;
    }

    /// Builder function that sets the [EventType] of the [EscrowEvent].
    pub fn with_event_type(mut self, event_type: EventType) -> Self {
        self.set_event_type(event_type);
        self
    }

    /// Gets the [MessageCode] of the [EscrowEvent].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Event(self.event_code())
    }

    /// Gets the [EventCode] of the [EscrowEvent].
    pub const fn event_code(&self) -> EventCode {
        EventCode::Escrow
    }

    /// Gets a reference to the [EscrowData] of the [EscrowEvent].
    pub const fn data(&self) -> &EscrowData {
        &self.data
    }

    /// Gets a reference to the [Currency] of the [EscrowEvent].
    pub const fn currency(&self) -> Result<&Currency> {
        match self.data() {
            EscrowData::Currency(data) => Ok(data),
            _ => Err(Error::InvalidEscrowData),
        }
    }

    /// Gets a reference to the [Ticket] of the [EscrowEvent].
    pub const fn ticket(&self) -> Result<&Ticket> {
        match self.data() {
            EscrowData::Ticket(data) => Ok(data),
            _ => Err(Error::InvalidEscrowData),
        }
    }
}

impl From<&EscrowEvent> for MessageData {
    fn from(val: &EscrowEvent) -> Self {
        MessageData::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code())
            .with_additional(val.data.to_vec().as_ref())
    }
}

impl From<EscrowEvent> for MessageData {
    fn from(val: EscrowEvent) -> Self {
        (&val).into()
    }
}

impl From<&EscrowEvent> for Message {
    fn from(val: &EscrowEvent) -> Self {
        MessageData::from(val).into()
    }
}

impl From<EscrowEvent> for Message {
    fn from(val: EscrowEvent) -> Self {
        MessageData::from(val).into()
    }
}

impl TryFrom<&MessageData> for EscrowEvent {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        match val.message_code().event_code()? {
            EventCode::Escrow => Ok(Self {
                event_type: val.message_type().event_type()?,
                data: val.additional().try_into()?,
            }),
            code => Err(Error::InvalidEventCode(code.into())),
        }
    }
}

impl TryFrom<MessageData> for EscrowEvent {
    type Error = Error;

    fn try_from(val: MessageData) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&Message> for EscrowEvent {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for EscrowEvent {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl Default for EscrowEvent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RequestCode, RequestType};

    #[test]
    fn test_escrow_event() -> Result<()> {
        let exp_types = (0x80..=0x8fu8).map(|e| MessageType::Event(EventType::from_u8(e)));

        let exp_code = MessageCode::Event(EventCode::Escrow);
        let exp_data = EscrowData::new();

        let mut exp_data_buf = vec![0u8; exp_data.len()];
        exp_data.to_bytes(exp_data_buf.as_mut())?;

        for exp_type in exp_types {
            let msg: Message = MessageData::new()
                .with_message_type(exp_type)
                .with_message_code(exp_code)
                .with_additional(exp_data_buf.as_ref())
                .into();

            let exp_req = EscrowEvent::create(exp_type.event_type()?, exp_data.clone());

            assert_eq!(exp_req.message_type(), exp_type);
            assert_eq!(exp_type.event_type(), Ok(exp_req.event_type()));

            assert_eq!(exp_req.message_code(), exp_code);
            assert_eq!(exp_code.event_code(), Ok(exp_req.event_code()));

            assert_eq!(Message::from(&exp_req), msg);
            assert_eq!(EscrowEvent::try_from(&msg), Ok(exp_req));
        }

        Ok(())
    }

    #[test]
    fn test_escrow_event_invalid() -> Result<()> {
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
                EventCode::Inhibit,
                EventCode::Idle,
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

                let val_event = EscrowEvent::new();

                let inval_type = MessageData::new()
                    .with_message_type(msg_type)
                    .with_message_code(val_event.message_code());

                let inval_code = MessageData::new()
                    .with_message_type(val_event.message_type())
                    .with_message_code(msg_code);

                let inval_currency = MessageData::new()
                    .with_message_type(val_event.message_type())
                    .with_message_code(val_event.message_code())
                    .with_additional(&[b'X', b'X', b'X', 0, 1, 0, 0]);

                let inval_denom = MessageData::new()
                    .with_message_type(val_event.message_type())
                    .with_message_code(val_event.message_code())
                    .with_additional(&[b'J', b'P', b'Y', 0xff, 0xff, 0xff, 0xff]);

                let inval_ticket_long = MessageData::new()
                    .with_message_type(val_event.message_type())
                    .with_message_code(val_event.message_code())
                    .with_additional(&[0u8, 0u8, 0xff /*buffer shorter than ticket length*/]);

                let inval_ticket_code = MessageData::new()
                    .with_message_type(val_event.message_type())
                    .with_message_code(val_event.message_code())
                    .with_additional(&[
                        0u8, 0u8, 9u8, 0xff, 0xb, 0xa, 0xd, b'A', b'S', b'C', b'I', b'I',
                    ]);

                for data in [
                    inval_data,
                    inval_type,
                    inval_code,
                    inval_currency,
                    inval_denom,
                    inval_ticket_long,
                    inval_ticket_code,
                ] {
                    assert!(EscrowEvent::try_from(&data).is_err());
                    assert!(EscrowEvent::try_from(Message::new().with_data(data)).is_err());
                }
            }
        }

        Ok(())
    }
}
