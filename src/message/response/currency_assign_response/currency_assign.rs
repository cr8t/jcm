use std::fmt;

use crate::{Currency, Error, Result};

pub const CURRENCY_ASSIGN_LEN: usize = 6;

/// Represents a currency-to-bit assignment for
/// [CurrencyAssignResponse](crate::CurrencyAssignResponse) messages.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurrencyAssign {
    bit: u8,
    currency: Currency,
}

impl CurrencyAssign {
    /// Creates a new [CurrencyAssign].
    pub const fn new() -> Self {
        Self {
            bit: 0,
            currency: Currency::new(),
        }
    }

    /// Gets the bit number of the [CurrencyAssign].
    pub const fn bit_number(&self) -> u8 {
        self.bit
    }

    /// Sets the bit number of the [CurrencyAssign].
    pub fn set_bit_number(&mut self, val: u8) {
        self.bit = val;
    }

    /// Builder function that sets the bit number of the [CurrencyAssign].
    pub fn with_bit_number(mut self, val: u8) -> Self {
        self.set_bit_number(val);
        self
    }

    /// Gets the currency of the [CurrencyAssign].
    pub const fn currency(&self) -> Currency {
        self.currency
    }

    /// Sets the currency of the [CurrencyAssign].
    pub fn set_currency(&mut self, val: Currency) {
        self.currency = val;
    }

    /// Builder function that sets the currency of the [CurrencyAssign].
    pub fn with_currency(mut self, val: Currency) -> Self {
        self.set_currency(val);
        self
    }

    /// Converts a byte buffer into a [CurrencyAssign].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        match buf.len() {
            len if len >= CURRENCY_ASSIGN_LEN => Ok(Self {
                bit: buf[0],
                currency: Currency::from_bytes(&buf[1..CURRENCY_ASSIGN_LEN])?,
            }),
            len => Err(Error::InvalidCurrencyAssignLen((len, CURRENCY_ASSIGN_LEN))),
        }
    }

    /// Writes a [CurrencyAssign] into a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        match buf.len() {
            len if len >= CURRENCY_ASSIGN_LEN => {
                buf[0] = self.bit;
                self.currency.to_bytes(&mut buf[1..CURRENCY_ASSIGN_LEN])?;
                Ok(())
            }
            len => Err(Error::InvalidCurrencyAssignLen((len, CURRENCY_ASSIGN_LEN))),
        }
    }

    /// Converts the [CurrencyAssign] into a byte array.
    pub fn into_bytes(&self) -> [u8; CURRENCY_ASSIGN_LEN] {
        let mut buf = [0u8; CURRENCY_ASSIGN_LEN];
        self.to_bytes(buf.as_mut()).ok();
        buf
    }
}

impl Default for CurrencyAssign {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&CurrencyAssign> for [u8; CURRENCY_ASSIGN_LEN] {
    fn from(val: &CurrencyAssign) -> Self {
        val.into_bytes()
    }
}

impl From<CurrencyAssign> for [u8; CURRENCY_ASSIGN_LEN] {
    fn from(val: CurrencyAssign) -> Self {
        (&val).into()
    }
}

impl TryFrom<&[u8]> for CurrencyAssign {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::from_bytes(val)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for CurrencyAssign {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}
impl<const N: usize> TryFrom<[u8; N]> for CurrencyAssign {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl fmt::Display for CurrencyAssign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""bit_number": {}, "#, self.bit)?;
        write!(f, r#""currency": {}"#, self.currency)?;
        write!(f, "}}")
    }
}

/// Represents a list of [CurrencyAsssign].
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct CurrencyAssignList(Vec<CurrencyAssign>);

impl CurrencyAssignList {
    /// Creates a new [CurrencyAssignList].
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Gets a reference to the list of [CurrencyAssign] items.
    pub fn items(&self) -> &[CurrencyAssign] {
        self.0.as_ref()
    }

    /// Sets a reference to the list of [CurrencyAssign] items.
    pub fn set_items(&mut self, items: &[CurrencyAssign]) {
        self.0 = items.into();
    }

    /// Builder function that sets a reference to the list of [CurrencyAssign] items.
    pub fn with_items(mut self, items: &[CurrencyAssign]) -> Self {
        self.set_items(items);
        self
    }

    /// Gets an iterator over the list of [CurrencyAssign] items.
    pub fn iter(&self) -> impl Iterator<Item = &CurrencyAssign> {
        self.0.iter()
    }

    /// Gets a mutable iterator over the list of [CurrencyAssign] items.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut CurrencyAssign> {
        self.0.iter_mut()
    }

    /// Gets the byte length of the [CurrencyAssignList].
    pub fn len(&self) -> usize {
        self.0.len().saturating_mul(CURRENCY_ASSIGN_LEN)
    }

    /// Gets whether the [CurrencyAssignList] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Pushes a [CurrencyAssign] item onto the [CurrencyAssignList].
    pub fn push(&mut self, item: CurrencyAssign) {
        self.0.push(item);
    }

    /// Pops a [CurrencyAssign] item onto the [CurrencyAssignList].
    pub fn pop(&mut self) -> Option<CurrencyAssign> {
        self.0.pop()
    }

    /// Gets an iterator over the [CurrencyAssignList] converted to bytes.
    pub fn iter_bytes(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.iter().flat_map(|c| c.into_bytes().into_iter())
    }
}

impl Default for CurrencyAssignList {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&[u8]> for CurrencyAssignList {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        match val.len() {
            len if len >= CURRENCY_ASSIGN_LEN && len % CURRENCY_ASSIGN_LEN == 0 => Ok(Self(
                val.chunks_exact(CURRENCY_ASSIGN_LEN)
                    .filter_map(|c| CurrencyAssign::try_from(c).ok())
                    .collect(),
            )),
            len => Err(Error::InvalidCurrencyAssignLen((len, CURRENCY_ASSIGN_LEN))),
        }
    }
}

impl fmt::Display for CurrencyAssignList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, ca) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{ca}")?;
        }
        write!(f, "]")
    }
}
