use std::fmt;

use crate::{Error, EventCode, Message, Result};

/// Represents an event [Message] sent by the device.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    code: EventCode,
    additional: Vec<u8>,
}

impl Event {
    /// Creates a new [Event].
    pub const fn new() -> Self {
        Self {
            code: EventCode::new(),
            additional: Vec::new(),
        }
    }

    /// Gets the [EventCode] of the [Event].
    pub const fn code(&self) -> EventCode {
        self.code
    }

    /// Sets the [EventCode] of the [Event].
    pub fn set_code(&mut self, code: EventCode) {
        self.code = code;
    }

    /// Builder function that sets the [EventCode] of the [Event].
    pub fn with_code(mut self, code: EventCode) -> Self {
        self.set_code(code);
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
        EventCode::len()
    }

    /// Gets whether the [Event] is empty.
    pub const fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    /// Writes the [Message] to the provided byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidMessageLen((buf_len, len)))
        } else {
            let msg_iter = self
                .code
                .to_bytes()
                .into_iter()
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
                code: val.try_into()?,
                additional: Vec::new(),
            }),
            _ => Ok(Self {
                code: val[..=1].try_into()?,
                additional: val[2..].into(),
            }),
        }
    }
}

impl TryFrom<&Message> for Event {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().additional().try_into()
    }
}

impl TryFrom<Message> for Event {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code":{}, "#, self.code)?;
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
    use crate::MessageData;

    #[test]
    fn test_event() {
        let raw = EventCode::PowerUp.to_bytes();
        let msg = Message::new().with_data(MessageData::new().with_additional(&raw));
        let exp = Event::new().with_code(EventCode::PowerUp);

        assert_eq!(Event::try_from(raw.as_ref()), Ok(exp.clone()));
        assert_eq!(Event::try_from(&msg), Ok(exp.clone()));
        assert_eq!(Event::try_from(msg), Ok(exp.clone()));

        let mut out = [0u8, 0u8];
        assert_eq!(exp.to_bytes(out.as_mut()), Ok(()));
        assert_eq!(out, raw);
    }

    #[test]
    fn test_event_with_data() {
        let event_bytes = EventCode::Escrow.to_bytes();
        let raw = [event_bytes[0], event_bytes[1], b'U', b'S', b'D', 0x64, 0x00];
        let msg = Message::new().with_data(MessageData::new().with_additional(&raw));
        let exp = Event::new()
            .with_code(EventCode::Escrow)
            .with_additional(&raw[2..]);

        assert_eq!(Event::try_from(raw.as_ref()), Ok(exp.clone()));
        assert_eq!(Event::try_from(&msg), Ok(exp.clone()));
        assert_eq!(Event::try_from(msg), Ok(exp.clone()));

        let mut out = [0u8; 7];
        assert_eq!(exp.to_bytes(out.as_mut()), Ok(()));
        assert_eq!(out, raw);
    }
}
