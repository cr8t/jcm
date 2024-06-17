const SIZE_AND_TOTAL: u8 = 0;

/// Represents the block number in a `Serial Number Image` request.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SerialNumberBlockNumber(u8);

impl SerialNumberBlockNumber {
    /// Represents the byte length of the [SerialNumberBlockNumber].
    pub const LEN: usize = 1;

    /// Creates a new [SerialNumberBlockNumber].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Gets the byte length of the [SerialNumberBlockNumber].
    pub const fn len(&self) -> usize {
        Self::LEN
    }

    /// Gets whether the [SerialNumberBlockNumber] is empty.
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Gets whether the [SerialNumberBlockNumber] is the initial request for size and total blocks.
    pub const fn is_size_and_total_info(&self) -> bool {
        self.0 == SIZE_AND_TOTAL 
    }

    /// Converts a [`u8`] into a [SerialNumberBlockNumber].
    pub const fn from_u8(val: u8) -> Self {
        Self(val)
    }

    /// Converts a [SerialNumberBlockNumber] into a [`u8`].
    pub const fn into_u8(self) -> u8 {
        self.0
    }

    /// Converts a [SerialNumberBlockNumber] into a byte array.
    pub const fn into_bytes(self) -> [u8; Self::LEN] {
        [self.0]
    }
}

impl Default for SerialNumberBlockNumber {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u8> for SerialNumberBlockNumber {
    fn from(val: u8) -> Self {
        Self::from_u8(val)
    }
}

impl From<SerialNumberBlockNumber> for u8 {
    fn from(val: SerialNumberBlockNumber) -> Self {
        val.into_u8()
    }
}

impl From<&SerialNumberBlockNumber> for u8 {
    fn from(val: &SerialNumberBlockNumber) -> Self {
        (*val).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_number() {
        let block = SerialNumberBlockNumber::new();

        assert!(block.is_size_and_total_info());
        assert_eq!(block.into_u8(), 0);

        (1..=u8::MAX).for_each(|n| {
            let exp = SerialNumberBlockNumber(n);

            assert_eq!(SerialNumberBlockNumber::from_u8(n), exp);
            assert_eq!(exp.into_u8(), n);

            assert_eq!(SerialNumberBlockNumber::from(n), exp);
            assert_eq!(u8::from(exp), n);
        });
    }
}
