use crate::{Error, MessageCode, RequestCode, Result};

/// Represents the device mode for collecting notes.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollectMode {
    /// Collect notes left in device at `PowerUp`.
    PowerUp = 0,
    /// Collect notes from the acceptor unit.
    Acceptor = 1,
    /// Collect notes from the recycler unit.
    Recycler = 2,
    /// Reserved (invalid).
    Reserved = 0xff,
}

impl CollectMode {
    /// Creates a new [CollectMode].
    pub const fn new() -> Self {
        Self::PowerUp
    }

    /// Infallible conversion from a [RequestCode] into a [CollectMode].
    pub const fn from_request_code(val: RequestCode) -> Self {
        match val {
            RequestCode::Collect => Self::PowerUp,
            RequestCode::AcceptorCollect => Self::Acceptor,
            RequestCode::RecyclerCollect => Self::Recycler,
            _ => Self::Reserved,
        }
    }

    /// Converts the [CollectMode] into a [RequestCode].
    pub const fn to_request_code(&self) -> RequestCode {
        match self {
            Self::PowerUp => RequestCode::Collect,
            Self::Acceptor => RequestCode::AcceptorCollect,
            Self::Recycler => RequestCode::RecyclerCollect,
            Self::Reserved => RequestCode::Reserved,
        }
    }
}

impl Default for CollectMode {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<RequestCode> for CollectMode {
    type Error = Error;

    fn try_from(val: RequestCode) -> Result<Self> {
        match Self::from_request_code(val) {
            Self::Reserved => Err(Error::InvalidRequestCode(val.into())),
            mode => Ok(mode),
        }
    }
}

impl TryFrom<&RequestCode> for CollectMode {
    type Error = Error;

    fn try_from(val: &RequestCode) -> Result<Self> {
        (*val).try_into()
    }
}

impl TryFrom<MessageCode> for CollectMode {
    type Error = Error;

    fn try_from(val: MessageCode) -> Result<Self> {
        match Self::from_request_code(val.request_code()?) {
            Self::Reserved => Err(Error::InvalidMessageCode((
                val.into(),
                RequestCode::Collect.into(),
            ))),
            mode => Ok(mode),
        }
    }
}

impl TryFrom<&MessageCode> for CollectMode {
    type Error = Error;

    fn try_from(val: &MessageCode) -> Result<Self> {
        (*val).try_into()
    }
}
