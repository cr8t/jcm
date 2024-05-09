use std::fmt;

use crate::{Error, Message, Response, ResponseCode, Result};

mod firmware_version;

pub use firmware_version::*;

/// Represents the response to a [VersionRequest](crate::VersionRequest).
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VersionResponse {
    code: ResponseCode,
    firmware_version: FirmwareVersion,
}

impl VersionResponse {
    /// Creates a new [VersionResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            firmware_version: FirmwareVersion::new(),
        }
    }

    /// Gets the [ResponseCode] for the [VersionResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [VersionResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [VersionResponse].
    pub fn with_code(mut self, code: ResponseCode) -> Self {
        self.set_code(code);
        self
    }

    /// Gets the [FirmwareVersion] for the [VersionResponse].
    pub const fn firmware_version(&self) -> &FirmwareVersion {
        &self.firmware_version
    }

    /// Sets the [FirmwareVersion] for the [VersionResponse].
    pub fn set_firmware_version(&mut self, firmware_version: FirmwareVersion) {
        self.firmware_version = firmware_version;
    }

    /// Builder function that sets the [FirmwareVersion] for the [VersionResponse].
    pub fn with_firmware_version(mut self, firmware_version: FirmwareVersion) -> Self {
        self.set_firmware_version(firmware_version);
        self
    }

    /// Gets the metadata length of the [VersionResponse].
    pub const fn meta_len() -> usize {
        ResponseCode::len()
    }

    /// Gets the length of the [VersionResponse].
    pub fn len(&self) -> usize {
        Self::meta_len() + self.firmware_version.len()
    }

    /// Gets whether the [VersionResponse] is empty.
    pub fn is_empty(&self) -> bool {
        self.code.is_empty() && self.firmware_version.is_empty()
    }

    /// Attempts to convert a byte buffer into a [VersionResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let meta_len = Self::meta_len();
        let buf_len = buf.len();

        match buf_len {
            bl if bl < meta_len => Err(Error::InvalidResponseLen((buf_len, meta_len))),
            bl if bl == meta_len => Ok(Self {
                code: buf[0].try_into()?,
                firmware_version: FirmwareVersion::new(),
            }),
            _ => Ok(Self {
                code: buf[0].try_into()?,
                firmware_version: buf[1..].try_into()?,
            }),
        }
    }

    /// Converts the [VersionResponse] into a byte iterator.
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        [self.code.to_u8()]
            .into_iter()
            .chain(self.firmware_version.iter())
    }

    /// Attempts to convert the [VersionResponse] into a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidVersionResponseLen((buf_len, len)))
        } else {
            buf.iter_mut()
                .take(len)
                .zip(self.iter())
                .for_each(|(dst, src)| *dst = src);
            Ok(())
        }
    }

    /// Converts the [VersionResponse] into a byte vector.
    pub fn into_bytes(&self) -> Vec<u8> {
        self.iter().collect()
    }
}

impl TryFrom<&[u8]> for VersionResponse {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::from_bytes(val)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for VersionResponse {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        Self::from_bytes(val.as_ref())
    }
}

impl<const N: usize> TryFrom<[u8; N]> for VersionResponse {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        Self::from_bytes(val.as_ref())
    }
}

impl From<&VersionResponse> for Response {
    fn from(val: &VersionResponse) -> Self {
        Self {
            code: val.code,
            additional: val.firmware_version.into_bytes(),
        }
    }
}

impl From<VersionResponse> for Response {
    fn from(val: VersionResponse) -> Self {
        (&val).into()
    }
}

impl TryFrom<&Response> for VersionResponse {
    type Error = Error;

    fn try_from(val: &Response) -> Result<Self> {
        match val.additional().len() {
            0 => Ok(Self {
                code: val.code,
                firmware_version: FirmwareVersion::new(),
            }),
            _ => Ok(Self {
                code: val.code,
                firmware_version: val.additional().try_into()?,
            }),
        }
    }
}

impl TryFrom<Response> for VersionResponse {
    type Error = Error;

    fn try_from(val: Response) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&VersionResponse> for Message {
    fn from(val: &VersionResponse) -> Self {
        use crate::{MessageData, VersionRequest};

        MessageData::from(VersionRequest::new())
            .with_additional(val.into_bytes().as_ref())
            .into()
    }
}

impl From<VersionResponse> for Message {
    fn from(val: VersionResponse) -> Self {
        (&val).into()
    }
}

impl TryFrom<&Message> for VersionResponse {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        Response::try_from(val)?.try_into()
    }
}

impl TryFrom<Message> for VersionResponse {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl Default for VersionResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for VersionResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code": {}, "#, self.code())?;
        write!(f, r#""firmware_version": {}"#, self.firmware_version())?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_response() -> Result<()> {
        let raw_code = ResponseCode::Ack as u8;
        let raw_version = b"i(JPY)-100-SS 1 SomeVersion 01-25-01\0";
        let raw: Vec<u8> = [raw_code]
            .into_iter()
            .chain(raw_version.iter().cloned())
            .collect();

        let exp_version = FirmwareVersion::try_from(raw_version.as_ref())?;

        let exp = VersionResponse::new()
            .with_code(ResponseCode::Ack)
            .with_firmware_version(exp_version.clone());

        let res = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(raw_version.as_ref());

        assert_eq!(VersionResponse::from_bytes(raw.as_ref())?, exp);
        assert_eq!(VersionResponse::try_from(&res)?, exp);
        assert_eq!(Response::from(&exp), res);

        let out = exp.into_bytes();
        assert_eq!(out, raw);

        Ok(())
    }

    #[test]
    fn test_version_response_invalid() -> Result<()> {
        let raw_code = ResponseCode::Ack as u8;
        let good_vers = FirmwareVersion::from_bytes(b"i(JPY)-100-SS 1 SomeVersion 01-25-01\0")?;
        let bad_vers = [
            // no date
            b"i(JPY)-100-SS 1 SomeVersion\0".as_ref(),
            // no date and version
            b"i(JPY)-100-SS 1\0".as_ref(),
            // no date, version, and interface number
            b"i(JPY)-100-SS\0".as_ref(),
            // no date, version, interface number, and name
            b"\0".as_ref(),
        ];

        let exp = VersionResponse::new()
            .with_code(ResponseCode::Ack)
            .with_firmware_version(good_vers.clone());
        let exp_len = exp.len();
        let mut out = vec![0u8; exp_len];

        for ver in bad_vers.into_iter() {
            let raw: Vec<u8> = [raw_code].into_iter().chain(ver.iter().cloned()).collect();
            assert!(VersionResponse::from_bytes(raw.as_ref()).is_err());
            assert!(exp.to_bytes(out[..raw.len()].as_mut()).is_err());
            assert!(exp.to_bytes(&mut []).is_err());
            assert!(VersionResponse::try_from(Response::new().with_additional(ver)).is_err());
            assert!(VersionResponse::try_from(
                Response::new()
                    .with_code(ResponseCode::Ack)
                    .with_additional(ver)
            )
            .is_err());
        }

        Ok(())
    }
}
