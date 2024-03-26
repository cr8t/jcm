use std::{fmt, mem};

use crate::{Error, Result};

mod conf_id;
mod func_id;
mod message_data;
mod message_id;
mod message_type;

pub use conf_id::*;
pub use func_id::*;
pub use message_data::*;
pub use message_id::*;
pub use message_type::*;

/// Maximum length of the [Message].
pub const MAX_LEN: usize = u16::MAX as usize;
/// Minimum length of the [Message].
pub const MIN_LEN: usize = Message::meta_len() + MessageData::meta_len();

/// Represents the generic message format for JCM host-device communication.
///
/// Message format:
///
/// Field name  | ID | Length | Data
/// ------------|----|--------|---------
/// Size (byte) | 1  | 2      | Variable
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    id: MessageId,
    data: MessageData,
}

impl Message {
    /// Creates a new [Message].
    pub const fn new() -> Self {
        Self {
            id: MessageId::Message,
            data: MessageData::new(),
        }
    }

    /// Gets the [MessageId] of the [Message].
    pub const fn id(&self) -> MessageId {
        self.id
    }

    /// Gets a reference to the [MessageData] of the [Message].
    pub const fn data(&self) -> &MessageData {
        &self.data
    }

    /// Sets the [MessageData] of the [Message].
    pub fn set_data(&mut self, data: MessageData) {
        self.data = data;
    }

    /// Builder function that sets the [MessageData] of the [Message].
    pub fn with_data(mut self, data: MessageData) -> Self {
        self.set_data(data);
        self
    }

    /// Gets the length of the [Message].
    pub fn len(&self) -> usize {
        Self::meta_len() + self.data.len()
    }

    pub(crate) const fn meta_len() -> usize {
        MessageId::len() + mem::size_of::<u16>()
    }

    /// Gets whether the [Message] is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl From<&Message> for Vec<u8> {
    fn from(val: &Message) -> Self {
        let len = (val.data.len() as u16).to_be_bytes();

        [val.id as u8, len[0], len[1]]
            .into_iter()
            .chain(Vec::<u8>::from(val.data()))
            .collect()
    }
}

impl From<Message> for Vec<u8> {
    fn from(val: Message) -> Self {
        let len = (val.data.len() as u16).to_be_bytes();

        [val.id as u8, len[0], len[1]]
            .into_iter()
            .chain(Vec::<u8>::from(val.data))
            .collect()
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        let len = val.len();
        if len < MIN_LEN {
            Err(Error::InvalidMessageLen((len, MIN_LEN)))
        } else {
            let id = MessageId::try_from(val[0])?;

            let data_len = u16::from_be_bytes([val[1], val[2]]) as usize;
            if data_len > len {
                Err(Error::InvalidMessageDataLen((
                    data_len,
                    len - Self::meta_len(),
                )))
            } else {
                let data = MessageData::try_from(&val[3..(3 + data_len)])?;

                Ok(Self { id, data })
            }
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""id": {},"#, self.id)?;
        write!(f, r#""data": {}"#, self.data)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_message() -> Result<()> {
        let raw: [u8; 7] = [
            // message ID
            0x12,
            // length
            0x00, 0x04,
            // message data
            //     conf ID
            0x10,
            //     UID
            0x00,
            //     message type
            0x00,
            //     func ID
            0x00,
            //     additional data (none)
        ];

        let exp = Message::new();
        let msg = Message::try_from(raw.as_ref())?;

        assert_eq!(msg, exp);

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_message_with_data() -> Result<()> {
        let raw: [u8; 15] = [
            // message ID
            0x12,
            // length
            0x00, 0x0c,
            // message data
            //     conf ID
            0x10,
            //     UID
            0x00,
            //     message type
            0x00,
            //     func ID
            0x00,
            //     additional data
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];

        let exp = Message::new().with_data(MessageData::new().with_additional(&raw[7..]));
        let msg = Message::try_from(raw.as_ref())?;

        assert_eq!(msg, exp);

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_message_with_invalid_data() -> Result<()> {
        let raw: [u8; 15] = [
            // message ID
            0x12,
            // length - longer than the raw message buffer
            0x00, 0xff,
            // message data
            //     conf ID
            0x10,
            //     UID
            0x00,
            //     message type
            0x00,
            //     func ID
            0x00,
            //     additional data
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];

        assert!(Message::try_from(raw.as_ref()).is_err());

        Ok(())
    }
}
