use std::fmt;

use crate::{
    Error, KeySetting, KeySettingList, Message, RequestCode, Response, ResponseCode, Result,
};

/// Represents the [Response] to a UID request [Message](crate::Message).
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyResponse {
    code: ResponseCode,
    settings: KeySettingList,
}

impl KeyResponse {
    /// Creates a new [KeyResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            settings: KeySettingList::new(),
        }
    }

    /// Gets the [ResponseCode] for the [KeyResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [KeyResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [KeyResponse].
    pub fn with_code(self, code: ResponseCode) -> Self {
        Self {
            code,
            settings: self.settings,
        }
    }

    /// Gets a reference to the [KeySetting] list for the [KeyResponse].
    pub fn settings(&self) -> &[KeySetting] {
        self.settings.items()
    }

    /// Sets the [KeySetting] list for the [KeyResponse].
    pub fn set_settings(&mut self, settings: &[KeySetting]) {
        self.settings = settings.into();
    }

    /// Builder function that sets the [KeySetting] list for the [KeyResponse].
    pub fn with_settings(self, settings: &[KeySetting]) -> Self {
        Self {
            code: self.code,
            settings: settings.into(),
        }
    }

    /// Builder function that sets the [KeySetting] list for the [KeyResponse] in-place.
    pub fn emplace_settings(self, settings: KeySettingList) -> Self {
        Self {
            code: self.code,
            settings,
        }
    }

    /// Gets the length of the [KeyResponse].
    pub fn len(&self) -> usize {
        ResponseCode::len() + self.settings.len()
    }

    /// Gets whether the [KeyResponse] is empty.
    pub fn is_empty(&self) -> bool {
        self.code.is_empty() && self.settings.is_empty()
    }

    /// Converts a [KeyResponse] into a byte buffer.
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
                        .chain(self.settings.iter_bytes()),
                )
                .for_each(|(dst, src)| *dst = src);

            Ok(())
        }
    }

    /// Converts a byte buffer into a [KeyResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        Ok(Self {
            code: buf
                .first()
                .copied()
                .ok_or(Error::InvalidResponseLen((0, 1)))?
                .try_into()?,
            settings: buf.get(1..).unwrap_or_default().try_into()?,
        })
    }
}

impl Default for KeyResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&Response> for KeyResponse {
    type Error = Error;

    fn try_from(val: &Response) -> Result<Self> {
        Ok(Self {
            code: val.code,
            settings: val.additional().try_into()?,
        })
    }
}

impl TryFrom<Response> for KeyResponse {
    type Error = Error;

    fn try_from(val: Response) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&KeyResponse> for Response {
    fn from(val: &KeyResponse) -> Self {
        Self {
            code: val.code,
            additional: val.settings.iter_bytes().collect(),
        }
    }
}

impl From<KeyResponse> for Response {
    fn from(val: KeyResponse) -> Self {
        (&val).into()
    }
}

impl TryFrom<Message> for KeyResponse {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&Message> for KeyResponse {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        match val.data.message_code().request_code()? {
            RequestCode::Key => Response::try_from(val)?.try_into(),
            code => Err(Error::InvalidRequestCode(code.into())),
        }
    }
}

impl fmt::Display for KeyResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code": {}, "#, self.code)?;
        write!(f, r#""settings": {}"#, self.settings)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_response() {
        let raw = [ResponseCode::Ack as u8, 0];
        let exp = KeyResponse::new()
            .with_code(ResponseCode::Ack)
            .emplace_settings(KeySettingList::create(vec![KeySetting::Disabled]));
        let res = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(&[0]);
        let mut out = [0u8; 2];

        assert_eq!(KeyResponse::from_bytes(raw.as_ref()).as_ref(), Ok(&exp));
        assert_eq!(KeyResponse::try_from(&res).as_ref(), Ok(&exp));
        assert_eq!(Response::from(&exp), res);

        assert!(exp.to_bytes(out.as_mut()).is_ok());

        assert_eq!(out, raw);
    }

    #[test]
    fn test_key_response_invalid() {
        let exp = KeyResponse::new()
            .with_code(ResponseCode::Ack)
            .with_settings(KeySettingList::create(vec![KeySetting::Disabled]).items());

        assert!(KeyResponse::from_bytes([ResponseCode::Reserved as u8, 0, 1, 2].as_ref()).is_err());
        assert!(exp.to_bytes(&mut []).is_err());
    }
}
