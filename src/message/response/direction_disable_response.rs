use std::fmt;

use crate::{Error, InhibitDirection, Message, RequestCode, Response, ResponseCode, Result};

/// Represents the [Response] to a [DirectionDisableRequest](crate::DirectionDisableRequest).
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectionDisableResponse {
    code: ResponseCode,
    dirs: InhibitDirection,
}

impl DirectionDisableResponse {
    /// Creates a new [DirectionDisableResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            dirs: InhibitDirection::new(),
        }
    }

    /// Gets the [ResponseCode] for the [DirectionDisableResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [DirectionDisableResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [DirectionDisableResponse].
    pub fn with_code(mut self, code: ResponseCode) -> Self {
        self.set_code(code);
        self
    }

    /// Gets the [InhibitDirection] for the [InhibitDirectionResponse].
    pub fn directions(&self) -> InhibitDirection {
        self.dirs
    }

    /// Sets the [InhibitDirection] for the [InhibitDirectionResponse].
    pub fn set_directions(&mut self, dirs: InhibitDirection) {
        self.dirs = dirs;
    }

    /// Builder function that sets the [InhibitDirection] for the [InhibitDirectionResponse].
    pub fn with_directions(mut self, dirs: InhibitDirection) -> Self {
        self.set_directions(dirs);
        self
    }

    /// Gets the length of the [DirectionDisableResponse].
    pub const fn len() -> usize {
        ResponseCode::len() + InhibitDirection::len()
    }

    /// Gets whether the [DirectionDisableResponse] is empty.
    pub fn is_empty(&self) -> bool {
        self.code.is_empty() && self.dirs.is_empty()
    }

    /// Gets an iterator over [DirectionDisableResponse] bytes.
    pub fn iter_bytes(&self) -> impl Iterator<Item = u8> + '_ {
        [self.code.into(), self.dirs.into()].into_iter()
    }

    /// Gets an iterator over [DirectionDisableResponse] bytes.
    pub fn into_iter_bytes(self) -> impl Iterator<Item = u8> {
        [self.code.into(), self.dirs.into()].into_iter()
    }

    /// Converts a [DirectionDisableResponse] into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.iter_bytes().collect()
    }

    /// Converts a [DirectionDisableResponse] into a byte vector.
    pub fn into_bytes(self) -> Vec<u8> {
        self.into_iter_bytes().collect()
    }

    /// Converts a byte buffer into a [DirectionDisableResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let len = Self::len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidResponseLen((buf_len, len)))
        } else {
            Ok(Self {
                code: buf[0].try_into()?,
                dirs: buf.get(1).copied().unwrap_or(0).into(),
            })
        }
    }
}

impl Default for DirectionDisableResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&Response> for DirectionDisableResponse {
    fn from(val: &Response) -> Self {
        Self {
            code: val.code(),
            dirs: val.additional().first().copied().unwrap_or(0).into(),
        }
    }
}

impl From<Response> for DirectionDisableResponse {
    fn from(val: Response) -> Self {
        (&val).into()
    }
}

impl From<DirectionDisableResponse> for Response {
    fn from(val: DirectionDisableResponse) -> Self {
        Self {
            code: val.code,
            additional: [val.dirs.bits()].into(),
        }
    }
}

impl From<&DirectionDisableResponse> for Response {
    fn from(val: &DirectionDisableResponse) -> Self {
        Self {
            code: val.code,
            additional: [val.dirs.bits()].into(),
        }
    }
}

impl TryFrom<Message> for DirectionDisableResponse {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&Message> for DirectionDisableResponse {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        match val.data.message_code().request_code()? {
            RequestCode::DirectionDisable => Ok(Response::try_from(val)?.into()),
            code => Err(Error::InvalidRequestCode(code.into())),
        }
    }
}

impl fmt::Display for DirectionDisableResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code":{}, "#, self.code)?;
        write!(f, r#""directions":{}"#, self.dirs)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_disable_response() {
        let raw = [ResponseCode::Ack as u8, 0];
        let exp = DirectionDisableResponse::new()
            .with_code(ResponseCode::Ack)
            .with_directions(InhibitDirection::new());
        let res = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(&[0]);

        assert_eq!(
            DirectionDisableResponse::from_bytes(raw.as_ref()).as_ref(),
            Ok(&exp)
        );
        assert_eq!(&DirectionDisableResponse::from(&res), &exp);
        assert_eq!(Response::from(&exp), res);

        let out = exp.into_bytes();

        assert_eq!(out, raw);
    }

    #[test]
    fn test_direction_disable_response_invalid() {
        assert!(DirectionDisableResponse::from_bytes(&[]).is_err());
        assert!(
            DirectionDisableResponse::from_bytes([ResponseCode::Reserved as u8, 0].as_ref())
                .is_err()
        );
    }
}
