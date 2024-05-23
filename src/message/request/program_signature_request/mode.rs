use crate::{Error, MessageType, RequestType, Result};

/// Represents the [ProgramSignatureRequest](super::ProgramSignatureRequest) request mode.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgramSignatureMode {
    /// Get the `Program Signature` hash algorithm.
    Get = 0,
    /// Set the `Program Signature` hash algorithm and default value.
    Set = 1,
}

impl ProgramSignatureMode {
    /// Creates a new [ProgramSignatureMode].
    pub const fn new() -> Self {
        Self::Get
    }

    /// Converts the [ProgramSignatureMode] into a [RequestType].
    pub const fn into_request_type(self) -> RequestType {
        match self {
            Self::Get => RequestType::Status,
            Self::Set => RequestType::Operation,
        }
    }

    /// Converts a [RequestType] into a [ProgramSignatureMode].
    pub const fn from_request_type(val: RequestType) -> Result<Self> {
        match val {
            RequestType::Status => Ok(Self::Get),
            RequestType::Operation => Ok(Self::Set),
            ty => Err(Error::InvalidRequestType(ty.to_u8())),
        }
    }
}

impl Default for ProgramSignatureMode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ProgramSignatureMode> for MessageType {
    fn from(val: ProgramSignatureMode) -> Self {
        Self::Request(val.into())
    }
}

impl From<ProgramSignatureMode> for RequestType {
    fn from(val: ProgramSignatureMode) -> Self {
        val.into_request_type()
    }
}

impl TryFrom<MessageType> for ProgramSignatureMode {
    type Error = Error;

    fn try_from(val: MessageType) -> Result<Self> {
        val.request_type()?.try_into()
    }
}

impl TryFrom<RequestType> for ProgramSignatureMode {
    type Error = Error;

    fn try_from(val: RequestType) -> Result<Self> {
        Self::from_request_type(val)
    }
}
