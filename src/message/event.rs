use std::fmt;

use crate::{Error, EventCode, EventType, Message, MessageCode, MessageData, MessageType, Result};

mod escrow_event;
mod inhibit_event;
mod rejected_event;

pub use escrow_event::*;
pub use inhibit_event::*;
pub use rejected_event::*;

/// Represents an event [Message] sent by the device.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    event_type: EventType,
    event_code: EventCode,
    additional: Vec<u8>,
}

impl Event {
    /// Creates a new [Event].
    pub const fn new() -> Self {
        Self {
            event_type: EventType::new(),
            event_code: EventCode::new(),
            additional: Vec::new(),
        }
    }

    /// Gets the [MessageType] of the [Event].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Event(self.event_type)
    }

    /// Gets the [EventType] of the [Event].
    pub const fn event_type(&self) -> EventType {
        self.event_type
    }

    /// Sets the [EventType] of the [Event].
    pub fn set_event_type(&mut self, event_type: EventType) {
        self.event_type = event_type;
    }

    /// Builder function that sets the [EventType] of the [Event].
    pub fn with_event_type(mut self, event_type: EventType) -> Self {
        self.set_event_type(event_type);
        self
    }

    /// Gets the [MessageCode] of the [Event].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Event(self.event_code)
    }

    /// Gets the [EventCode] of the [Event].
    pub const fn event_code(&self) -> EventCode {
        self.event_code
    }

    /// Sets the [EventCode] of the [Event].
    pub fn set_event_code(&mut self, code: EventCode) {
        self.event_code = code;
    }

    /// Builder function that sets the [EventCode] of the [Event].
    pub fn with_event_code(mut self, code: EventCode) -> Self {
        self.set_event_code(code);
        self
    }

    /// Gets a reference to the additional data of the [Event].
    pub fn additional(&self) -> &[u8] {
        &self.additional
    }

    /// Sets the additional data of the [Event].
    pub fn set_additional(&mut self, additional: &[u8]) {
        self.additional = additional.into();
    }

    /// Builder function that sets the additional data of the [Event].
    pub fn with_additional(mut self, additional: &[u8]) -> Self {
        self.set_additional(additional);
        self
    }

    /// Gets the length of the [Message].
    pub fn len(&self) -> usize {
        Self::meta_len() + self.additional.len()
    }

    pub(crate) const fn meta_len() -> usize {
        EventType::len() + EventCode::len()
    }

    /// Gets whether the [Event] is empty.
    pub const fn is_empty(&self) -> bool {
        self.event_type.is_empty() || self.event_code.is_empty()
    }

    /// Writes the [Message] to the provided byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidMessageLen((buf_len, len)))
        } else {
            let msg_iter = [self.event_type.to_u8()]
                .into_iter()
                .chain(self.event_code.to_bytes())
                .chain(self.additional.iter().cloned());

            buf.iter_mut()
                .take(len)
                .zip(msg_iter)
                .for_each(|(dst, src)| *dst = src);

            Ok(())
        }
    }
}

impl TryFrom<&[u8]> for Event {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        let meta_len = Self::meta_len();
        let len = val.len();

        match len {
            l if l < meta_len => Err(Error::InvalidEventLen((len, meta_len))),
            l if l == meta_len => Ok(Self {
                event_type: val[0].try_into()?,
                event_code: val[EventType::len()..].try_into()?,
                additional: Vec::new(),
            }),
            _ => Ok(Self {
                event_type: val[0].try_into()?,
                event_code: val[EventType::len()..].try_into()?,
                additional: val[Self::meta_len()..].into(),
            }),
        }
    }
}

impl TryFrom<&Message> for Event {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        Ok(Self {
            event_type: val.data().message_type().event_type()?,
            event_code: val.data().message_code().event_code()?,
            additional: val.data().additional().into(),
        })
    }
}

impl TryFrom<Message> for Event {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&Event> for Message {
    fn from(val: &Event) -> Self {
        Self::new().with_data(
            MessageData::new()
                .with_message_type(val.message_type())
                .with_message_code(val.message_code())
                .with_additional(val.additional()),
        )
    }
}

impl From<Event> for Message {
    fn from(val: Event) -> Self {
        (&val).into()
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""event_type":{}, "#, self.event_type)?;
        write!(f, r#""event_code":{}, "#, self.event_code)?;
        write!(f, r#""additional_data": ["#)?;

        for (i, d) in self.additional.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{d}")?;
        }

        write!(f, "]}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event() {
        let type_bytes = EventType::Sequence0.to_u8();
        let code_bytes = EventCode::PowerUp.to_bytes();

        let raw = [type_bytes, code_bytes[0], code_bytes[1]];

        let msg = Message::new().with_data(
            MessageData::new()
                .with_message_type(MessageType::Event(EventType::Sequence0))
                .with_message_code(MessageCode::Event(EventCode::PowerUp)),
        );

        let exp = Event::new()
            .with_event_type(EventType::Sequence0)
            .with_event_code(EventCode::PowerUp);

        assert_eq!(Event::try_from(raw.as_ref()), Ok(exp.clone()));
        assert_eq!(Event::try_from(&msg), Ok(exp.clone()));
        assert_eq!(Event::try_from(msg), Ok(exp.clone()));

        let mut out = [0u8; Event::meta_len()];
        assert_eq!(exp.to_bytes(out.as_mut()), Ok(()));
        assert_eq!(out, raw);
    }

    #[test]
    fn test_event_with_data() {
        let type_bytes = EventType::Sequence0.to_u8();
        let code_bytes = EventCode::Escrow.to_bytes();

        let raw = [
            type_bytes,
            code_bytes[0],
            code_bytes[1],
            b'U',
            b'S',
            b'D',
            0x64,
            0x00,
        ];

        let msg = Message::new().with_data(
            MessageData::new()
                .with_message_type(MessageType::Event(EventType::Sequence0))
                .with_message_code(MessageCode::Event(EventCode::Escrow))
                .with_additional(raw[Event::meta_len()..].as_ref()),
        );

        let exp = Event::new()
            .with_event_type(EventType::Sequence0)
            .with_event_code(EventCode::Escrow)
            .with_additional(raw[Event::meta_len()..].as_ref());

        assert_eq!(Event::try_from(raw.as_ref()), Ok(exp.clone()));
        assert_eq!(Event::try_from(&msg), Ok(exp.clone()));
        assert_eq!(Event::try_from(msg), Ok(exp.clone()));

        let mut out = [0u8; 8];
        assert_eq!(exp.to_bytes(out.as_mut()), Ok(()));
        assert_eq!(out, raw);
    }
}
