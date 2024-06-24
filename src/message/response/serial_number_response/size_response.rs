use std::fmt;

use crate::{Error, Message, Response, ResponseCode, Result, SerialNumberSize};

/// Represents the [Response] to a UID request [Message](crate::Message).
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SerialNumberSizeResponse {
    code: ResponseCode,
    size_total: SerialNumberSize,
}

impl SerialNumberSizeResponse {
    /// Gets the byte length of the [SerialNumberSizeResponse].
    pub const LEN: usize = ResponseCode::len() + SerialNumberSize::LEN;

    /// Creates a new [SerialNumberSizeResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            size_total: SerialNumberSize::new(),
        }
    }

    /// Gets the [ResponseCode] for the [SerialNumberSizeResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [SerialNumberSizeResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [SerialNumberSizeResponse].
    pub fn with_code(mut self, code: ResponseCode) -> Self {
        self.set_code(code);
        self
    }

    /// Gets the [SizeTotal](SerialNumberSize) for the [SerialNumberSizeResponse].
    pub const fn size_total(&self) -> &SerialNumberSize {
        &self.size_total
    }

    /// Sets the UID for the [SerialNumberSizeResponse].
    pub fn set_size_total(&mut self, val: SerialNumberSize) {
        self.size_total = val;
    }

    /// Builder function that sets the UID for the [SerialNumberSizeResponse].
    pub const fn with_size_total(self, val: SerialNumberSize) -> Self {
        Self {
            code: self.code,
            size_total: val,
        }
    }

    /// Gets whether the device supports serial number images.
    pub const fn is_supported(&self) -> bool {
        self.size_total.is_supported()
    }

    /// Gets the length of the [SerialNumberSizeResponse].
    pub const fn len(&self) -> usize {
        Self::LEN
    }

    /// Gets whether the [SerialNumberSizeResponse] is empty.
    pub const fn is_empty(&self) -> bool {
        self.code.is_empty() && self.size_total.is_empty()
    }

    /// Converts a [SerialNumberSizeResponse] into a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidResponseLen((buf_len, len)))
        } else {
            buf.iter_mut()
                .take(len)
                .zip([u8::from(self.code)].into_iter().chain(self.size_total))
                .for_each(|(dst, src)| *dst = src);

            Ok(())
        }
    }

    /// Converts a byte buffer into a [SerialNumberSizeResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        Ok(Self {
            code: buf
                .first()
                .copied()
                .ok_or(Error::InvalidResponseLen((buf.len(), Self::LEN)))?
                .try_into()?,
            size_total: buf
                .get(1..)
                .ok_or(Error::InvalidResponseLen((buf.len(), Self::LEN)))?
                .try_into()?,
        })
    }
}

impl Default for SerialNumberSizeResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&Response> for SerialNumberSizeResponse {
    type Error = Error;

    fn try_from(val: &Response) -> Result<Self> {
        Ok(Self {
            code: val.code,
            size_total: val.additional.as_slice().try_into()?,
        })
    }
}

impl TryFrom<Response> for SerialNumberSizeResponse {
    type Error = Error;

    fn try_from(val: Response) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&Message> for SerialNumberSizeResponse {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        Response::try_from(val)?.try_into()
    }
}

impl TryFrom<Message> for SerialNumberSizeResponse {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        Response::try_from(val)?.try_into()
    }
}

impl From<SerialNumberSizeResponse> for Response {
    fn from(val: SerialNumberSizeResponse) -> Self {
        Self {
            code: val.code,
            additional: val.size_total.into_bytes().into(),
        }
    }
}

impl From<&SerialNumberSizeResponse> for Response {
    fn from(val: &SerialNumberSizeResponse) -> Self {
        (*val).into()
    }
}

impl fmt::Display for SerialNumberSizeResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code":{}, "#, self.code)?;
        write!(f, r#""size_total":{}"#, self.size_total)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_number_size_total_response() {
        let raw = [ResponseCode::Ack as u8, 0, 0, 0, 0, 0];
        let exp = SerialNumberSizeResponse::new().with_code(ResponseCode::Ack);
        let exp_size_total = SerialNumberSize::new();
        let res = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(exp_size_total.into_bytes().as_ref());
        let mut out = [0u8; SerialNumberSizeResponse::LEN];

        assert_eq!(SerialNumberSizeResponse::from_bytes(raw.as_ref()), Ok(exp));
        assert_eq!(SerialNumberSizeResponse::try_from(&res), Ok(exp));
        assert_eq!(Response::from(&exp), res);

        assert!(exp.to_bytes(out.as_mut()).is_ok());

        assert_eq!(out, raw);
    }

    #[test]
    fn test_serial_number_size_total_response_invalid() {
        let raw = [ResponseCode::Ack as u8, 0];
        let exp = SerialNumberSizeResponse::new().with_code(ResponseCode::Ack);
        let mut out = [0u8; SerialNumberSizeResponse::LEN];

        assert!(SerialNumberSizeResponse::from_bytes(raw[..1].as_ref()).is_err());
        assert!(
            SerialNumberSizeResponse::from_bytes([ResponseCode::Reserved as u8, 0].as_ref())
                .is_err()
        );
        assert!(exp.to_bytes(out[..1].as_mut()).is_err());
        assert!(exp.to_bytes(&mut []).is_err());
        assert!(SerialNumberSizeResponse::try_from(Response::new().with_additional(&[])).is_err());
        assert!(SerialNumberSizeResponse::try_from(
            Response::new()
                .with_code(ResponseCode::Ack)
                .with_additional(&[])
        )
        .is_err());
    }
}
