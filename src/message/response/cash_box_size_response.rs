use std::fmt;

use crate::{CashBoxSize, Error, Message, Response, ResponseCode, Result};

/// Represents the response to a [CashBoxSizeRequest](crate::CashBoxSizeRequest).
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CashBoxSizeResponse {
    code: ResponseCode,
    cash_box_size: CashBoxSize,
}

impl CashBoxSizeResponse {
    /// Creates a new [CashBoxSizeResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            cash_box_size: CashBoxSize::new(),
        }
    }

    /// Gets the [ResponseCode] for the [CashBoxSizeResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [CashBoxSizeResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [CashBoxSizeResponse].
    pub const fn with_code(self, code: ResponseCode) -> Self {
        Self {
            code,
            cash_box_size: self.cash_box_size,
        }
    }

    /// Gets the [CashBoxSize] for the [CashBoxSizeResponse].
    pub const fn cash_box_size(&self) -> &CashBoxSize {
        &self.cash_box_size
    }

    /// Sets the [CashBoxSize] for the [CashBoxSizeResponse].
    pub fn set_cash_box_size(&mut self, cash_box_size: CashBoxSize) {
        self.cash_box_size = cash_box_size;
    }

    /// Builder function that sets the [CashBoxSize] for the [CashBoxSizeResponse].
    pub const fn with_cash_box_size(self, cash_box_size: CashBoxSize) -> Self {
        Self {
            code: self.code,
            cash_box_size,
        }
    }

    /// Gets the metadata length of the [CashBoxSizeResponse].
    pub const fn meta_len() -> usize {
        ResponseCode::len()
    }

    /// Gets the length of the [CashBoxSizeResponse].
    pub fn len(&self) -> usize {
        Self::meta_len() + self.cash_box_size.len()
    }

    /// Gets whether the [CashBoxSizeResponse] is empty.
    pub fn is_empty(&self) -> bool {
        self.code.is_empty() && self.cash_box_size.is_empty()
    }

    /// Attempts to convert a byte buffer into a [CashBoxSizeResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let meta_len = Self::meta_len();
        let buf_len = buf.len();

        Ok(Self {
            code: buf
                .first()
                .copied()
                .ok_or(Error::InvalidResponseLen((buf_len, meta_len)))?
                .try_into()?,
            cash_box_size: buf
                .get(1..)
                .ok_or(Error::InvalidResponseLen((buf_len, meta_len)))?
                .try_into()?,
        })
    }

    /// Converts the [CashBoxSizeResponse] into a byte iterator.
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        [self.code.to_u8()].into_iter().chain(self.cash_box_size)
    }

    /// Attempts to convert the [CashBoxSizeResponse] into a byte buffer.
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

    /// Converts the [CashBoxSizeResponse] into a byte vector.
    pub fn into_bytes(&self) -> Vec<u8> {
        self.iter().collect()
    }
}

impl TryFrom<&[u8]> for CashBoxSizeResponse {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::from_bytes(val)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for CashBoxSizeResponse {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        Self::from_bytes(val.as_ref())
    }
}

impl<const N: usize> TryFrom<[u8; N]> for CashBoxSizeResponse {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        Self::from_bytes(val.as_ref())
    }
}

impl From<&CashBoxSizeResponse> for Response {
    fn from(val: &CashBoxSizeResponse) -> Self {
        Self {
            code: val.code,
            additional: val.cash_box_size.into(),
        }
    }
}

impl From<CashBoxSizeResponse> for Response {
    fn from(val: CashBoxSizeResponse) -> Self {
        (&val).into()
    }
}

impl TryFrom<&Response> for CashBoxSizeResponse {
    type Error = Error;

    fn try_from(val: &Response) -> Result<Self> {
        match val.additional().len() {
            0 => Ok(Self {
                code: val.code,
                cash_box_size: CashBoxSize::new(),
            }),
            _ => Ok(Self {
                code: val.code,
                cash_box_size: val.additional().try_into()?,
            }),
        }
    }
}

impl TryFrom<Response> for CashBoxSizeResponse {
    type Error = Error;

    fn try_from(val: Response) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&CashBoxSizeResponse> for Message {
    fn from(val: &CashBoxSizeResponse) -> Self {
        use crate::{CashBoxSizeRequest, MessageData};

        MessageData::from(CashBoxSizeRequest::new())
            .with_additional(val.into_bytes().as_ref())
            .into()
    }
}

impl From<CashBoxSizeResponse> for Message {
    fn from(val: CashBoxSizeResponse) -> Self {
        (&val).into()
    }
}

impl TryFrom<&Message> for CashBoxSizeResponse {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        Response::try_from(val)?.try_into()
    }
}

impl TryFrom<Message> for CashBoxSizeResponse {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl Default for CashBoxSizeResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CashBoxSizeResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code": {}, "#, self.code())?;
        write!(f, r#""cash_box_size": {}"#, self.cash_box_size())?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cash_box_size_response() -> Result<()> {
        let raw_code = ResponseCode::Ack as u8;
        let raw_version = b"100\0";
        let raw: Vec<u8> = [raw_code]
            .into_iter()
            .chain(raw_version.iter().cloned())
            .collect();

        let exp_version = CashBoxSize::try_from(raw_version.as_ref())?;

        let exp = CashBoxSizeResponse::new()
            .with_code(ResponseCode::Ack)
            .with_cash_box_size(exp_version.clone());

        let res = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(raw_version.as_ref());

        assert_eq!(CashBoxSizeResponse::from_bytes(raw.as_ref())?, exp);
        assert_eq!(CashBoxSizeResponse::try_from(&res)?, exp);
        assert_eq!(Response::from(&exp), res);

        let out = exp.into_bytes();
        assert_eq!(out, raw);

        Ok(())
    }

    #[test]
    fn test_cash_box_size_response_invalid() -> Result<()> {
        let raw_code = ResponseCode::Ack as u8;
        let good_vers = CashBoxSize::try_from(b"100\0")?;
        let bad_vers = [
            // no nul
            b"100".as_ref(),
            // invalid number
            b"100S\0".as_ref(),
            // no number
            b"\0".as_ref(),
        ];

        let exp = CashBoxSizeResponse::new()
            .with_code(ResponseCode::Ack)
            .with_cash_box_size(good_vers.clone());

        for ver in bad_vers.into_iter() {
            let raw: Vec<u8> = [raw_code].into_iter().chain(ver.iter().cloned()).collect();
            assert!(CashBoxSizeResponse::from_bytes(raw.as_ref()).is_err());
            assert!(exp.to_bytes(&mut []).is_err());
            assert!(CashBoxSizeResponse::try_from(Response::new().with_additional(ver)).is_err());
            assert!(CashBoxSizeResponse::try_from(
                Response::new()
                    .with_code(ResponseCode::Ack)
                    .with_additional(ver)
            )
            .is_err());
        }

        Ok(())
    }
}
