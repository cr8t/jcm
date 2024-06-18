use std::fmt;

use crate::{Error, Message, Response, ResponseCode, Result, SerialNumberBlock};

/// Represents the [Response] to a UID request [Message](crate::Message).
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerialNumberBlockResponse {
    code: ResponseCode,
    block: SerialNumberBlock,
}

impl SerialNumberBlockResponse {
    /// Creates a new [SerialNumberBlockResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            block: SerialNumberBlock::new(),
        }
    }

    /// Gets the [ResponseCode] for the [SerialNumberBlockResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [SerialNumberBlockResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [SerialNumberBlockResponse].
    pub fn with_code(mut self, code: ResponseCode) -> Self {
        self.set_code(code);
        self
    }

    /// Gets the [Block](SerialNumberBlock) for the [SerialNumberBlockResponse].
    pub const fn block(&self) -> &SerialNumberBlock {
        &self.block
    }

    /// Sets the UID for the [SerialNumberBlockResponse].
    pub fn set_block(&mut self, val: SerialNumberBlock) {
        self.block = val;
    }

    /// Builder function that sets the UID for the [SerialNumberBlockResponse].
    pub fn with_block(self, val: SerialNumberBlock) -> Self {
        Self {
            code: self.code,
            block: val,
        }
    }

    /// Gets the length of the [SerialNumberBlockResponse].
    pub fn len(&self) -> usize {
        ResponseCode::len().saturating_add(self.block.len())
    }

    /// Gets whether the [SerialNumberBlockResponse] is empty.
    pub fn is_empty(&self) -> bool {
        self.code.is_empty() && self.block.is_empty()
    }

    /// Converts a [SerialNumberBlockResponse] into a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidResponseLen((buf_len, len)))
        } else {
            buf.iter_mut()
                .take(len)
                .zip(
                    [u8::from(self.code)]
                        .into_iter()
                        .chain(self.block.iter().copied()),
                )
                .for_each(|(dst, src)| *dst = src);

            Ok(())
        }
    }

    /// Converts a byte buffer into a [SerialNumberBlockResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        Ok(Self {
            code: buf
                .first()
                .copied()
                .ok_or(Error::InvalidResponseLen((buf.len(), 1)))?
                .try_into()?,
            block: buf
                .get(1..)
                .map(SerialNumberBlock::from)
                .unwrap_or_default(),
        })
    }
}

impl Default for SerialNumberBlockResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&Response> for SerialNumberBlockResponse {
    type Error = Error;

    fn try_from(val: &Response) -> Result<Self> {
        Ok(Self {
            code: val.code,
            block: val.additional.as_slice().into(),
        })
    }
}

impl TryFrom<Response> for SerialNumberBlockResponse {
    type Error = Error;

    fn try_from(val: Response) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&Message> for SerialNumberBlockResponse {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        Response::try_from(val)?.try_into()
    }
}

impl TryFrom<Message> for SerialNumberBlockResponse {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        Response::try_from(val)?.try_into()
    }
}

impl From<SerialNumberBlockResponse> for Response {
    fn from(val: SerialNumberBlockResponse) -> Self {
        Self {
            code: val.code,
            additional: val.block.into(),
        }
    }
}

impl From<&SerialNumberBlockResponse> for Response {
    fn from(val: &SerialNumberBlockResponse) -> Self {
        val.clone().into()
    }
}

impl fmt::Display for SerialNumberBlockResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code":{}, "#, self.code)?;
        write!(f, r#""block":{}"#, self.block)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_number_block_response() {
        let raw: Vec<u8> = [ResponseCode::Ack as u8]
            .into_iter()
            .chain([0; 128])
            .collect();
        let exp = SerialNumberBlockResponse::new()
            .with_code(ResponseCode::Ack)
            .with_block(raw[1..].into());
        let exp_block = SerialNumberBlock::create(raw[1..].into());
        let res = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(exp_block.block());
        let mut out = [0u8; 129];

        assert!(exp.to_bytes(out.as_mut()).is_ok());
        assert_eq!(&out, raw.as_slice());

        assert_eq!(
            SerialNumberBlockResponse::from_bytes(raw.as_ref()),
            Ok(exp.clone())
        );

        assert_eq!(Response::from(&exp), res);
        assert_eq!(SerialNumberBlockResponse::try_from(&res), Ok(exp));
    }

    #[test]
    fn test_serial_number_block_response_invalid() {
        (0..=u8::MAX)
            .filter(|&c| ResponseCode::try_from(c).is_err())
            .for_each(|invalid| {
                assert!(SerialNumberBlockResponse::from_bytes(&[invalid]).is_err());
            });
    }
}
