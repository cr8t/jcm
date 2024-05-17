use std::fmt;

use crate::{Error, Message, Response, ResponseCode, Result};

mod currency_assign;

pub use currency_assign::*;

/// Represents the [Response] to a UID request [Message](crate::Message).
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct CurrencyAssignResponse {
    code: ResponseCode,
    currency_assign: CurrencyAssignList,
}

impl CurrencyAssignResponse {
    /// Creates a new [CurrencyAssignResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            currency_assign: CurrencyAssignList::new(),
        }
    }

    /// Gets the [ResponseCode] for the [CurrencyAssignResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [CurrencyAssignResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [CurrencyAssignResponse].
    pub fn with_code(mut self, code: ResponseCode) -> Self {
        self.set_code(code);
        self
    }

    /// Gets a reference to the list of [CurrencyAssign] items.
    pub fn currency_assign(&self) -> &[CurrencyAssign] {
        self.currency_assign.items()
    }

    /// Sets the list of [CurrencyAssign] items.
    pub fn set_currency_assign(&mut self, val: &[CurrencyAssign]) {
        self.currency_assign.set_items(val);
    }

    /// Builder function that sets the list of [CurrencyAssign] items.
    pub fn with_currency_assign(mut self, val: &[CurrencyAssign]) -> Self {
        self.set_currency_assign(val);
        self
    }

    /// Gets the length of the [CurrencyAssignResponse] metadata.
    pub const fn meta_len() -> usize {
        ResponseCode::len()
    }

    /// Gets the full length of the [CurrencyAssignResponse].
    pub fn len(&self) -> usize {
        Self::meta_len() + self.currency_assign.len()
    }

    /// Gets whether the [CurrencyAssignResponse] is empty.
    pub fn is_empty(&self) -> bool {
        self.code.is_empty() && self.currency_assign.is_empty()
    }

    /// Converts a [CurrencyAssignResponse] into a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        match buf_len {
            bl if bl < len => Err(Error::InvalidResponseLen((buf_len, len))),
            _ => {
                buf.iter_mut()
                    .take(len)
                    .zip(
                        [self.code.into()]
                            .into_iter()
                            .chain(self.currency_assign.iter_bytes()),
                    )
                    .for_each(|(dst, src)| *dst = src);

                Ok(())
            }
        }
    }

    /// Converts a byte buffer into a [CurrencyAssignResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let meta_len = Self::meta_len();
        let buf_len = buf.len();

        match buf_len {
            bl if bl < meta_len => Err(Error::InvalidResponseLen((buf_len, meta_len))),
            bl if bl == meta_len => Ok(Self {
                code: buf[0].try_into()?,
                currency_assign: CurrencyAssignList::new(),
            }),
            _ => Ok(Self {
                code: buf[0].try_into()?,
                currency_assign: buf[1..].as_ref().try_into()?,
            }),
        }
    }
}

impl Default for CurrencyAssignResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&Response> for CurrencyAssignResponse {
    type Error = Error;

    fn try_from(val: &Response) -> Result<Self> {
        let meta_len = Self::meta_len();

        match val.len() {
            res_len if res_len < meta_len => Err(Error::InvalidResponseLen((res_len, meta_len))),
            _ => Ok(Self {
                code: val.code,
                currency_assign: CurrencyAssignList::try_from(val.additional())?,
            }),
        }
    }
}

impl TryFrom<Response> for CurrencyAssignResponse {
    type Error = Error;

    fn try_from(val: Response) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&Message> for CurrencyAssignResponse {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        Response::try_from(val)?.try_into()
    }
}

impl TryFrom<Message> for CurrencyAssignResponse {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        Response::try_from(val)?.try_into()
    }
}

impl From<&CurrencyAssignResponse> for Response {
    fn from(val: &CurrencyAssignResponse) -> Self {
        Self {
            code: val.code,
            additional: val.currency_assign.iter_bytes().collect(),
        }
    }
}

impl From<CurrencyAssignResponse> for Response {
    fn from(val: CurrencyAssignResponse) -> Self {
        (&val).into()
    }
}

impl fmt::Display for CurrencyAssignResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code":{}, "#, self.code)?;
        write!(f, r#""currency_assign": {}"#, self.currency_assign)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Currency, CurrencyCode, Denomination};

    #[test]
    fn test_currency_assign_response() {
        let raw = [
            ResponseCode::Ack as u8,
            0,
            b'J',
            b'P',
            b'Y',
            1,
            0,
            1,
            b'J',
            b'P',
            b'Y',
            5,
            0,
        ];

        let exp_currency_assign: Vec<CurrencyAssign> = [1u64, 5u64]
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                CurrencyAssign::new()
                    .with_bit_number(i as u8)
                    .with_currency(
                        Currency::new()
                            .with_code(CurrencyCode::JPY)
                            .with_denomination(Denomination::from_value(v)),
                    )
            })
            .collect();

        let exp = CurrencyAssignResponse::new()
            .with_code(ResponseCode::Ack)
            .with_currency_assign(exp_currency_assign.as_ref());

        let res = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(&raw[1..]);

        let mut out = [0u8; 13];

        assert_eq!(
            CurrencyAssignResponse::from_bytes(raw.as_ref()).as_ref(),
            Ok(&exp)
        );
        assert_eq!(CurrencyAssignResponse::try_from(&res).as_ref(), Ok(&exp));
        assert_eq!(Response::from(&exp), res);

        assert!(exp.to_bytes(out.as_mut()).is_ok());

        assert_eq!(out, raw);
    }

    #[test]
    fn test_currency_assign_response_invalid() {
        let raw = [ResponseCode::Ack as u8, 0, 0, 0, 0, 0, 0];
        let exp = CurrencyAssignResponse::new()
            .with_code(ResponseCode::Ack)
            .with_currency_assign(&[CurrencyAssign::new()]);
        let mut out = [0u8; 7];

        assert!(CurrencyAssignResponse::from_bytes(raw[..2].as_ref()).is_err());
        assert!(
            CurrencyAssignResponse::from_bytes([ResponseCode::Reserved as u8, 0].as_ref()).is_err()
        );
        assert!(exp.to_bytes(out[..1].as_mut()).is_err());
        assert!(exp.to_bytes(&mut []).is_err());
        assert!(CurrencyAssignResponse::try_from(Response::new().with_additional(&[])).is_err());
        assert!(CurrencyAssignResponse::try_from(
            Response::new()
                .with_code(ResponseCode::Ack)
                .with_additional(&[])
        )
        .is_err());
    }
}
