use std::fmt;

/// Represents an image data block.
///
/// Block length is variable, and is calculated as:
///
/// ```text,ignore
/// block_len = total_image_size / total_image_blocks
/// ```
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageBlock(Vec<u8>);

impl ImageBlock {
    /// Creates a new [ImageBlock].
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new [ImageBlock] from the provided parameter.
    pub const fn create(val: Vec<u8>) -> Self {
        Self(val)
    }

    /// Gets the length of the [ImageBlock].
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Gets whether the [ImageBlock] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets a reference to the block data.
    pub fn block(&self) -> &[u8] {
        self.0.as_ref()
    }

    /// Gets an iterator over the [ImageBlock] bytes.
    pub fn iter(&self) -> impl Iterator<Item = &u8> + '_ {
        self.0.iter()
    }
}

impl Default for ImageBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&[u8]> for ImageBlock {
    fn from(val: &[u8]) -> Self {
        Self(val.into())
    }
}

impl From<ImageBlock> for Vec<u8> {
    fn from(val: ImageBlock) -> Self {
        val.0
    }
}

impl From<&ImageBlock> for Vec<u8> {
    fn from(val: &ImageBlock) -> Self {
        val.0.clone()
    }
}

impl IntoIterator for ImageBlock {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for ImageBlock {
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
        let block = ImageBlock::create(raw.into());

        assert_eq!(ImageBlock::new().len(), 0);
        assert!(ImageBlock::new().is_empty());

        assert_eq!(block.block(), raw.as_ref());
    }
}
