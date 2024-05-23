use std::fmt;

use crate::{Error, Message, Response, ResponseCode, Result};

mod model_name;

pub use model_name::*;

/// Represents the response to a [VersionRequest](crate::VersionRequest).
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelNameResponse {
    code: ResponseCode,
    model_name: ModelName,
}

impl ModelNameResponse {
    /// Creates a new [ModelNameResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            model_name: ModelName::new(),
        }
    }

    /// Gets the [ResponseCode] for the [ModelNameResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [ModelNameResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [ModelNameResponse].
    pub fn with_code(mut self, code: ResponseCode) -> Self {
        self.set_code(code);
        self
    }

    /// Gets the model name for the [ModelNameResponse].
    pub const fn model_name(&self) -> &ModelName {
        &self.model_name
    }

    /// Sets the model name for the [ModelNameResponse].
    pub fn set_model_name(&mut self, model_name: ModelName) {
        self.model_name = model_name;
    }

    /// Builder function that sets the [CString] for the [ModelNameResponse].
    pub fn with_model_name(mut self, model_name: ModelName) -> Self {
        self.set_model_name(model_name);
        self
    }

    /// Gets the metadata length of the [ModelNameResponse].
    pub const fn meta_len() -> usize {
        ResponseCode::len()
    }

    /// Gets the length of the [ModelNameResponse].
    pub fn len(&self) -> usize {
        Self::meta_len() + self.model_name.len()
    }

    /// Gets whether the [ModelNameResponse] is empty.
    pub fn is_empty(&self) -> bool {
        self.code.is_empty() && self.model_name.is_empty()
    }

    /// Attempts to convert a byte buffer into a [ModelNameResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let meta_len = Self::meta_len();
        let buf_len = buf.len();

        match buf_len {
            bl if bl < meta_len => Err(Error::InvalidResponseLen((buf_len, meta_len))),
            bl if bl == meta_len => Ok(Self {
                code: buf[0].try_into()?,
                model_name: ModelName::new(),
            }),
            _ => Ok(Self {
                code: buf[0].try_into()?,
                model_name: buf[1..].try_into()?,
            }),
        }
    }

    /// Converts the [ModelNameResponse] into a byte iterator.
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        [self.code.to_u8()]
            .into_iter()
            .chain(self.model_name.iter_with_nul())
    }

    /// Attempts to convert the [ModelNameResponse] into a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidResponseLen((buf_len, len)))
        } else {
            buf.iter_mut()
                .take(len)
                .zip(self.iter())
                .for_each(|(dst, src)| *dst = src);
            Ok(())
        }
    }

    /// Converts the [ModelNameResponse] into a byte vector.
    pub fn into_bytes(&self) -> Vec<u8> {
        self.iter().collect()
    }
}

impl TryFrom<&[u8]> for ModelNameResponse {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::from_bytes(val)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for ModelNameResponse {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        Self::from_bytes(val.as_ref())
    }
}

impl<const N: usize> TryFrom<[u8; N]> for ModelNameResponse {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        Self::from_bytes(val.as_ref())
    }
}

impl From<ModelNameResponse> for Response {
    fn from(val: ModelNameResponse) -> Self {
        Self {
            code: val.code,
            additional: val.model_name.into_bytes_with_nul(),
        }
    }
}

impl From<&ModelNameResponse> for Response {
    fn from(val: &ModelNameResponse) -> Self {
        val.clone().into()
    }
}

impl TryFrom<&Response> for ModelNameResponse {
    type Error = Error;

    fn try_from(val: &Response) -> Result<Self> {
        match val.additional().len() {
            0 => Ok(Self {
                code: val.code,
                model_name: ModelName::new(),
            }),
            _ => Ok(Self {
                code: val.code,
                model_name: val.additional().try_into()?,
            }),
        }
    }
}

impl TryFrom<Response> for ModelNameResponse {
    type Error = Error;

    fn try_from(val: Response) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&ModelNameResponse> for Message {
    fn from(val: &ModelNameResponse) -> Self {
        use crate::{MessageData, ModelNameRequest};

        MessageData::from(ModelNameRequest::new())
            .with_additional(val.into_bytes().as_ref())
            .into()
    }
}

impl From<ModelNameResponse> for Message {
    fn from(val: ModelNameResponse) -> Self {
        (&val).into()
    }
}

impl TryFrom<&Message> for ModelNameResponse {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        Response::try_from(val)?.try_into()
    }
}

impl TryFrom<Message> for ModelNameResponse {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl Default for ModelNameResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ModelNameResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code": {}, "#, self.code())?;
        write!(f, r#""model_name": {}"#, self.model_name())?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_name_response() -> Result<()> {
        let raw_code = ResponseCode::Ack as u8;
        let raw_model_name = b"SomeVersion\0";
        let raw: Vec<u8> = [raw_code]
            .into_iter()
            .chain(raw_model_name.iter().copied())
            .collect();

        let exp_model_name = ModelName::from_string("SomeVersion");

        let exp = ModelNameResponse::new()
            .with_code(ResponseCode::Ack)
            .with_model_name(exp_model_name.clone());

        let res = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(raw_model_name.as_ref());

        assert_eq!(ModelNameResponse::from_bytes(raw.as_ref())?, exp);
        assert_eq!(ModelNameResponse::try_from(&res)?, exp);
        assert_eq!(Response::from(&exp), res);

        let out = exp.into_bytes();
        assert_eq!(out, raw);

        Ok(())
    }

    #[test]
    fn test_model_name_response_invalid() -> Result<()> {
        let raw_code = ResponseCode::Ack as u8;
        let good_vers = ModelName::from_bytes_until_nul(b"SomeVersion\0")?;
        let bad_vers = [
            // invalid UTF-8 model name
            [0xff, 0xff, 0xff, b'\0'].as_ref(),
        ];

        let exp = ModelNameResponse::new()
            .with_code(ResponseCode::Ack)
            .with_model_name(good_vers.clone());
        let exp_len = exp.len();
        let mut out = vec![0u8; exp_len];

        for ver in bad_vers.into_iter() {
            let raw: Vec<u8> = [raw_code].into_iter().chain(ver.iter().cloned()).collect();
            assert!(ModelNameResponse::from_bytes(raw.as_ref()).is_err());
            assert!(exp.to_bytes(out[..raw.len()].as_mut()).is_err());
            assert!(exp.to_bytes(&mut []).is_err());
            assert!(ModelNameResponse::try_from(Response::new().with_additional(ver)).is_err());
            assert!(ModelNameResponse::try_from(
                Response::new()
                    .with_code(ResponseCode::Ack)
                    .with_additional(ver)
            )
            .is_err());
        }

        Ok(())
    }
}
