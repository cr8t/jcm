use std::fmt;

use crate::{Error, Message, NearFullData, RequestCode, Response, ResponseCode, Result};

/// Represents the [Response] to a [DirectionDisableRequest](crate::DirectionDisableRequest).
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NearFullResponse {
    code: ResponseCode,
    data: Option<NearFullData>,
}

impl NearFullResponse {
    /// Creates a new [NearFullResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            data: None,
        }
    }

    /// Gets the [ResponseCode] for the [NearFullResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [NearFullResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [NearFullResponse].
    pub const fn with_code(self, code: ResponseCode) -> Self {
        Self {
            code,
            data: self.data,
        }
    }

    /// Gets the [NearFullData] for the [NearFullDataResponse].
    pub fn data(&self) -> Option<NearFullData> {
        self.data
    }

    /// Sets the [NearFullData] for the [NearFullDataResponse].
    pub fn set_data(&mut self, data: NearFullData) {
        self.data = Some(data);
    }

    /// Unsets the [NearFullData] for the [NearFullDataResponse].
    pub fn unset_data(&mut self) -> Option<NearFullData> {
        self.data.take()
    }

    /// Builder function that sets the [NearFullData] for the [NearFullDataResponse].
    pub const fn with_data(self, data: NearFullData) -> Self {
        Self {
            code: self.code,
            data: Some(data),
        }
    }

    /// Gets the length of the [NearFullResponse].
    pub const fn len() -> usize {
        ResponseCode::len() + NearFullData::len()
    }

    /// Gets whether the [NearFullResponse] is empty.
    pub fn is_empty(&self) -> bool {
        self.code.is_empty() && self.data.is_none()
    }

    /// Gets an iterator over [NearFullResponse] bytes.
    pub fn into_iter_bytes(self) -> impl Iterator<Item = u8> {
        let mut code_iter = [self.code.into()].into_iter();
        let mut data_iter = self.data.map(|d| d.into_iter());

        std::iter::from_fn(move || match (code_iter.next(), data_iter.as_mut()) {
            (Some(c), _) => Some(c),
            (None, Some(d)) => d.next(),
            (None, None) => None,
        })
    }

    /// Converts a [NearFullResponse] into a byte vector.
    pub fn into_bytes(self) -> Vec<u8> {
        self.into_iter_bytes().collect()
    }

    /// Converts a byte buffer into a [NearFullResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        Ok(Self {
            code: buf
                .first()
                .copied()
                .ok_or(Error::InvalidResponseLen((0, 1)))?
                .try_into()?,
            data: match buf
                .get(1..1 + NearFullData::len())
                .map(NearFullData::from_bytes)
            {
                Some(d) => Some(d?),
                None => None,
            },
        })
    }
}

impl Default for NearFullResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&Response> for NearFullResponse {
    fn from(val: &Response) -> Self {
        Self {
            code: val.code(),
            data: val.additional().try_into().ok(),
        }
    }
}

impl From<Response> for NearFullResponse {
    fn from(val: Response) -> Self {
        (&val).into()
    }
}

impl From<NearFullResponse> for Response {
    fn from(val: NearFullResponse) -> Self {
        Self {
            code: val.code,
            additional: val
                .data
                .map(|d| d.into_bytes().to_vec())
                .unwrap_or_default(),
        }
    }
}

impl From<&NearFullResponse> for Response {
    fn from(val: &NearFullResponse) -> Self {
        Self {
            code: val.code,
            additional: val
                .data
                .map(|d| d.into_bytes().to_vec())
                .unwrap_or_default(),
        }
    }
}

impl TryFrom<Message> for NearFullResponse {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&Message> for NearFullResponse {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        match val.data.message_code().request_code()? {
            RequestCode::DirectionDisable => Ok(Response::try_from(val)?.into()),
            code => Err(Error::InvalidRequestCode(code.into())),
        }
    }
}

impl fmt::Display for NearFullResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code":{}"#, self.code)?;
        if let Some(data) = self.data.as_ref() {
            write!(f, r#", "data":{data}"#)?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NearFullMode;

    #[test]
    fn test_near_full_response() {
        for mode in [NearFullMode::Get, NearFullMode::Set] {
            let raw = match mode {
                NearFullMode::Get => vec![ResponseCode::Ack as u8, 0, 0, 0],
                NearFullMode::Set => vec![ResponseCode::Ack as u8],
            };

            let exp = match mode {
                NearFullMode::Get => NearFullResponse::new()
                    .with_code(ResponseCode::Ack)
                    .with_data(NearFullData::new()),
                NearFullMode::Set => NearFullResponse::new().with_code(ResponseCode::Ack),
            };

            let res = match mode {
                NearFullMode::Get => Response::new()
                    .with_code(ResponseCode::Ack)
                    .with_additional(raw[1..].as_ref()),
                NearFullMode::Set => Response::new().with_code(ResponseCode::Ack),
            };

            assert_eq!(
                NearFullResponse::from_bytes(raw.as_ref()).as_ref(),
                Ok(&exp),
            );
            assert_eq!(&NearFullResponse::from(&res), &exp);
            assert_eq!(Response::from(&exp), res);

            let out = exp.into_bytes();

            assert_eq!(out, raw);
        }
    }

    #[test]
    fn test_near_full_response_invalid() {
        assert!(NearFullResponse::from_bytes(&[]).is_err());
        assert!(NearFullResponse::from_bytes([ResponseCode::Reserved as u8, 0].as_ref()).is_err());
    }
}
