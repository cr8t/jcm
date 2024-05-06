use crate::{Currency, Error, Result, Ticket};

/// Represents the minimum byte length of [EscrowData].
pub const MIN_ESCROW_DATA_LEN: usize = 3;

/// Represents the additional data of an [EscrowEvent].
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum EscrowData {
    Currency(Currency),
    Ticket(Ticket),
}

impl EscrowData {
    /// Creates a new [EscrowData].
    pub const fn new() -> Self {
        Self::Currency(Currency::new())
    }

    /// Creates a new [EscrowData] with a [Currency] variant.
    pub const fn new_currency(currency: Currency) -> Self {
        Self::Currency(currency)
    }

    /// Creates a new [EscrowData] with a [Ticket] variant.
    pub const fn new_ticket(ticket: Ticket) -> Self {
        Self::Ticket(ticket)
    }

    /// Gets the length of the [EscrowData].
    pub fn len(&self) -> usize {
        match self {
            Self::Currency(_data) => Currency::len(),
            Self::Ticket(data) => data.len(),
        }
    }

    /// Gets whether the [EscrowData] is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Currency(data) => data.is_empty(),
            Self::Ticket(data) => data.is_empty(),
        }
    }

    /// Writes the [EscrowData] to a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidEscrowDataLen((buf_len, len)))
        } else {
            match self {
                Self::Currency(data) => data.to_bytes(buf),
                Self::Ticket(data) => data.to_bytes(buf),
            }
        }
    }

    /// Converts the [EscrowData] into a byte vector.
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Currency(data) => data.to_vec(),
            Self::Ticket(data) => data.to_vec(),
        }
    }
}

impl TryFrom<&[u8]> for EscrowData {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        let min_len = MIN_ESCROW_DATA_LEN;
        let val_len = val.len();

        if val_len < min_len {
            Err(Error::InvalidEscrowDataLen((val_len, min_len)))
        } else {
            match u16::from_le_bytes([val[0], val[1]]) {
                0 => {
                    let ticket_len = val[2] as usize;
                    let exp_len = min_len + ticket_len;

                    match ticket_len {
                        t if t > 0 && val_len >= exp_len => {
                            Ok(Self::Ticket(Ticket::try_from(val[3..3 + t].as_ref())?))
                        }
                        0 => Ok(Self::Ticket(Ticket::new())),
                        _ => Err(Error::InvalidEscrowDataLen((val_len, exp_len))),
                    }
                }
                _ => Ok(Self::Currency(val.try_into()?)),
            }
        }
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for EscrowData {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for EscrowData {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl Default for EscrowData {
    fn default() -> Self {
        Self::new()
    }
}
