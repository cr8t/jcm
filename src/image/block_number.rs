use std::fmt;

const SIZE_AND_TOTAL: u8 = 0;

/// Represents the block number in an image request.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageBlockNumber(u8);

impl ImageBlockNumber {
    /// Represents the byte length of the [ImageBlockNumber].
    pub const LEN: usize = 1;

    /// Creates a new [ImageBlockNumber].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Gets the byte length of the [ImageBlockNumber].
    pub const fn len(&self) -> usize {
        Self::LEN
    }

    /// Gets whether the [ImageBlockNumber] is empty.
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Gets whether the [ImageBlockNumber] is the initial request for size and total blocks.
    pub const fn is_size_and_total_info(&self) -> bool {
        self.0 == SIZE_AND_TOTAL
    }

    /// Converts a [`u8`] into a [ImageBlockNumber].
    pub const fn from_u8(val: u8) -> Self {
        Self(val)
    }

    /// Converts a [ImageBlockNumber] into a [`u8`].
    pub const fn into_u8(self) -> u8 {
        self.0
    }

    /// Converts a [ImageBlockNumber] into a byte array.
    pub const fn into_bytes(self) -> [u8; Self::LEN] {
        [self.0]
    }
}

impl Default for ImageBlockNumber {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u8> for ImageBlockNumber {
    fn from(val: u8) -> Self {
        Self::from_u8(val)
    }
}

impl From<ImageBlockNumber> for u8 {
    fn from(val: ImageBlockNumber) -> Self {
        val.into_u8()
    }
}

impl From<&ImageBlockNumber> for u8 {
    fn from(val: &ImageBlockNumber) -> Self {
        (*val).into()
    }
}

impl fmt::Display for ImageBlockNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_number() {
        let block = ImageBlockNumber::new();

        assert!(block.is_size_and_total_info());
        assert_eq!(block.into_u8(), 0);

        (1..=u8::MAX).for_each(|n| {
            let exp = ImageBlockNumber(n);

            assert_eq!(ImageBlockNumber::from_u8(n), exp);
            assert_eq!(exp.into_u8(), n);

            assert_eq!(ImageBlockNumber::from(n), exp);
            assert_eq!(u8::from(exp), n);
        });
    }
}
