use std::{cmp, fmt, mem};

use crate::{DeviceStatus, Error, Response, ResponseCode, Result, UnitStatus, UnitStatusList};

/// Maximum number of [UnitStatus] items in a [StatusResponse].
pub const MAX_UNIT_STATUS_LEN: usize = u8::MAX as usize;

/// Represents the [Response] to a UID request [Message](crate::Message).
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusResponse {
    code: ResponseCode,
    status: DeviceStatus,
    unit_status: UnitStatusList,
}

impl StatusResponse {
    /// Creates a new [StatusResponse].
    pub const fn new() -> Self {
        Self {
            code: ResponseCode::new(),
            status: DeviceStatus::new(),
            unit_status: UnitStatusList::new(),
        }
    }

    /// Gets the [ResponseCode] for the [StatusResponse].
    pub const fn code(&self) -> ResponseCode {
        self.code
    }

    /// Sets the [ResponseCode] for the [StatusResponse].
    pub fn set_code(&mut self, code: ResponseCode) {
        self.code = code;
    }

    /// Builder function that sets the [ResponseCode] for the [StatusResponse].
    pub fn with_code(mut self, code: ResponseCode) -> Self {
        self.set_code(code);
        self
    }

    /// Gets the [DeviceStatus] for the [StatusResponse].
    pub const fn status(&self) -> DeviceStatus {
        self.status
    }

    /// Sets the [DeviceStatus] for the [StatusResponse].
    pub fn set_status(&mut self, status: DeviceStatus) {
        self.status = status;
    }

    /// Builder function that sets the [DeviceStatus] for the [StatusResponse].
    pub fn with_status(mut self, status: DeviceStatus) -> Self {
        self.set_status(status);
        self
    }

    /// Gets a reference to the list of [UnitStatus] items.
    pub fn unit_status(&self) -> &[UnitStatus] {
        self.unit_status.items()
    }

    /// Sets the list of [UnitStatus] items.
    ///
    /// **NOTE** A maximum of [MAX_UNIT_STATUS_LEN] items will be set.
    pub fn set_unit_status(&mut self, status: &[UnitStatus]) {
        let len = cmp::min(status.len(), MAX_UNIT_STATUS_LEN);
        self.unit_status = UnitStatusList(status[..len].into());
    }

    /// Builder function that sets the list of [UnitStatus] items.
    ///
    /// **NOTE** A maximum of [MAX_UNIT_STATUS_LEN] items will be set.
    pub fn with_unit_status(mut self, status: &[UnitStatus]) -> Self {
        self.set_unit_status(status);
        self
    }

    /// Gets the length of the [StatusResponse] metadata.
    pub const fn meta_len() -> usize {
        ResponseCode::len() + mem::size_of::<u8>() + DeviceStatus::len()
    }

    /// Gets the full length of the [StatusResponse].
    pub fn len(&self) -> usize {
        Self::meta_len() + self.unit_status.len()
    }

    /// Gets whether the [StatusResponse] is empty.
    pub fn is_empty(&self) -> bool {
        self.code.is_empty() && self.status.is_empty() && self.unit_status.is_empty()
    }

    /// Converts a [StatusResponse] into a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        match buf_len {
            bl if bl < len => Err(Error::InvalidResponseLen((buf_len, len))),
            _ => {
                let unit_len = self.unit_status.len();
                let main_len = len - unit_len;

                buf.iter_mut()
                    .take(main_len)
                    .zip(
                        [self.code.into(), len as u8]
                            .into_iter()
                            .chain(self.status.to_bytes()),
                    )
                    .for_each(|(dst, src)| *dst = src);

                if unit_len > 0 {
                    self.unit_status.to_bytes(buf[main_len..].as_mut())?;
                }

                Ok(())
            }
        }
    }

    /// Converts a byte buffer into a [StatusResponse].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let meta_len = Self::meta_len();
        let buf_len = buf.len();

        match buf_len {
            bl if bl < meta_len => Err(Error::InvalidResponseLen((buf_len, meta_len))),
            bl if bl == meta_len => {
                let status_len = buf[1] as usize;
                if status_len == meta_len {
                    Ok(Self {
                        code: buf[0].try_into()?,
                        status: [buf[2], buf[3]].try_into()?,
                        unit_status: UnitStatusList::new(),
                    })
                } else {
                    Err(Error::InvalidResponseLen((status_len, meta_len)))
                }
            }
            _ => Ok(Self {
                code: buf[0].try_into()?,
                status: [buf[1], buf[2]].try_into()?,
                unit_status: buf[3..].as_ref().try_into()?,
            }),
        }
    }
}

impl Default for StatusResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&Response> for StatusResponse {
    type Error = Error;

    fn try_from(val: &Response) -> Result<Self> {
        let meta_len = Self::meta_len();
        let res_len = val.len();

        match res_len {
            rl if rl < meta_len => Err(Error::InvalidResponseLen((res_len, meta_len))),
            _ => {
                let status_len = val.additional[0] as usize;
                match status_len {
                    sl if sl == meta_len && sl == res_len => Ok(Self {
                        code: val.code,
                        status: val.additional[1..=2].as_ref().try_into()?,
                        unit_status: UnitStatusList::new(),
                    }),
                    sl if sl > meta_len && sl == res_len => Ok(Self {
                        code: val.code,
                        status: val.additional[1..=2].as_ref().try_into()?,
                        unit_status: val.additional[3..].as_ref().try_into()?,
                    }),
                    _ => Err(Error::InvalidResponseLen((status_len, res_len))),
                }
            }
        }
    }
}

impl TryFrom<Response> for StatusResponse {
    type Error = Error;

    fn try_from(val: Response) -> Result<Self> {
        (&val).try_into()
    }
}

impl From<&StatusResponse> for Response {
    fn from(val: &StatusResponse) -> Self {
        Self {
            code: val.code,
            additional: [val.len() as u8]
                .into_iter()
                .chain(val.status.to_bytes())
                .chain(val.unit_status.as_bytes())
                .collect(),
        }
    }
}

impl From<StatusResponse> for Response {
    fn from(val: StatusResponse) -> Self {
        (&val).into()
    }
}

impl fmt::Display for StatusResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""code":{}, "#, self.code)?;
        write!(f, r#""len":{}, "#, self.len())?;
        write!(f, r#""status":{}, "#, self.status)?;
        write!(f, r#""unit_status": {}"#, self.unit_status)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_response() {
        let meta_len = StatusResponse::meta_len() as u8;
        let raw = [ResponseCode::Ack as u8, meta_len, 0, 0];
        let exp = StatusResponse::new().with_code(ResponseCode::Ack);
        let res = Response::new()
            .with_code(ResponseCode::Ack)
            .with_additional(&[meta_len, 0, 0]);
        let mut out = [0u8; 4];

        assert_eq!(StatusResponse::from_bytes(raw.as_ref()).as_ref(), Ok(&exp));
        assert_eq!(StatusResponse::try_from(&res).as_ref(), Ok(&exp));
        assert_eq!(Response::from(&exp), res);

        assert!(exp.to_bytes(out.as_mut()).is_ok());

        assert_eq!(out, raw);
    }

    #[test]
    fn test_status_response_invalid() {
        let raw = [ResponseCode::Ack as u8, 0, 0];
        let exp = StatusResponse::new().with_code(ResponseCode::Ack);
        let mut out = [0u8; 3];

        assert!(StatusResponse::from_bytes(raw[..1].as_ref()).is_err());
        assert!(StatusResponse::from_bytes([ResponseCode::Reserved as u8, 0].as_ref()).is_err());
        assert!(exp.to_bytes(out[..1].as_mut()).is_err());
        assert!(exp.to_bytes(&mut []).is_err());
        assert!(StatusResponse::try_from(Response::new().with_additional(&[])).is_err());
        assert!(StatusResponse::try_from(
            Response::new()
                .with_code(ResponseCode::Ack)
                .with_additional(&[])
        )
        .is_err());
    }
}
