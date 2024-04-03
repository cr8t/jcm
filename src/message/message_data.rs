use std::{cmp, fmt, mem};

use super::{ConfId, MessageCode, MessageType, RawMessageCode, MAX_LEN};
use crate::{Error, Result};

const MAX_DATA_LEN: usize = MAX_LEN - MessageData::meta_len();

/// Represents message data for JCM host-device communication.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageData {
    conf_id: ConfId,
    uid: u8,
    message_type: MessageType,
    message_code: MessageCode,
    additional: Vec<u8>,
}

impl MessageData {
    /// Creates a new [MessageData].
    pub const fn new() -> Self {
        Self {
            conf_id: ConfId::new(),
            uid: 0,
            message_type: MessageType::new(),
            message_code: MessageCode::new(),
            additional: Vec::new(),
        }
    }

    /// Gets the [ConfId] of the [MessageData].
    pub const fn conf_id(&self) -> ConfId {
        self.conf_id
    }

    /// Sets the [ConfId] of the [MessageData].
    pub fn set_conf_id(&mut self, val: ConfId) {
        self.conf_id = val;
    }

    /// Builder function that sets the [ConfId] of the [MessageData].
    pub fn with_conf_id(mut self, val: ConfId) -> Self {
        self.set_conf_id(val);
        self
    }

    /// Gets the UID of the [MessageData].
    ///
    /// - `0`: device powered on, and/or USB cable disconnected.
    /// - `1-255`: device ID for disconnected device to send in a UID Request Message.
    pub const fn uid(&self) -> u8 {
        self.uid
    }

    /// Sets the UID of the [MessageData].
    pub fn set_uid(&mut self, val: u8) {
        self.uid = val;
    }

    /// Builder function that sets the UID of the [MessageData].
    pub fn with_uid(mut self, val: u8) -> Self {
        self.set_uid(val);
        self
    }

    /// Gets the [MessageType] of the [MessageData].
    pub const fn message_type(&self) -> MessageType {
        self.message_type
    }

    /// Sets the [MessageType] of the [MessageData].
    pub fn set_message_type(&mut self, val: MessageType) {
        self.message_type = val;
    }

    /// Builder function that sets the [MessageType] of the [MessageData].
    pub fn with_message_type(mut self, val: MessageType) -> Self {
        self.set_message_type(val);
        self
    }

    /// Gets the [MessageCode] of the [MessageData].
    pub const fn message_code(&self) -> MessageCode {
        self.message_code
    }

    /// Sets the [MessageCode] of the [MessageData].
    pub fn set_message_code(&mut self, val: MessageCode) {
        self.message_code = val;
    }

    /// Builder function that sets the [MessageCode] of the [MessageData].
    pub fn with_message_code(mut self, val: MessageCode) -> Self {
        self.set_message_code(val);
        self
    }

    /// Gets a reference to the additional data of the [MessageData].
    pub fn additional(&self) -> &[u8] {
        &self.additional
    }

    /// Sets the additional data of the [MessageData].
    pub fn set_additional(&mut self, additional: &[u8]) {
        let len = cmp::min(additional.len(), MAX_DATA_LEN);
        self.additional = additional[..len].into()
    }

    /// Builder function that sets the additional data of the [MessageData].
    pub fn with_additional(mut self, additional: &[u8]) -> Self {
        self.set_additional(additional);
        self
    }

    /// Gets the length of the [MessageData].
    pub fn len(&self) -> usize {
        Self::meta_len() + self.additional.len()
    }

    pub(crate) const fn meta_len() -> usize {
        ConfId::len() + mem::size_of::<u8>() + MessageType::len() + MessageCode::len()
    }

    /// Gets whether the [MessageData] is empty.
    pub fn is_empty(&self) -> bool {
        self.conf_id.is_empty()
            || self.message_type.is_empty()
            || self.message_code.is_empty()
            || self.message_code.func_id().is_empty()
            || self.additional.is_empty()
    }

    /// Writes the [MessageData] to the provided byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidMessageDataLen((buf_len, len)))
        } else {
            buf.iter_mut()
                .take(len)
                .zip(
                    [self.conf_id.into(), self.uid, self.message_type.into()]
                        .into_iter()
                        .chain(self.message_code.to_bytes())
                        .chain(self.additional.iter().cloned()),
                )
                .for_each(|(dst, src)| *dst = src);

            Ok(())
        }
    }
}

impl From<&MessageData> for Vec<u8> {
    fn from(val: &MessageData) -> Self {
        [val.conf_id.into(), val.uid, val.message_type.into()]
            .into_iter()
            .chain(val.message_code.to_bytes())
            .chain(val.additional.iter().cloned())
            .collect()
    }
}

impl From<MessageData> for Vec<u8> {
    fn from(val: MessageData) -> Self {
        [val.conf_id.into(), val.uid, val.message_type.into()]
            .into_iter()
            .chain(val.message_code.to_bytes())
            .chain(val.additional)
            .collect()
    }
}

impl TryFrom<&[u8]> for MessageData {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        let len = val.len();
        let meta_len = Self::meta_len();

        if len < meta_len {
            Err(Error::InvalidMessageDataLen((len, meta_len)))
        } else if len > MAX_LEN {
            Err(Error::InvalidMessageDataLen((len, MAX_LEN)))
        } else {
            let conf_id = ConfId::try_from(val[0])?;
            let uid = val[1];
            let message_type = MessageType::try_from(val[2])?;
            let message_code = MessageCode::try_from(RawMessageCode::create(
                message_type,
                u16::from_le_bytes([val[3], val[4]]),
            ))?;
            let additional = val[5..].into();

            Ok(Self {
                conf_id,
                uid,
                message_type,
                message_code,
                additional,
            })
        }
    }
}

impl fmt::Display for MessageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""conf_id": {},"#, self.conf_id)?;
        write!(f, r#""uid": {},"#, self.uid)?;
        write!(f, r#""message_type": {},"#, self.message_type)?;
        write!(f, r#""message_code": {},"#, self.message_code)?;
        write!(f, r#""additional": ["#)?;
        for (i, d) in self.additional.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{d}")?;
        }
        write!(f, "]}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_message_data() -> Result<()> {
        let raw: [u8; 5] = [
            // conf ID
            0x10,
            // UID
            0x00,
            // message type
            0x00,
            // func ID + request/event code
            0x01, 0x00,
            // additional data (none)
        ];

        let exp = MessageData::new();
        let msg = MessageData::try_from(raw.as_ref())?;

        assert_eq!(msg, exp);

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_message_data_with_additional() -> Result<()> {
        let raw: [u8; 13] = [
            // conf ID
            0x10,
            // UID
            0x00,
            // message type
            0x00,
            // func ID + request/event code
            0x01, 0x00,
            // additional data
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];

        let exp = MessageData::new().with_additional(&raw[5..]);
        let msg = MessageData::try_from(raw.as_ref())?;

        assert_eq!(msg, exp);

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_message_data_too_short() -> Result<()> {
        let raw: [u8; 3] = [
            // conf ID
            0x10,
            // UID
            0x00,
            // message type
            0x00,
        ];

        assert!(MessageData::try_from(raw.as_ref()).is_err());

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_message_data_too_long() -> Result<()> {
        let raw: Vec<u8> = [
            // conf ID
            0x10,
            // UID
            0x00,
            // message type
            0x00,
            // func ID + request/event code
            0x01, 0x00,
        ].into_iter().chain([0xff; MAX_LEN]).collect();

        assert!(MessageData::try_from(raw.as_ref()).is_err());

        Ok(())
    }
}
