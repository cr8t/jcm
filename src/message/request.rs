//! Contains types for additional data in request messages.

use std::fmt;

use crate::{
    Error, Message, MessageCode, MessageData, MessageType, RequestCode, RequestType, Result,
};

mod idle_request;
mod inhibit_request;
mod request_mode;
mod reset_request;
mod stack_request;
mod status_request;
mod uid_request;

pub use idle_request::*;
pub use inhibit_request::*;
pub use request_mode::*;
pub use reset_request::*;
pub use stack_request::*;
pub use status_request::*;
pub use uid_request::*;

/// Represents an event [Message] sent by the device.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    request_type: RequestType,
    request_code: RequestCode,
    additional: Vec<u8>,
}

impl Request {
    /// Creates a new [Request].
    pub const fn new() -> Self {
        Self {
            request_type: RequestType::new(),
            request_code: RequestCode::new(),
            additional: Vec::new(),
        }
    }

    /// Gets the [MessageType] of the [Request].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Request(self.request_type)
    }

    /// Gets the [RequestType] of the [Request].
    pub const fn request_type(&self) -> RequestType {
        self.request_type
    }

    /// Sets the [RequestType] of the [Request].
    pub fn set_request_type(&mut self, val: RequestType) {
        self.request_type = val;
    }

    /// Builder function that sets the [RequestType] of the [Request].
    pub fn with_request_type(mut self, val: RequestType) -> Self {
        self.set_request_type(val);
        self
    }

    /// Gets the [MessageCode] of the [Request].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Request(self.request_code)
    }

    /// Gets the [RequestCode] of the [Request].
    pub const fn request_code(&self) -> RequestCode {
        self.request_code
    }

    /// Sets the [RequestCode] of the [Request].
    pub fn set_request_code(&mut self, code: RequestCode) {
        self.request_code = code;
    }

    /// Builder function that sets the [RequestCode] of the [Request].
    pub fn with_request_code(mut self, code: RequestCode) -> Self {
        self.set_request_code(code);
        self
    }

    /// Gets a reference to the additional data of the [Request].
    pub fn additional(&self) -> &[u8] {
        &self.additional
    }

    /// Sets the additional data of the [Request].
    pub fn set_additional(&mut self, additional: &[u8]) {
        self.additional = additional.into();
    }

    /// Builder function that sets the additional data of the [Request].
    pub fn with_additional(mut self, additional: &[u8]) -> Self {
        self.set_additional(additional);
        self
    }

    /// Gets the length of the [Message].
    pub fn len(&self) -> usize {
        Self::meta_len() + self.additional.len()
    }

    pub(crate) const fn meta_len() -> usize {
        RequestType::len() + RequestCode::len()
    }

    /// Gets whether the [Request] is empty.
    pub const fn is_empty(&self) -> bool {
        self.request_code.is_empty()
    }

    /// Writes the [Message] to the provided byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidMessageLen((buf_len, len)))
        } else {
            let msg_iter = [self.request_type.to_u8()]
                .into_iter()
                .chain(self.request_code.to_bytes())
                .chain(self.additional.iter().cloned());

            buf.iter_mut()
                .take(len)
                .zip(msg_iter)
                .for_each(|(dst, src)| *dst = src);

            Ok(())
        }
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        let meta_len = Self::meta_len();
        let len = val.len();

        match len {
            l if l < meta_len => Err(Error::InvalidRequestLen((len, meta_len))),
            l if l == meta_len => Ok(Self {
                request_type: val[0].try_into()?,
                request_code: val[RequestType::len()..].try_into()?,
                additional: Vec::new(),
            }),
            _ => Ok(Self {
                request_type: val[0].try_into()?,
                request_code: val[RequestType::len()..].try_into()?,
                additional: val[Self::meta_len()..].into(),
            }),
        }
    }
}

impl TryFrom<&Message> for Request {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        Ok(Self {
            request_type: val.data().message_type().request_type()?,
            request_code: val.data().message_code().request_code()?,
            additional: val.data().additional().into(),
        })
    }
}

impl TryFrom<Message> for Request {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&Request> for Message {
    fn from(val: &Request) -> Self {
        Self::new().with_data(
            MessageData::new()
                .with_message_type(val.message_type())
                .with_message_code(val.message_code())
                .with_additional(val.additional()),
        )
    }
}

impl From<Request> for Message {
    fn from(val: Request) -> Self {
        (&val).into()
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""request_type":{}, "#, self.request_type)?;
        write!(f, r#""request_code":{}, "#, self.request_code)?;
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
    fn test_request() {
        let type_bytes = RequestType::Operation.to_u8();
        let code_bytes = RequestCode::Stack.to_bytes();

        let raw = [type_bytes, code_bytes[0], code_bytes[1]];

        let msg = Message::new().with_data(
            MessageData::new()
                .with_message_type(MessageType::Request(RequestType::Operation))
                .with_message_code(MessageCode::Request(RequestCode::Stack)),
        );

        let exp = Request::new()
            .with_request_type(RequestType::Operation)
            .with_request_code(RequestCode::Stack);

        assert_eq!(Request::try_from(raw.as_ref()), Ok(exp.clone()));
        assert_eq!(Request::try_from(&msg), Ok(exp.clone()));
        assert_eq!(Request::try_from(msg), Ok(exp.clone()));

        let mut out = [0u8; Request::meta_len()];
        assert_eq!(exp.to_bytes(out.as_mut()), Ok(()));
        assert_eq!(out, raw);
    }

    #[test]
    fn test_request_with_data() {
        let type_bytes = RequestType::SetFeature.to_u8();
        let code_bytes = RequestCode::Uid.to_bytes();
        let raw = [type_bytes, code_bytes[0], code_bytes[1], 0x01];

        let msg = Message::new().with_data(
            MessageData::new()
                .with_message_type(MessageType::Request(RequestType::SetFeature))
                .with_message_code(MessageCode::Request(RequestCode::Uid))
                .with_additional(raw[Request::meta_len()..].as_ref()),
        );

        let exp = Request::new()
            .with_request_type(RequestType::SetFeature)
            .with_request_code(RequestCode::Uid)
            .with_additional(raw[Request::meta_len()..].as_ref());

        assert_eq!(Request::try_from(raw.as_ref()), Ok(exp.clone()));
        assert_eq!(Request::try_from(&msg), Ok(exp.clone()));
        assert_eq!(Request::try_from(msg), Ok(exp.clone()));

        let mut out = [0u8; 4];
        assert_eq!(exp.to_bytes(out.as_mut()), Ok(()));
        assert_eq!(out, raw);
    }
}
