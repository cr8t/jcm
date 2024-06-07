use std::fmt;

use crate::{Error, MessageType, RequestType, Result};

/// Represents the [RequestType] modes for the [NearFullRequest].
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NearFullMode {
    Get,
    Set,
}

impl NearFullMode {
    /// Creates a new [NearFullMode].
    pub const fn new() -> Self {
        Self::Get
    }

    /// Converts a [RequestType] into a [NearFullMode].
    pub const fn from_request_type(val: RequestType) -> Option<Self> {
        match val {
            RequestType::Status => Some(Self::Get),
            RequestType::SetFeature => Some(Self::Set),
            _ => None,
        }
    }

    /// Converts a [NearFullMode] into a [RequestType].
    pub const fn into_request_type(self) -> RequestType {
        match self {
            Self::Get => RequestType::Status,
            Self::Set => RequestType::SetFeature,
        }
    }
}

impl Default for NearFullMode {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<RequestType> for NearFullMode {
    type Error = Error;

    fn try_from(val: RequestType) -> Result<Self> {
        Self::from_request_type(val).ok_or(Error::InvalidNearFullMode(val.into()))
    }
}

impl From<NearFullMode> for RequestType {
    fn from(val: NearFullMode) -> Self {
        val.into_request_type()
    }
}

impl From<NearFullMode> for MessageType {
    fn from(val: NearFullMode) -> Self {
        MessageType::Request(val.into_request_type())
    }
}

impl From<NearFullMode> for &'static str {
    fn from(val: NearFullMode) -> Self {
        match val {
            NearFullMode::Get => "get",
            NearFullMode::Set => "set",
        }
    }
}

impl From<&NearFullMode> for &'static str {
    fn from(val: &NearFullMode) -> Self {
        (*val).into()
    }
}

impl fmt::Display for NearFullMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
