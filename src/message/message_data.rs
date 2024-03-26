use std::{cmp, fmt, mem};

use super::{ConfId, FuncId, MessageType, MAX_LEN};
use crate::{Error, Result};

/// Represents message data for JCM host-device communication.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageData {
    conf_id: ConfId,
    uid: u8,
    message_type: MessageType,
    func_id: FuncId,
    additional: Vec<u8>,
}

impl MessageData {
    /// Creates a new [MessageData].
    pub const fn new() -> Self {
        Self {
            conf_id: ConfId::new(),
            uid: 0,
            message_type: MessageType::new(),
            func_id: FuncId::new(),
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

    /// Gets the [FuncId] of the [MessageData].
    pub const fn func_id(&self) -> FuncId {
        self.func_id
    }

    /// Sets the [FuncId] of the [MessageData].
    pub fn set_func_id(&mut self, val: FuncId) {
        self.func_id = val;
    }

    /// Builder function that sets the [FuncId] of the [MessageData].
    pub fn with_func_id(mut self, val: FuncId) -> Self {
        self.set_func_id(val);
        self
    }

    /// Gets a reference to the additional data of the [MessageData].
    pub fn additional(&self) -> &[u8] {
        &self.additional
    }

    /// Sets the additional data of the [MessageData].
    pub fn set_additional(&mut self, additional: &[u8]) {
        let len = cmp::min(additional.len(), MAX_LEN);
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
        ConfId::len() + mem::size_of::<u8>() + MessageType::len() + FuncId::len()
    }

    /// Gets whether the [MessageData] is empty.
    pub fn is_empty(&self) -> bool {
        self.additional.is_empty()
    }
}

impl From<&MessageData> for Vec<u8> {
    fn from(val: &MessageData) -> Self {
        [
            val.conf_id as u8,
            val.uid,
            val.message_type.into(),
            val.func_id as u8,
        ]
        .into_iter()
        .chain(val.additional.iter().cloned())
        .collect()
    }
}

impl From<MessageData> for Vec<u8> {
    fn from(val: MessageData) -> Self {
        [
            val.conf_id as u8,
            val.uid,
            val.message_type.into(),
            val.func_id as u8,
        ]
        .into_iter()
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
            let func_id = FuncId::try_from(val[3])?;
            let additional = val[4..].into();

            Ok(Self {
                conf_id,
                uid,
                message_type,
                func_id,
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
        write!(f, r#""func_id": {},"#, self.func_id)?;
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
        let raw: [u8; 4] = [
            // conf ID
            0x10,
            // UID
            0x00,
            // message type
            0x00,
            // func ID
            0x00,
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
        let raw: [u8; 12] = [
            // conf ID
            0x10,
            // UID
            0x00,
            // message type
            0x00,
            // func ID
            0x00,
            // additional data
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];

        let exp = MessageData::new().with_additional(&raw[4..]);
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
            // func ID
            0x00,
        ].into_iter().chain([0xff; MAX_LEN]).collect();

        assert!(MessageData::try_from(raw.as_ref()).is_err());

        Ok(())
    }
}
