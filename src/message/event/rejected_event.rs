use crate::{Error, EventCode, EventType, Message, MessageCode, MessageData, MessageType, Result};

mod reject_code;

pub use reject_code::*;

/// Represents a reject event.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RejectedEvent {
    event_type: EventType,
    event_code: EventCode,
    reject_code: RejectCode,
}

impl RejectedEvent {
    /// Creates a new [RejectedEvent].
    pub const fn new() -> Self {
        Self {
            event_type: EventType::new(),
            event_code: EventCode::new(),
            reject_code: RejectCode::new(),
        }
    }

    /// Creates a new [RejectedEvent] from the provided parameters.
    pub const fn create(
        event_type: EventType,
        event_code: EventCode,
        reject_code: RejectCode,
    ) -> Self {
        Self {
            event_type,
            event_code,
            reject_code,
        }
    }

    /// Gets the [MessageType] of the [RejectedEvent].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Event(self.event_type())
    }

    /// Gets the [EventType] of the [RejectedEvent].
    pub const fn event_type(&self) -> EventType {
        self.event_type
    }

    /// Sets the [EventType] of the [RejectedEvent].
    pub fn set_event_type(&mut self, event_type: EventType) {
        self.event_type = event_type;
    }

    /// Builder function that sets the [EventType] of the [RejectedEvent].
    pub fn with_event_type(mut self, event_type: EventType) -> Self {
        self.set_event_type(event_type);
        self
    }

    /// Gets the [MessageCode] of the [RejectedEvent].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Event(self.event_code())
    }

    /// Gets the [EventCode] of the [RejectedEvent].
    pub const fn event_code(&self) -> EventCode {
        self.event_code
    }

    /// Sets the [EventCode] of the [RejectedEvent].
    ///
    /// `event_code` must be one of [EventCode::Rejected] or [EventCode::AcceptorRejected].
    pub fn set_event_code(&mut self, event_code: EventCode) -> Result<()> {
        match event_code {
            EventCode::Rejected | EventCode::AcceptorRejected => {
                self.event_code = event_code;
                Ok(())
            }
            _ => Err(Error::InvalidEventCode(event_code.into())),
        }
    }

    /// Builder function that sets the [EventCode] of the [RejectedEvent].
    ///
    /// `event_code` must be one of [EventCode::Rejected] or [EventCode::AcceptorRejected].
    pub fn with_event_code(mut self, event_code: EventCode) -> Result<Self> {
        self.set_event_code(event_code)?;
        Ok(self)
    }

    /// Gets the [RejectCode] of the [RejectedEvent].
    pub const fn reject_code(&self) -> RejectCode {
        self.reject_code
    }

    /// Sets the [RejectCode] of the [RejectedEvent].
    pub fn set_reject_code(&mut self, reject_code: RejectCode) {
        self.reject_code = reject_code;
    }

    /// Builder function that sets the [RejectCode] of the [RejectedEvent].
    pub fn with_reject_code(mut self, reject_code: RejectCode) -> Self {
        self.set_reject_code(reject_code);
        self
    }
}

impl From<&RejectedEvent> for MessageData {
    fn from(val: &RejectedEvent) -> Self {
        MessageData::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code())
            .with_additional(&[val.reject_code().into()])
    }
}

impl From<RejectedEvent> for MessageData {
    fn from(val: RejectedEvent) -> Self {
        (&val).into()
    }
}

impl From<&RejectedEvent> for Message {
    fn from(val: &RejectedEvent) -> Self {
        MessageData::from(val).into()
    }
}

impl From<RejectedEvent> for Message {
    fn from(val: RejectedEvent) -> Self {
        MessageData::from(val).into()
    }
}

impl TryFrom<&MessageData> for RejectedEvent {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        let event_code = val.message_code().event_code()?;
        let reject_code =
            RejectCode::try_from(val.additional().first().cloned().unwrap_or(0xffu8))?;

        match event_code {
            EventCode::Rejected | EventCode::AcceptorRejected => Ok(Self {
                event_type: val.message_type().event_type()?,
                event_code,
                reject_code,
            }),
            code => Err(Error::InvalidEventCode(code.into())),
        }
    }
}

impl TryFrom<MessageData> for RejectedEvent {
    type Error = Error;

    fn try_from(val: MessageData) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&Message> for RejectedEvent {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for RejectedEvent {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl Default for RejectedEvent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RequestCode, RequestType};

    #[test]
    fn test_rejected_event() -> Result<()> {
        let exp_types = (0x80..=0x8fu8).map(|e| MessageType::Event(EventType::from_u8(e)));
        let exp_codes = [EventCode::Rejected, EventCode::AcceptorRejected].map(MessageCode::Event);
        let reject_codes = [
            RejectCode::AbnormalInsertion,
            RejectCode::AbnormalSensor,
            RejectCode::ReturnedRemaining,
            RejectCode::AbnormalMagnification,
            RejectCode::Transportation,
            RejectCode::Inhibited,
            RejectCode::PhotoPattern1,
            RejectCode::PhotoLevel,
            RejectCode::InhibitBeforeEscrow,
            RejectCode::Return,
            RejectCode::TransportStacker,
            RejectCode::TransportFraud,
            RejectCode::NoteLength,
            RejectCode::PhotoPattern2,
            RejectCode::TrueBillFeature,
            RejectCode::ValidateBarcode,
            RejectCode::BarcodeDigits,
            RejectCode::BarcodeStartBit,
            RejectCode::BarcodeStopBit,
            RejectCode::DoubleTicket,
            RejectCode::TicketWrongSideUp,
            RejectCode::TicketLength,
        ];

        for exp_type in exp_types {
            for exp_code in exp_codes {
                for reject_code in reject_codes {
                    let msg: Message = MessageData::new()
                        .with_message_type(exp_type)
                        .with_message_code(exp_code)
                        .with_additional(&[reject_code.into()])
                        .into();

                    let exp_req = RejectedEvent::create(
                        exp_type.event_type()?,
                        exp_code.event_code()?,
                        reject_code,
                    );

                    assert_eq!(exp_req.message_type(), exp_type);
                    assert_eq!(exp_type.event_type(), Ok(exp_req.event_type()));

                    assert_eq!(exp_req.message_code(), exp_code);
                    assert_eq!(exp_code.event_code(), Ok(exp_req.event_code()));

                    assert_eq!(reject_code, exp_req.reject_code());

                    assert_eq!(Message::from(exp_req), msg);
                    assert_eq!(RejectedEvent::try_from(&msg), Ok(exp_req));
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_rejected_event_invalid() -> Result<()> {
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
                EventCode::Inhibit,
                EventCode::ProgramSignature,
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
                let val_reject = RejectCode::new();
                let inval_data = MessageData::new()
                    .with_message_type(msg_type)
                    .with_message_code(msg_code)
                    .with_additional(&[val_reject.into()]);

                let val_event = RejectedEvent::new();

                let inval_type = MessageData::new()
                    .with_message_type(msg_type)
                    .with_message_code(val_event.message_code())
                    .with_additional(&[val_reject.into()]);

                let inval_code = MessageData::new()
                    .with_message_type(val_event.message_type())
                    .with_message_code(msg_code)
                    .with_additional(&[val_reject.into()]);

                for data in [inval_data, inval_type, inval_code] {
                    assert!(RejectedEvent::try_from(&data).is_err());
                    assert!(RejectedEvent::try_from(Message::new().with_data(data)).is_err());
                }

                for inval_reject_code in (0..=255u8).filter(|&c| RejectCode::from_u8(c).is_empty())
                {
                    let data = MessageData::new()
                        .with_message_type(val_event.message_type())
                        .with_message_code(val_event.message_code())
                        .with_additional(&[inval_reject_code]);

                    assert!(RejectedEvent::try_from(&data).is_err());
                    assert!(RejectedEvent::try_from(Message::new().with_data(data)).is_err());
                }
            }
        }

        Ok(())
    }
}
