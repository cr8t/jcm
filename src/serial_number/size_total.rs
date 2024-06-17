use core::fmt;

use crate::{Error, Result};

/// Represents the image size and total block number of the Serial Number Image.
///
/// The initial `SerialNumberRequest` returns the total image size and number of blocks.
///
/// If the size and total are both zero, the device does not support sending the Serial Number
/// Image.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SerialNumberSizeTotal {
    size: u32,
    total: u8,
}

impl SerialNumberSizeTotal {
    const SIZE_LEN: usize = 4;
    const TOTAL_LEN: usize = 1;
    const UNSUPPORTED: u8 = 0;

    /// Represents the total byte length of the [SerialNumberSizeTotal].
    pub const LEN: usize = Self::SIZE_LEN + Self::TOTAL_LEN;

    /// Creates a new [SerialNumberSizeTotal].
    pub const fn new() -> Self {
        Self { size: 0, total: 0 }
    }

    /// Gets the byte length of the [SerialNumberSizeTotal].
    pub const fn len(&self) -> usize {
        Self::LEN
    }

    /// Gets whether the [SerialNumberSizeTotal] is empty.
    pub const fn is_empty(&self) -> bool {
        self.size == Self::UNSUPPORTED as u32 && self.total == Self::UNSUPPORTED
    }

    /// Gets the serial number image size.
    pub const fn size(&self) -> usize {
        self.size as usize
    }

    /// Sets the serial number image size.
    pub fn set_size(&mut self, val: usize) {
        self.size = val as u32;
    }

    /// Builder function that sets the serial number image size.
    pub const fn with_size(self, val: usize) -> Self {
        Self {
            size: val as u32,
            total: self.total,
        }
    }

    /// Gets the total number of blocks of serial number image data.
    pub const fn total_blocks(&self) -> usize {
        self.total as usize
    }

    /// Sets the total number of blocks of serial number image data.
    pub fn set_total_blocks(&mut self, val: usize) {
        self.total = val as u8;
    }

    /// Builder function that sets the total number of blocks of serial number image data.
    pub const fn with_total_blocks(self, val: usize) -> Self {
        Self {
            size: self.size,
            total: val as u8,
        }
    }

    /// Gets whether the `Serial Number Image` is supported.
    pub const fn is_supported(&self) -> bool {
        self.size != Self::UNSUPPORTED as u32 && self.total != Self::UNSUPPORTED
    }

    /// Converts a byte buffer into a [SerialNumberSizeTotal].
    pub const fn from_bytes(buf: &[u8]) -> Result<Self> {
        match buf.len() {
            Self::LEN => Ok(Self {
                size: u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
                total: buf[4],
            }),
            len => Err(Error::InvalidSerialNumberSizeTotalLen((len, Self::LEN))),
        }
    }

    /// Converts a [SerialNumberSizeTotal] into a byte array.
    pub const fn into_bytes(self) -> [u8; Self::LEN] {
        let size = self.size.to_le_bytes();
        [size[0], size[1], size[2], size[3], self.total]
    }
}

impl Default for SerialNumberSizeTotal {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&[u8]> for SerialNumberSizeTotal {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::from_bytes(val)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for SerialNumberSizeTotal {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for SerialNumberSizeTotal {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl From<SerialNumberSizeTotal> for [u8; SerialNumberSizeTotal::LEN] {
    fn from(val: SerialNumberSizeTotal) -> Self {
        val.into_bytes()
    }
}

impl IntoIterator for SerialNumberSizeTotal {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, { Self::LEN }>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_bytes().into_iter()
    }
}

impl fmt::Display for SerialNumberSizeTotal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""size": {}, "#, self.size)?;
        write!(f, r#""total_blocks": {}"#, self.total)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_total() {
        let size_total = SerialNumberSizeTotal::new();

        assert!(size_total.is_empty());
        assert!(!size_total.is_supported());

        assert_eq!(size_total.size(), 0);
        assert_eq!(size_total.total_blocks(), 0);

        let raw = [1, 2, 3, 4, 5];
        let exp = SerialNumberSizeTotal::new()
            .with_size(0x04030201)
            .with_total_blocks(5);

        assert_eq!(SerialNumberSizeTotal::from_bytes(raw.as_ref()), Ok(exp));
        assert_eq!(SerialNumberSizeTotal::try_from(raw.as_ref()), Ok(exp));
        assert_eq!(exp.size(), 0x04030201);
        assert_eq!(exp.total_blocks(), 5);
    }

    #[test]
    fn test_size_total_invalid() {
        (0..=u8::MAX as usize)
            .filter(|l| l != &SerialNumberSizeTotal::LEN)
            .for_each(|len| {
                assert_eq!(
                    SerialNumberSizeTotal::from_bytes(vec![0u8; len].as_ref()),
                    Err(Error::InvalidSerialNumberSizeTotalLen((
                        len,
                        SerialNumberSizeTotal::LEN
                    )))
                );
                assert_eq!(
                    SerialNumberSizeTotal::try_from(vec![0u8; len].as_slice()),
                    Err(Error::InvalidSerialNumberSizeTotalLen((
                        len,
                        SerialNumberSizeTotal::LEN
                    )))
                );
            });
    }
}
