use crate::{Error, RequestType, Result};

/// Represents the request mode for the [DirectionDisableRequest].
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectionDisableMode {
    Get = 0,
    Set = 1,
    Reserved = 0xff,
}

impl DirectionDisableMode {
    /// Creates a new [DirectionDisableMode].
    pub const fn new() -> Self {
        Self::Get
    }

    pub const fn from_request_type(val: RequestType) -> Self {
        match val {
            RequestType::Status => Self::Get,
            RequestType::SetFeature => Self::Set,
            _ => Self::Reserved,
        }
    }

    pub const fn to_request_type(&self) -> RequestType {
        match self {
            Self::Get => RequestType::Status,
            Self::Set => RequestType::SetFeature,
            Self::Reserved => RequestType::Reserved,
        }
    }
}

impl TryFrom<DirectionDisableMode> for RequestType {
    type Error = Error;

    fn try_from(val: DirectionDisableMode) -> Result<Self> {
        match val.to_request_type() {
            Self::Reserved => Err(Error::InvalidRequestType(Self::Reserved.to_u8())),
            req => Ok(req),
        }
    }
}

impl TryFrom<&DirectionDisableMode> for RequestType {
    type Error = Error;

    fn try_from(val: &DirectionDisableMode) -> Result<Self> {
        (*val).try_into()
    }
}

impl TryFrom<RequestType> for DirectionDisableMode {
    type Error = Error;

    fn try_from(val: RequestType) -> Result<Self> {
        match Self::from_request_type(val) {
            Self::Reserved => Err(Error::InvalidRequestType(val.to_u8())),
            mode => Ok(mode),
        }
    }
}

impl TryFrom<&RequestType> for DirectionDisableMode {
    type Error = Error;

    fn try_from(val: &RequestType) -> Result<Self> {
        (*val).try_into()
    }
}

impl Default for DirectionDisableMode {
    fn default() -> Self {
        Self::new()
    }
}
