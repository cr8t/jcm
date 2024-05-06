use std::{fmt, mem};

use crate::{Error, Result};

mod event_type;
mod request_type;

pub use event_type::*;
pub use request_type::*;

const RESERVED: u8 = 0xff;

/// Represents the message type.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageType {
    /// Host-to-device request message.
    Request(RequestType),
    /// Device-to-host status event.
    Event(EventType),
    /// Reserved message type.
    Reserved = RESERVED,
}

impl MessageType {
    /// Creates a new [MessageType].
    pub const fn new() -> Self {
        Self::Request(RequestType::new())
    }

    /// Infallible conversion from a [`u8`] into a [RequestType].
    pub const fn from_u8(val: u8) -> Self {
        match (RequestType::from_u8(val), EventType::from_u8(val)) {
            (RequestType::Reserved, EventType::Reserved) => Self::Reserved,
            (req, EventType::Reserved) => Self::Request(req),
            (RequestType::Reserved, event) => Self::Event(event),
            // NOTE: technically `unreachable`, but let's make the compiler happy without `panic`ing
            (_, _) => Self::Reserved,
        }
    }

    /// Converts the [MessageType] to a [`u8`].
    pub const fn to_u8(&self) -> u8 {
        match self {
            Self::Request(c) => c.to_u8(),
            Self::Event(c) => c.to_u8(),
            Self::Reserved => RESERVED,
        }
    }

    /// Gets the length of the [MessageType].
    pub const fn len() -> usize {
        mem::size_of::<u8>()
    }

    /// Attempts to get the [MessageType] as a [RequestType].
    pub const fn request_type(&self) -> Result<RequestType> {
        match self {
            Self::Request(req) => Ok(*req),
            _ => Err(Error::InvalidRequestType(self.to_u8())),
        }
    }

    /// Attempts to get the [MessageType] as a [EventType].
    pub const fn event_type(&self) -> Result<EventType> {
        match self {
            Self::Event(evt) => Ok(*evt),
            _ => Err(Error::InvalidEventType(self.to_u8())),
        }
    }

    pub const fn is_request(&self) -> bool {
        matches!(self, Self::Request(_))
    }

    pub const fn is_event(&self) -> bool {
        matches!(self, Self::Event(_))
    }

    /// Gets whether the [MessageType] contains a reserved variant.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Reserved)
            || matches!(self, Self::Request(RequestType::Reserved))
            || matches!(self, Self::Event(EventType::Reserved))
    }
}

impl TryFrom<u8> for MessageType {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidMessageType(val)),
            v => Ok(v),
        }
    }
}

impl From<MessageType> for u8 {
    fn from(val: MessageType) -> Self {
        match val {
            MessageType::Request(ty) => ty as u8,
            MessageType::Event(ty) => ty as u8,
            _ => RESERVED,
        }
    }
}

impl From<&MessageType> for u8 {
    fn from(val: &MessageType) -> Self {
        (*val).into()
    }
}

impl Default for MessageType {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(mt) => write!(f, r#"{{"message_type": "request", "code": {mt}}}"#),
            Self::Event(mt) => write!(f, r#"{{"message_type": "event", "code": {mt}}}"#),
            Self::Reserved => write!(f, r#"{{"message_type": "reserved", "code": "reserved"}}"#),
        }
    }
}
