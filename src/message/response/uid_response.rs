use std::{fmt, mem};

use crate::{Error, Response, ResponseCode, Result};

/// Represents the [Response] to a UID request [Message](crate::Message).
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UidResponse {
    code: ResponseCode,
    uid: u8,
}

impl UidResponse {
    /// Creates a new [UidResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            uid: 0,
        }
    }

    /// Gets the [ResponseCode] for the [UidResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [UidResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [UidResponse].
    pub fn with_code(mut self, code: ResponseCode) -> Self {
        self.set_code(code);
        self
    }

    /// Gets the UID for the [UidResponse].
    pub const fn uid(&self) -> u8 {
        self.uid
    }

    /// Sets the UID for the [UidResponse].
    pub fn set_uid(&mut self, uid: u8) {
        self.uid = uid;
    }

    /// Builder function that sets the UID for the [UidResponse].
    pub fn with_uid(mut self, uid: u8) -> Self {
        self.set_uid(uid);
        self
    }

    /// Gets the length of the [UidResponse].
    pub const fn len() -> usize {
        ResponseCode::len() + mem::size_of::<u8>()
    }

    /// Gets whether the [UidResponse] is empty.
    pub const fn is_empty(&self) -> bool {
        self.code.is_empty() && self.uid == 0
    }

    /// Converts a [UidResponse] into a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = Self::len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidResponseLen((buf_len, len)))
        } else {
            buf.iter_mut()
                .take(len)
                .zip([u8::from(self.code), self.uid])
                .for_each(|(dst, src)| *dst = src);

            Ok(())
        }
    }

    /// Converts a byte buffer into a [UidResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let len = Self::len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidResponseLen((buf_len, len)))
        } else {
            Ok(Self {
                code: buf[0].try_into()?,
                uid: buf[1],
            })
        }
    }
}

impl Default for UidResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&Response> for UidResponse {
    type Error = Error;

    fn try_from(val: &Response) -> Result<Self> {
        let len = Self::len();
        let res_len = val.len();

        if res_len < len {
            Err(Error::InvalidResponseLen((res_len, len)))
        } else {
            Ok(Self {
                code: val.code,
                uid: val.additional[0],
            })
        }
    }
}

impl TryFrom<Response> for UidResponse {
    type Error = Error;

    fn try_from(val: Response) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<UidResponse> for Response {
    fn from(val: UidResponse) -> Self {
        Self {
            code: val.code,
            additional: [val.uid].into(),
        }
    }
}

impl From<&UidResponse> for Response {
    fn from(val: &UidResponse) -> Self {
        (*val).into()
    }
}

impl fmt::Display for UidResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code":{}, "#, self.code)?;
        write!(f, r#""uid":{:#02x}"#, self.uid)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uid_response() {
        let raw = [ResponseCode::Ack as u8, 0];
        let exp = UidResponse::new().with_code(ResponseCode::Ack);
        let res = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(&[0]);
        let mut out = [0u8; 2];

        assert_eq!(UidResponse::from_bytes(raw.as_ref()), Ok(exp));
        assert_eq!(UidResponse::try_from(&res), Ok(exp));
        assert_eq!(Response::from(&exp), res);

        assert!(exp.to_bytes(out.as_mut()).is_ok());

        assert_eq!(out, raw);
    }

    #[test]
    fn test_uid_response_invalid() {
        let raw = [ResponseCode::Ack as u8, 0];
        let exp = UidResponse::new().with_code(ResponseCode::Ack);
        let mut out = [0u8; 2];

        assert!(UidResponse::from_bytes(raw[..1].as_ref()).is_err());
        assert!(UidResponse::from_bytes([ResponseCode::Reserved as u8, 0].as_ref()).is_err());
        assert!(exp.to_bytes(out[..1].as_mut()).is_err());
        assert!(exp.to_bytes(&mut []).is_err());
        assert!(UidResponse::try_from(Response::new().with_additional(&[])).is_err());
        assert!(UidResponse::try_from(
            Response::new()
                .with_code(ResponseCode::Ack)
                .with_additional(&[])
        )
        .is_err());
    }
}
