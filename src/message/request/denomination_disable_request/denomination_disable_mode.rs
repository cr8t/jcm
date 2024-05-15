use crate::{Error, RequestType, Result};

/// Represents the request mode for the [DenominationDisableRequest].
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DenominationDisableMode {
    Get = 0,
    Set = 1,
    Reserved = 0xff,
}

impl DenominationDisableMode {
    /// Creates a new [DenominationDisableMode].
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

impl TryFrom<DenominationDisableMode> for RequestType {
    type Error = Error;

    fn try_from(val: DenominationDisableMode) -> Result<Self> {
        match val.to_request_type() {
            Self::Reserved => Err(Error::InvalidRequestType(Self::Reserved.to_u8())),
            req => Ok(req),
        }
    }
}

impl TryFrom<&DenominationDisableMode> for RequestType {
    type Error = Error;

    fn try_from(val: &DenominationDisableMode) -> Result<Self> {
        (*val).try_into()
    }
}

impl TryFrom<RequestType> for DenominationDisableMode {
    type Error = Error;

    fn try_from(val: RequestType) -> Result<Self> {
        match Self::from_request_type(val) {
            Self::Reserved => Err(Error::InvalidRequestType(val.to_u8())),
            mode => Ok(mode),
        }
    }
}

impl TryFrom<&RequestType> for DenominationDisableMode {
    type Error = Error;

    fn try_from(val: &RequestType) -> Result<Self> {
        (*val).try_into()
    }
}

impl Default for DenominationDisableMode {
    fn default() -> Self {
        Self::new()
    }
}
