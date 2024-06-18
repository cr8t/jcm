use std::fmt;

/// Represents serial number image data block.
///
/// Block length is variable, and is calculated as:
///
/// ```text,ignore
/// block_len = total_image_size / total_image_blocks
/// ```
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerialNumberBlock(Vec<u8>);

impl SerialNumberBlock {
    /// Creates a new [SerialNumberBlock].
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new [SerialNumberBlock] from the provided parameter.
    pub const fn create(val: Vec<u8>) -> Self {
        Self(val)
    }

    /// Gets the length of the [SerialNumberBlock].
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Gets whether the [SerialNumberBlock] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets a reference to the block data.
    pub fn block(&self) -> &[u8] {
        self.0.as_ref()
    }

    /// Gets an iterator over the [SerialNumberBlock] bytes.
    pub fn iter(&self) -> impl Iterator<Item = &u8> + '_ {
        self.0.iter()
    }
}

impl Default for SerialNumberBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&[u8]> for SerialNumberBlock {
    fn from(val: &[u8]) -> Self {
        Self(val.into())
    }
}

impl From<SerialNumberBlock> for Vec<u8> {
    fn from(val: SerialNumberBlock) -> Self {
        val.0
    }
}

impl From<&SerialNumberBlock> for Vec<u8> {
    fn from(val: &SerialNumberBlock) -> Self {
        val.0.clone()
    }
}

impl IntoIterator for SerialNumberBlock {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for SerialNumberBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, b) in self.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{b}")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_number_block() {
        let raw = [0; 128];
        let block = SerialNumberBlock::create(raw.into());

        assert_eq!(SerialNumberBlock::new().len(), 0);
        assert!(SerialNumberBlock::new().is_empty());

        assert_eq!(block.block(), raw.as_ref());
    }
}
