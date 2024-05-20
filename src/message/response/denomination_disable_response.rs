use std::fmt;

use crate::{
    DenominationDisable, DenominationDisableList, Error, Message, RequestCode, Response,
    ResponseCode, Result,
};

/// Represents the [Response] to a [DenominationDisableRequest](crate::DenominationDisableRequest).
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenominationDisableResponse {
    code: ResponseCode,
    denoms: DenominationDisableList,
}

impl DenominationDisableResponse {
    /// Creates a new [DenominationDisableResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            denoms: DenominationDisableList::new(),
        }
    }

    /// Gets the [ResponseCode] for the [DenominationDisableResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [DenominationDisableResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [DenominationDisableResponse].
    pub fn with_code(mut self, code: ResponseCode) -> Self {
        self.set_code(code);
        self
    }

    /// Gets the list of [DenominationDisable] items for the [DenominationDisableResponse].
    pub fn denominations(&self) -> &[DenominationDisable] {
        self.denoms.items()
    }

    /// Sets the list of [DenominationDisable] items for the [DenominationDisableResponse].
    pub fn set_denominations(&mut self, denoms: &[DenominationDisable]) {
        self.denoms = denoms.into();
    }

    /// Builder function that sets the list of [DenominationDisable] items for the [DenominationDisableResponse].
    pub fn with_denominations(mut self, denoms: &[DenominationDisable]) -> Self {
        self.set_denominations(denoms);
        self
    }

    /// Gets the metadata length of the [DenominatinoDisableResponse].
    pub const fn meta_len() -> usize {
        ResponseCode::len()
    }

    /// Gets the length of the [DenominationDisableResponse].
    pub fn len(&self) -> usize {
        ResponseCode::len().saturating_add(self.denoms.len())
    }

    /// Gets whether the [DenominationDisableResponse] is empty.
    pub fn is_empty(&self) -> bool {
        self.code.is_empty() && self.denoms.is_empty()
    }

    /// Gets an iterator over [DenominationDisableResponse] bytes.
    pub fn iter_bytes(&self) -> impl Iterator<Item = u8> + '_ {
        [u8::from(self.code)]
            .into_iter()
            .chain(self.denoms.iter_bytes())
    }

    /// Gets an iterator over [DenominationDisableResponse] bytes.
    pub fn into_iter_bytes(self) -> impl Iterator<Item = u8> {
        [u8::from(self.code)]
            .into_iter()
            .chain(self.denoms.into_iter_bytes())
    }

    /// Converts a [DenominationDisableResponse] into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.iter_bytes().collect()
    }

    /// Converts a [DenominationDisableResponse] into a byte vector.
    pub fn into_bytes(self) -> Vec<u8> {
        self.into_iter_bytes().collect()
    }

    /// Converts a byte buffer into a [DenominationDisableResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let len = Self::meta_len();
        let buf_len = buf.len();

        let denoms = DenominationDisableList::from_bytes(buf.get(1..).unwrap_or_default());

        if buf_len < len {
            Err(Error::InvalidResponseLen((buf_len, len)))
        } else {
            Ok(Self {
                code: buf[0].try_into()?,
                denoms,
            })
        }
    }
}

impl Default for DenominationDisableResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&Response> for DenominationDisableResponse {
    fn from(val: &Response) -> Self {
        Self {
            code: val.code(),
            denoms: DenominationDisableList::from_bytes(val.additional()),
        }
    }
}

impl From<Response> for DenominationDisableResponse {
    fn from(val: Response) -> Self {
        (&val).into()
    }
}

impl From<DenominationDisableResponse> for Response {
    fn from(val: DenominationDisableResponse) -> Self {
        Self {
            code: val.code,
            additional: val.denoms.into_bytes(),
        }
    }
}

impl From<&DenominationDisableResponse> for Response {
    fn from(val: &DenominationDisableResponse) -> Self {
        Self {
            code: val.code,
            additional: val.denoms.to_bytes(),
        }
    }
}

impl TryFrom<Message> for DenominationDisableResponse {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&Message> for DenominationDisableResponse {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        match val.data.message_code().request_code()? {
            RequestCode::DenominationDisable => Ok(Response::try_from(val)?.into()),
            code => Err(Error::InvalidRequestCode(code.into())),
        }
    }
}

impl fmt::Display for DenominationDisableResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code":{}, "#, self.code)?;
        write!(f, r#""denominations":{}"#, self.denoms)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_denomination_disable_response() {
        let raw = [ResponseCode::Ack as u8, 0, 0];
        let exp = DenominationDisableResponse::new()
            .with_code(ResponseCode::Ack)
            .with_denominations(&[DenominationDisable::new()]);
        let res = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(&[0, 0]);

        assert_eq!(
            DenominationDisableResponse::from_bytes(raw.as_ref()).as_ref(),
            Ok(&exp)
        );
        assert_eq!(&DenominationDisableResponse::from(&res), &exp);
        assert_eq!(Response::from(&exp), res);

        let out = exp.into_bytes();

        assert_eq!(out, raw);
    }

    #[test]
    fn test_denomination_disable_response_invalid() {
        assert!(DenominationDisableResponse::from_bytes(&[]).is_err());
        assert!(DenominationDisableResponse::from_bytes(
            [ResponseCode::Reserved as u8, 0].as_ref()
        )
        .is_err());
    }
}
