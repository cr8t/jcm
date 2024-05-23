use std::fmt;

use crate::{AlgorithmNumber, Error, Message, Response, ResponseCode, Result};

/// Represents the [Response] to a UID request [Message](crate::Message).
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProgramSignatureResponse {
    code: ResponseCode,
    algorithm: AlgorithmNumber,
}

impl ProgramSignatureResponse {
    /// Creates a new [ProgramSignatureResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            algorithm: AlgorithmNumber::new(),
        }
    }

    /// Gets the [ResponseCode] for the [ProgramSignatureResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [ProgramSignatureResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [ProgramSignatureResponse].
    pub fn with_code(mut self, code: ResponseCode) -> Self {
        self.set_code(code);
        self
    }

    /// Gets the [AlgorithmNumber] for the [ProgramSignatureResponse].
    pub const fn algorithm_number(&self) -> AlgorithmNumber {
        self.algorithm
    }

    /// Sets the [AlgorithmNumber] for the [ProgramSignatureResponse].
    pub fn set_algorithm_number(&mut self, algo: AlgorithmNumber) {
        self.algorithm = algo;
    }

    /// Builder function that sets the [AlgorithmNumber] for the [ProgramSignatureResponse].
    pub fn with_algorithm_number(mut self, algo: AlgorithmNumber) -> Self {
        self.set_algorithm_number(algo);
        self
    }

    /// Gets the metadata length of the [ProgramSignatureResponse].
    pub const fn meta_len() -> usize {
        ResponseCode::len()
    }

    /// Gets the byte length of the [ProgramSignatureResponse].
    pub const fn len(&self) -> usize {
        ResponseCode::len() + self.algorithm.len()
    }

    /// Gets whether the [ProgramSignatureResponse] is empty.
    pub const fn is_empty(&self) -> bool {
        self.code.is_empty() && self.algorithm.is_empty()
    }

    /// Converts the [ProgramSignatureResponse] into a byte iterator.
    pub fn iter(self) -> impl Iterator<Item = u8> {
        [self.code.to_u8(), self.algorithm.into_u8()]
            .into_iter()
            .take(self.len())
    }

    /// Converts a [ProgramSignatureResponse] into a byte buffer.
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

    /// Converts a byte buffer into a [ProgramSignatureResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let buf_len = buf.len();

        match buf.len() {
            0 => Err(Error::InvalidResponseLen((buf_len, Self::meta_len()))),
            1 => Ok(Self {
                code: buf[0].try_into()?,
                algorithm: AlgorithmNumber::Reserved,
            }),
            _ => Ok(Self {
                code: buf[0].try_into()?,
                algorithm: buf[1].try_into()?,
            }),
        }
    }
}

impl Default for ProgramSignatureResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&Response> for ProgramSignatureResponse {
    type Error = Error;

    fn try_from(val: &Response) -> Result<Self> {
        let len = Self::meta_len();

        match val.len() {
            res_len if res_len < len => Err(Error::InvalidResponseLen((res_len, len))),
            1 => Ok(Self {
                code: val.code,
                algorithm: AlgorithmNumber::Reserved,
            }),
            _ => Ok(Self {
                code: val.code,
                algorithm: val.additional[0].try_into()?,
            }),
        }
    }
}

impl TryFrom<Response> for ProgramSignatureResponse {
    type Error = Error;

    fn try_from(val: Response) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<Message> for ProgramSignatureResponse {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        Response::try_from(val)?.try_into()
    }
}

impl TryFrom<&Message> for ProgramSignatureResponse {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        Response::try_from(val)?.try_into()
    }
}

impl From<ProgramSignatureResponse> for Response {
    fn from(val: ProgramSignatureResponse) -> Self {
        Self {
            code: val.code,
            additional: match val.algorithm {
                AlgorithmNumber::Reserved => Vec::new(),
                algo => [algo.into_u8()].into(),
            },
        }
    }
}

impl From<&ProgramSignatureResponse> for Response {
    fn from(val: &ProgramSignatureResponse) -> Self {
        (*val).into()
    }
}

impl fmt::Display for ProgramSignatureResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code":{}, "#, self.code)?;
        write!(f, r#""algorithm": {}"#, self.algorithm)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_signature_response() {
        let raw = [ResponseCode::Ack as u8, AlgorithmNumber::Crc16.into_u8()];
        let exp = ProgramSignatureResponse::new().with_code(ResponseCode::Ack);
        let res = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(&[raw[1]]);
        let mut out = [0u8; 2];

        assert_eq!(ProgramSignatureResponse::from_bytes(raw.as_ref()), Ok(exp));
        assert_eq!(ProgramSignatureResponse::try_from(&res), Ok(exp));
        assert_eq!(Response::from(&exp), res);

        assert!(exp.to_bytes(out.as_mut()).is_ok());

        assert_eq!(out, raw);
    }

    #[test]
    fn test_program_signature_response_invalid() {
        let exp = ProgramSignatureResponse::new().with_code(ResponseCode::Ack);

        assert!(ProgramSignatureResponse::from_bytes(&[]).is_err());
        assert!(
            ProgramSignatureResponse::from_bytes([ResponseCode::Reserved as u8, 0].as_ref())
                .is_err()
        );
        assert!(exp.to_bytes(&mut []).is_err());
        assert!(ProgramSignatureResponse::try_from(Response::new().with_additional(&[0])).is_err());
        assert!(ProgramSignatureResponse::try_from(
            Response::new()
                .with_code(ResponseCode::Ack)
                .with_additional(&[0])
        )
        .is_err());
    }
}
