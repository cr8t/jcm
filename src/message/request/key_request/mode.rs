use crate::{Error, MessageType, RequestType, Result};

/// Represents the request mode for [KeyRequest](super::KeyRequest).
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyMode {
    Get = 0,
    Set = 1,
}

impl KeyMode {
    /// Creates a new [KeyMode].
    pub const fn new() -> Self {
        Self::Get
    }

    /// Attempts to convert a [RequestType] into a [KeyMode].
    pub const fn from_request_type(code: RequestType) -> Option<Self> {
        match code {
            RequestType::Status => Some(Self::Get),
            RequestType::SetFeature => Some(Self::Set),
            _ => None,
        }
    }

    /// Converts a [KeyMode] into a [RequestType].
    pub const fn into_request_type(self) -> RequestType {
        match self {
            Self::Get => RequestType::Status,
            Self::Set => RequestType::SetFeature,
        }
    }

    /// Attempts to convert a [MessageType] into a [KeyMode].
    pub const fn from_message_type(code: MessageType) -> Option<Self> {
        match code {
            MessageType::Request(c) => Self::from_request_type(c),
            _ => None,
        }
    }

    /// Converts a [KeyMode] into a [MessageType].
    pub const fn into_message_type(self) -> MessageType {
        MessageType::Request(self.into_request_type())
    }
}

impl Default for KeyMode {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<RequestType> for KeyMode {
    type Error = Error;

    fn try_from(val: RequestType) -> Result<Self> {
        Self::from_request_type(val).ok_or(Error::InvalidRequestType(val.into()))
    }
}

impl From<KeyMode> for RequestType {
    fn from(val: KeyMode) -> Self {
        val.into_request_type()
    }
}

impl TryFrom<MessageType> for KeyMode {
    type Error = Error;

    fn try_from(val: MessageType) -> Result<Self> {
        Self::from_message_type(val).ok_or(Error::InvalidMessageType(val.into()))
    }
}

impl From<KeyMode> for MessageType {
    fn from(val: KeyMode) -> Self {
        val.into_message_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_mode() {
        [RequestType::Status, RequestType::SetFeature]
            .into_iter()
            .zip([KeyMode::Get, KeyMode::Set])
            .for_each(|(req, exp)| {
                assert_eq!(KeyMode::try_from(req), Ok(exp));
                assert_eq!(KeyMode::try_from(MessageType::Request(req)), Ok(exp));

                assert_eq!(RequestType::from(exp), req);
                assert_eq!(MessageType::from(exp), MessageType::Request(req));
            });
    }

    #[test]
    fn test_key_mode_invalid() {
        let req = RequestType::Operation;

        assert_eq!(
            KeyMode::try_from(req),
            Err(Error::InvalidRequestType(req.into()))
        );

        assert_eq!(
            KeyMode::try_from(MessageType::Request(req)),
            Err(Error::InvalidMessageType(req.into()))
        );
    }
}
