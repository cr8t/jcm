use std::fmt;

use crate::{Error, Message, Result};

mod response_code;

pub use response_code::*;

/// Represents the generic response format for JCM host-device communication.
///
/// Response data is encoded in the [Message](crate::Message) format as additional data in
/// [MessageData](crate::MessageData).
///
/// Response format:
///
/// Field name  | Response Code | Data
/// ------------|---------------|---------
/// Size (byte) | 1             | Variable
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    code: ResponseCode,
    additional: Vec<u8>,
}

impl Response {
    /// Creates a new [Response].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            additional: Vec::new(),
        }
    }

    /// Gets the [ResponseCode] of the [Response].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] of the [Response].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] of the [Response].
    pub fn with_code(mut self, code: ResponseCode) -> Self {
        self.set_code(code);
        self
    }

    /// Gets a reference to the additional data of the [Response].
    pub fn additional(&self) -> &[u8] {
        &self.additional
    }

    /// Sets the additional data of the [Response].
    pub fn set_additional(&mut self, additional: &[u8]) {
        self.additional = additional.into();
    }

    /// Builder function that sets the additional data of the [Response].
    pub fn with_additional(mut self, additional: &[u8]) -> Self {
        self.set_additional(additional);
        self
    }

    /// Gets the length of the [Message].
    pub fn len(&self) -> usize {
        Self::meta_len() + self.additional.len()
    }

    pub(crate) const fn meta_len() -> usize {
        ResponseCode::len()
    }

    /// Gets whether the [Response] is empty.
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
            let msg_iter = [self.code.into()]
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

impl TryFrom<&[u8]> for Response {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        let meta_len = Self::meta_len();
        let len = val.len();

        match len {
            l if l < meta_len => Err(Error::InvalidResponseLen((len, meta_len))),
            l if l == meta_len => Ok(Self {
                code: val[0].try_into()?,
                additional: Vec::new(),
            }),
            _ => Ok(Self {
                code: val[0].try_into()?,
                additional: val[1..].into(),
            }),
        }
    }
}

impl TryFrom<&Message> for Response {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().additional().try_into()
    }
}

impl TryFrom<Message> for Response {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl fmt::Display for Response {
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
    fn test_response() {
        let raw = [u8::from(ResponseCode::Ack)];
        let msg = Message::new().with_data(MessageData::new().with_additional(&raw));
        let exp = Response::new().with_code(ResponseCode::Ack);

        assert_eq!(Response::try_from(raw.as_ref()), Ok(exp.clone()));
        assert_eq!(Response::try_from(&msg), Ok(exp.clone()));
        assert_eq!(Response::try_from(msg), Ok(exp.clone()));

        let mut out = [0u8];
        assert_eq!(exp.to_bytes(out.as_mut()), Ok(()));
        assert_eq!(out, raw);
    }

    #[test]
    fn test_response_with_data() {
        let raw = [u8::from(ResponseCode::Ack), 0x02, 0x01];
        let msg = Message::new().with_data(MessageData::new().with_additional(&raw));
        let exp = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(&raw[1..]);

        assert_eq!(Response::try_from(raw.as_ref()), Ok(exp.clone()));
        assert_eq!(Response::try_from(&msg), Ok(exp.clone()));
        assert_eq!(Response::try_from(msg), Ok(exp.clone()));

        let mut out = [0u8; 3];
        assert_eq!(exp.to_bytes(out.as_mut()), Ok(()));
        assert_eq!(out, raw);
    }
}
