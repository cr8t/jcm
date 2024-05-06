use std::{fmt, mem};

use crate::{Error, Result};

/// Represents the maximum length of a [Ticket] ASCII code.
pub const MAX_TICKET_LEN: usize = u8::MAX as usize;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TicketMetadata {
    ticket_len: u8,
}

impl TicketMetadata {
    /// Creates a new [TicketMetadata].
    pub const fn new() -> Self {
        Self { ticket_len: 0 }
    }

    /// Creates a new [TicketMetadata] from the provided parameter.
    pub const fn create(len: usize) -> Self {
        Self {
            ticket_len: len as u8,
        }
    }

    /// Gets the length of the [TicketMetadata].
    pub const fn len() -> usize {
        mem::size_of::<u16>() + mem::size_of::<u8>()
    }

    /// Get the length of the [Ticket] for this [TicketMetadata].
    pub const fn ticket_len(&self) -> u8 {
        self.ticket_len
    }

    /// Gets whether the [TicketMetadata] is empty.
    pub const fn is_empty(&self) -> bool {
        self.ticket_len == 0
    }

    /// Converts the [TicketMetadata] to a byte array.
    pub fn to_bytes(self) -> [u8; 3] {
        [0, 0, self.ticket_len]
    }
}

impl Default for TicketMetadata {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a `ticket` handled by the JCM device.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ticket {
    code: String,
}

impl Ticket {
    /// Creates a new [Ticket].
    pub const fn new() -> Self {
        Self {
            code: String::new(),
        }
    }

    /// Gets the ASCII code of the [Ticket].
    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    /// Sets the ASCII code of the [Ticket].
    ///
    /// `code` length must not exceed [MAX_LEN].
    pub fn set_code(&mut self, code: &str) -> Result<()> {
        if code.as_bytes().iter().any(|b| !b.is_ascii()) {
            Err(Error::InvalidAsciiString)
        } else {
            match code.len() {
                l if l <= MAX_TICKET_LEN => {
                    self.code = code.into();
                    Ok(())
                }
                l => Err(Error::InvalidTicketLen((l, MAX_TICKET_LEN))),
            }
        }
    }

    /// Builder function that sets the ASCII code of the [Ticket].
    ///
    /// `code` length must not exceed [MAX_LEN].
    pub fn with_code(mut self, code: &str) -> Result<Self> {
        self.set_code(code)?;
        Ok(self)
    }

    /// Gets the [TicketMetadata] for the [Ticket].
    pub fn metadata(&self) -> TicketMetadata {
        TicketMetadata::create(self.code.as_bytes().len())
    }

    /// Gets the length of the metadata + ASCII code of the [Ticket].
    pub fn len(&self) -> usize {
        TicketMetadata::len() + self.code.as_bytes().len()
    }

    /// Gets whether the ASCII code of the [Ticket] is empty.
    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    /// Attempts to convert a byte buffer into a [Ticket].
    ///
    /// The buffer should only contain the ASCII-encode portion of the ticket.
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        Self::new().with_code(std::str::from_utf8(buf).map_err(|_| Error::InvalidUtf8String)?)
    }

    /// Writes the [Ticket] to a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidTicketLen((buf_len, len)))
        } else {
            let ticket_iter = self
                .metadata()
                .to_bytes()
                .into_iter()
                .chain(self.code.as_bytes().iter().cloned());

            buf.iter_mut()
                .zip(ticket_iter)
                .for_each(|(dst, src)| *dst = src);

            Ok(())
        }
    }

    /// Converts the [Ticket] into byte vector.
    pub fn to_vec(&self) -> Vec<u8> {
        let mut ret = vec![0u8; self.len()];
        self.to_bytes(ret.as_mut()).ok();
        ret
    }
}

impl From<&Ticket> for Vec<u8> {
    fn from(val: &Ticket) -> Self {
        val.to_vec()
    }
}

impl From<Ticket> for Vec<u8> {
    fn from(val: Ticket) -> Self {
        val.to_vec()
    }
}

impl TryFrom<&[u8]> for Ticket {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::from_bytes(val)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for Ticket {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for Ticket {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl Default for Ticket {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, self.code())
    }
}
