use std::fmt;

pub use currency_iso4217::Currency as CurrencyCode;

use crate::{Denomination, Error, Result};

/// Represents the length in bytes of the [Currency].
pub const CURRENCY_LEN: usize = 5;

/// Represents device currency code and denomination.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Currency {
    code: CurrencyCode,
    denomination: Denomination,
}

impl Currency {
    /// Creates a new [Currency].
    pub const fn new() -> Self {
        Self {
            code: CurrencyCode::JPY,
            denomination: Denomination::new(),
        }
    }

    /// Gets the [CurrencyCode] for the [Currency].
    pub const fn code(&self) -> CurrencyCode {
        self.code
    }

    /// Sets the [CurrencyCode] for the [Currency].
    pub fn set_code(&mut self, code: CurrencyCode) {
        self.code = code;
    }

    /// Builder function that sets the [CurrencyCode] for the [Currency].
    pub fn with_code(mut self, code: CurrencyCode) -> Self {
        self.set_code(code);
        self
    }

    /// Gets the [Denomination] for the [Currency].
    pub const fn denomination(&self) -> Denomination {
        self.denomination
    }

    /// Sets the [Denomination] for the [Currency].
    pub fn set_denomination(&mut self, denomination: Denomination) {
        self.denomination = denomination;
    }

    /// Builder function that sets the [Denomination] for the [Currency].
    pub fn with_denomination(mut self, denomination: Denomination) -> Self {
        self.set_denomination(denomination);
        self
    }

    /// Gets the length of the [Currency].
    pub const fn len() -> usize {
        CurrencyCode::LEN + Denomination::len()
    }

    /// Gets whether the [Currency] is empty.
    pub const fn is_empty(&self) -> bool {
        matches!(self.code, CurrencyCode::XXX) || self.denomination.is_empty()
    }

    /// Attempts to convert a byte buffer into a [Currency].
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let len = Self::len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidCurrencyLen((buf_len, len)))
        } else {
            match CurrencyCode::from(buf) {
                code if code == CurrencyCode::XXX => Err(Error::InvalidCurrency((code.into(), 0))),
                code => {
                    let denomination = Denomination::try_from(buf[CurrencyCode::LEN..].as_ref())?;
                    Ok(Self { code, denomination })
                }
            }
        }
    }

    /// Writes the [Currency] to a byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = Self::len();
        let buf_len = buf.len();

        if buf_len < len {
            Err(Error::InvalidCurrencyLen((buf_len, len)))
        } else {
            buf[..CurrencyCode::LEN].copy_from_slice(<&str>::from(self.code).as_bytes());
            self.denomination.to_bytes(&mut buf[CurrencyCode::LEN..len])
        }
    }

    /// Converts the [Currency] into a byte array.
    pub fn into_bytes(&self) -> [u8; CURRENCY_LEN] {
        let mut buf = [0u8; CURRENCY_LEN];
        buf[..CurrencyCode::LEN].copy_from_slice(<&str>::from(self.code).as_bytes());
        self.denomination
            .to_bytes(&mut buf[CurrencyCode::LEN..])
            .ok();
        buf
    }

    /// Converts the [Currency] into byte vector.
    pub fn to_vec(&self) -> Vec<u8> {
        self.into_bytes().into()
    }
}

impl Default for Currency {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#""{} {}""#,
            self.denomination.value(),
            <&str>::from(self.code)
        )
    }
}

impl TryFrom<&[u8]> for Currency {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::from_bytes(val)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for Currency {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for Currency {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}
