use std::fmt;

use crate::{
    ConfId, Error, Message, MessageCode, MessageData, MessageType, RequestCode, RequestType,
    Result, UnitNumber,
};

mod stack_status_change;

pub use stack_status_change::*;

/// Represents the additional data in a stack request.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StackRequestData {
    stack_box: Option<UnitNumber>,
    status_change: Option<StackStatusChange>,
}

impl StackRequestData {
    /// Creates a new [StackRequestData].
    pub const fn new() -> Self {
        Self {
            stack_box: None,
            status_change: None,
        }
    }

    /// Gets the length of the [StackRequestData].
    pub const fn len(&self) -> usize {
        match (self.stack_box, self.status_change) {
            (Some(_sb), None) => UnitNumber::len(),
            (None, Some(_sc)) => 0,
            (Some(_sb), Some(_sc)) => UnitNumber::len() + StackStatusChange::len(),
            _ => 0,
        }
    }

    /// Gets whether the [StackRequestData] is empty.
    pub const fn is_empty(&self) -> bool {
        self.stack_box.is_none() && self.status_change.is_none()
    }

    /// Gets the recycler box [UnitNumber] used to stack notes.
    pub const fn stack_box(&self) -> Option<UnitNumber> {
        self.stack_box
    }

    /// Sets the recycler box [UnitNumber] used to stack notes.
    pub fn set_stack_box(&mut self, stack_box: UnitNumber) {
        self.stack_box.replace(stack_box);
    }

    /// Builder function that sets the recycler box [UnitNumber] used to stack notes.
    pub fn with_stack_box(mut self, stack_box: UnitNumber) -> Self {
        self.set_stack_box(stack_box);
        self
    }

    /// Unsets the recycler box [UnitNumber] used to stack notes (use the default box).
    pub fn unset_stack_box(&mut self) -> Option<UnitNumber> {
        self.stack_box.take()
    }

    /// Gets the device [StackStatusChange] after a collection operation.
    pub const fn status_change(&self) -> Option<StackStatusChange> {
        self.status_change
    }

    /// Sets the device [StackStatusChange] after a collection operation.
    pub fn set_status_change(&mut self, status_change: StackStatusChange) {
        self.status_change.replace(status_change);
    }

    /// Builder function that sets the device [StackStatusChange] after a collection operation.
    pub fn with_status_change(mut self, status_change: StackStatusChange) -> Self {
        self.set_status_change(status_change);
        self
    }

    /// Unsets the device [StackStatusChange] after a collection operation (default status change).
    pub fn unset_status_change(&mut self) -> Option<StackStatusChange> {
        self.status_change.take()
    }

    /// Converts a byte buffer into a [StackRequestData].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        match buf.len() {
            0 => Ok(Self::new()),
            1 => Ok(Self {
                stack_box: Some(UnitNumber::from_u8(buf[0])),
                status_change: None,
            }),
            _ => Ok(Self {
                stack_box: Some(UnitNumber::from_u8(buf[0])),
                status_change: Some(buf[1].try_into()?),
            }),
        }
    }

    /// Writes the [StackRequestData] to a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidStackRequestDataLen((buf_len, len)))
        } else {
            match (self.stack_box, self.status_change) {
                (Some(sb), None) if sb.is_valid() => buf[0] = sb.to_u8(),
                (Some(sb), None) if sb.is_empty() => buf[0] = 0,
                (Some(sb), Some(sc)) if sb.is_valid() && sc.is_valid() => {
                    buf[..=1].copy_from_slice(&[sb.to_u8(), sc.to_u8()])
                }
                (Some(sb), Some(sc)) if sb.is_empty() && sc.is_valid() => {
                    buf[..=1].copy_from_slice(&[0, sc.to_u8()])
                }
                _ => (),
            }

            Ok(())
        }
    }

    /// Converts the [StackRequestData] to a byte vector.
    pub fn as_bytes(&self) -> Vec<u8> {
        let len = self.len();
        match len {
            0 => Vec::new(),
            _ => {
                let mut out = vec![0u8; len];
                self.to_bytes(&mut out).ok();
                out
            }
        }
    }

    /// Converts the [StackRequestData] to a byte vector.
    pub fn into_bytes(self) -> Vec<u8> {
        self.as_bytes()
    }
}

impl From<StackRequestData> for MessageData {
    fn from(val: StackRequestData) -> Self {
        Self::new()
            .with_conf_id(ConfId::Acceptor)
            .with_message_type(MessageType::Request(RequestType::Operation))
            .with_message_code(MessageCode::Request(RequestCode::Stack))
            .with_additional(val.into_bytes().as_ref())
    }
}

impl From<&StackRequestData> for MessageData {
    fn from(val: &StackRequestData) -> Self {
        Self::new()
            .with_conf_id(ConfId::Acceptor)
            .with_message_type(MessageType::Request(RequestType::Operation))
            .with_message_code(MessageCode::Request(RequestCode::Stack))
            .with_additional(val.as_bytes().as_ref())
    }
}

impl From<StackRequestData> for Message {
    fn from(val: StackRequestData) -> Self {
        Self::new().with_data(val.into())
    }
}

impl From<&StackRequestData> for Message {
    fn from(val: &StackRequestData) -> Self {
        Self::new().with_data(val.into())
    }
}

impl TryFrom<&MessageData> for StackRequestData {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        if val.conf_id().is_empty() {
            Err(Error::InvalidConfId(val.conf_id().into()))
        } else if !matches!(
            val.message_type(),
            MessageType::Request(RequestType::Operation)
        ) {
            Err(Error::InvalidMessageType(val.message_type().into()))
        } else if !matches!(val.message_code(), MessageCode::Request(RequestCode::Stack)) {
            Err(Error::InvalidMessageCode((
                val.message_code().into(),
                MessageCode::Request(RequestCode::Stack).into(),
            )))
        } else {
            Self::from_bytes(val.additional())
        }
    }
}

impl TryFrom<MessageData> for StackRequestData {
    type Error = Error;

    fn try_from(val: MessageData) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&Message> for StackRequestData {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for StackRequestData {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for StackRequestData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        match (self.stack_box.as_ref(), self.status_change.as_ref()) {
            (Some(sb), Some(sc)) => {
                write!(f, r#""stack_box": {sb}, "#)?;
                write!(f, r#""status_change": {sc}"#)?;
            }
            (Some(sb), None) => {
                write!(f, r#""stack_box": {sb}"#)?;
            }
            (None, Some(sc)) => {
                write!(f, r#""status_change": {sc}"#)?;
            }
            _ => (),
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EventCode, EventType, RequestCode, RequestType};

    #[test]
    fn test_stack_request_data() -> Result<()> {
        let msg_data = MessageData::new()
            .with_conf_id(ConfId::Acceptor)
            .with_message_type(MessageType::Request(RequestType::Operation))
            .with_message_code(MessageCode::Request(RequestCode::Stack));

        for stat_change in [
            StackStatusChange::Idle.to_u8(),
            StackStatusChange::Inhibit.to_u8(),
        ] {
            for stack_box in 0x0..=0xf {
                let stack_data = msg_data.clone().with_additional(&[stack_box, stat_change]);
                let stack_req = StackRequestData::try_from(stack_data)?;

                assert_eq!(stack_req.stack_box(), Some(UnitNumber::from_u8(stack_box)));
                assert_eq!(
                    stack_req.status_change(),
                    Some(StackStatusChange::from_u8(stat_change))
                );
            }
        }

        let stack_req = StackRequestData::try_from(msg_data)?;

        assert!(stack_req.stack_box().is_none());
        assert!(stack_req.status_change().is_none());

        Ok(())
    }

    #[test]
    fn test_stack_request_data_invalid() -> Result<()> {
        let msg_data = MessageData::new()
            .with_conf_id(ConfId::Acceptor)
            .with_message_type(MessageType::Request(RequestType::Operation))
            .with_message_code(MessageCode::Request(RequestCode::Stack));

        for stat_change in 0x2..=0xff {
            for stack_box in 0x0..=0xff {
                let stack_data = msg_data.clone().with_additional(&[stack_box, stat_change]);
                assert!(StackRequestData::try_from(stack_data).is_err());
            }
        }

        for msg_type in [MessageType::Reserved]
            .into_iter()
            .chain((0x80..=0x8f).map(|m| MessageType::Event(EventType::from_u8(m))))
            .chain(
                [
                    RequestType::Status,
                    RequestType::SetFeature,
                    RequestType::Reserved,
                ]
                .map(MessageType::Request),
            )
        {
            let stack_data = msg_data.clone().with_message_type(msg_type);
            assert!(StackRequestData::try_from(stack_data).is_err());
        }

        for msg_code in [
            RequestCode::Uid,
            RequestCode::ProgramSignature,
            RequestCode::Version,
            RequestCode::SerialNumber,
            RequestCode::ModelName,
            RequestCode::Status,
            RequestCode::Reset,
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
        ) {
            let stack_data = msg_data.clone().with_message_code(msg_code);
            assert!(StackRequestData::try_from(stack_data).is_err());
        }

        Ok(())
    }
}
