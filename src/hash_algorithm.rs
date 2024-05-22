//! Represents `Program Signature` hash algorithm information.

use crate::{Error, Result};

/// The raw CRC-16 `ProgramSignature` hash number.
pub const CRC16: u8 = 0b0001;
/// The raw CRC-32 `ProgramSignature` hash number.
pub const CRC32: u8 = 0b0010;
/// The raw SHA-1 `ProgramSignature` hash number.
pub const SHA1: u8 = 0b0100;
const RESERVED: u8 = 0xff;

/// The `ProgramSignature` CRC-16 hash length.
pub const CRC16_LEN: usize = 2;
const CRC16_REQ_LEN: usize = 3;
/// The `ProgramSignature` CRC-32 hash length.
pub const CRC32_LEN: usize = 4;
const CRC32_REQ_LEN: usize = 5;
/// The `ProgramSignature` SHA-1 hash length.
pub const SHA1_LEN: usize = 32;
const SHA1_REQ_LEN: usize = 33;

/// Represents the default value for a firmware program signture.
///
/// This value is supplied to the
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HashAlgorithm {
    Crc16([u8; CRC16_LEN]),
    Crc32([u8; CRC32_LEN]),
    Sha1([u8; SHA1_LEN]),
}

impl HashAlgorithm {
    /// Creates a new [HashAlgorithm].
    pub const fn new() -> Self {
        Self::Crc16([0u8; CRC16_LEN])
    }

    /// Gets the [AlgorithmNumber] for the [HashAlgorithm].
    pub const fn algorithm_number(&self) -> AlgorithmNumber {
        match self {
            Self::Crc16(_) => AlgorithmNumber::Crc16,
            Self::Crc32(_) => AlgorithmNumber::Crc32,
            Self::Sha1(_) => AlgorithmNumber::Sha1,
        }
    }

    /// Converts a `ProgramSignature` request buffer into a [HashAlgorithm].
    pub const fn from_request(req: &[u8]) -> Result<Self> {
        match req.len() {
            CRC16_REQ_LEN if matches!(AlgorithmNumber::from_u8(req[0]), AlgorithmNumber::Crc16) => {
                Ok(Self::Crc16([req[1], req[2]]))
            }
            CRC32_REQ_LEN if matches!(AlgorithmNumber::from_u8(req[0]), AlgorithmNumber::Crc32) => {
                Ok(Self::Crc32([req[1], req[2], req[3], req[4]]))
            }
            SHA1_REQ_LEN if matches!(AlgorithmNumber::from_u8(req[0]), AlgorithmNumber::Sha1) => {
                Ok(Self::Sha1([
                    req[1], req[2], req[3], req[4], req[5], req[6], req[7], req[8], req[9],
                    req[10], req[11], req[12], req[13], req[14], req[15], req[16], req[17],
                    req[18], req[19], req[20], req[21], req[22], req[23], req[24], req[25],
                    req[26], req[27], req[28], req[29], req[30], req[31], req[32],
                ]))
            }
            1 => Self::from_u8(req[0]),
            len => Err(Error::InvalidRequestLen((len, CRC16_REQ_LEN))),
        }
    }

    /// Converts a [`u8`] into a [HashAlgorithm].
    pub const fn from_u8(val: u8) -> Result<Self> {
        match AlgorithmNumber::from_u8(val) {
            AlgorithmNumber::Crc16 => Ok(Self::Crc16([0u8; CRC16_LEN])),
            AlgorithmNumber::Crc32 => Ok(Self::Crc32([0u8; CRC32_LEN])),
            AlgorithmNumber::Sha1 => Ok(Self::Sha1([0u8; SHA1_LEN])),
            _ => Err(Error::InvalidAlgorithmNumber(val)),
        }
    }

    /// Gets a reference the the default [HashAlgorithm] value.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Crc16(hash) => hash.as_ref(),
            Self::Crc32(hash) => hash.as_ref(),
            Self::Sha1(hash) => hash.as_ref(),
        }
    }

    /// Converts the [HashAlgorithm] into a `ProgramSignature` request buffer.
    pub fn to_request(&self, buf: &mut [u8]) -> Result<()> {
        match (self, buf.len()) {
            (Self::Crc16(hash), len) if len >= CRC16_REQ_LEN => {
                buf.iter_mut()
                    .take(CRC16_REQ_LEN)
                    .zip([AlgorithmNumber::Crc16.into_u8()].iter().chain(hash.iter()))
                    .for_each(|(dst, &src)| *dst = src);
                Ok(())
            }
            (Self::Crc16(_), len) => Err(Error::InvalidRequestLen((len, CRC16_REQ_LEN))),
            (Self::Crc32(hash), len) if len >= CRC32_REQ_LEN => {
                buf.iter_mut()
                    .take(CRC32_REQ_LEN)
                    .zip([AlgorithmNumber::Crc32.into_u8()].iter().chain(hash.iter()))
                    .for_each(|(dst, &src)| *dst = src);
                Ok(())
            }
            (Self::Crc32(_), len) => Err(Error::InvalidRequestLen((len, CRC32_REQ_LEN))),
            (Self::Sha1(hash), len) if len >= SHA1_REQ_LEN => {
                buf.iter_mut()
                    .take(SHA1_REQ_LEN)
                    .zip([AlgorithmNumber::Sha1.into_u8()].iter().chain(hash.iter()))
                    .for_each(|(dst, &src)| *dst = src);
                Ok(())
            }
            (Self::Sha1(_), len) => Err(Error::InvalidRequestLen((len, SHA1_REQ_LEN))),
        }
    }

    /// Converts the [HashAlgorithm] into a `ProgramSignature` request buffer.
    pub fn into_request(self) -> Vec<u8> {
        match self {
            Self::Crc16(hash) => [AlgorithmNumber::Crc16.into_u8()]
                .into_iter()
                .chain(hash)
                .collect(),
            Self::Crc32(hash) => [AlgorithmNumber::Crc32.into_u8()]
                .into_iter()
                .chain(hash)
                .collect(),
            Self::Sha1(hash) => [AlgorithmNumber::Sha1.into_u8()]
                .into_iter()
                .chain(hash)
                .collect(),
        }
    }
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<AlgorithmNumber> for HashAlgorithm {
    type Error = Error;

    fn try_from(val: AlgorithmNumber) -> Result<Self> {
        Self::from_u8(val.into_u8())
    }
}

impl TryFrom<u8> for HashAlgorithm {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        Self::from_u8(val)
    }
}

impl TryFrom<&[u8]> for HashAlgorithm {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::from_request(val)
    }
}

/// Represents a hash algorithm number for the program firmware signature.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlgorithmNumber {
    Crc16 = CRC16,
    Crc32 = CRC32,
    Sha1 = SHA1,
    Reserved = RESERVED,
}

impl AlgorithmNumber {
    /// Creates a new [AlgorithmNumber].
    pub const fn new() -> Self {
        Self::Crc16
    }

    /// Infallible conversion of a [`u8`] to a [AlgorithmNumber].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            CRC16 => Self::Crc16,
            CRC32 => Self::Crc32,
            SHA1 => Self::Sha1,
            _ => Self::Reserved,
        }
    }

    /// Converts the [AlgorithmNumber] into a [`u8`].
    pub const fn into_u8(self) -> u8 {
        self as u8
    }
}

impl Default for AlgorithmNumber {
    fn default() -> Self {
        Self::new()
    }
}

impl From<AlgorithmNumber> for u8 {
    fn from(val: AlgorithmNumber) -> Self {
        val.into_u8()
    }
}

impl From<&AlgorithmNumber> for u8 {
    fn from(val: &AlgorithmNumber) -> Self {
        val.into_u8()
    }
}

impl TryFrom<u8> for AlgorithmNumber {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidAlgorithmNumber(val)),
            algo => Ok(algo),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_algorithm() -> Result<()> {
        let raw_crc16 = [0u8; CRC16_LEN];
        let raw_crc16_req: Vec<u8> = [CRC16].into_iter().chain(raw_crc16).collect();
        let exp_crc16 = HashAlgorithm::Crc16(raw_crc16);

        let raw_crc32 = [1u8; CRC32_LEN];
        let raw_crc32_req: Vec<u8> = [CRC32].into_iter().chain(raw_crc32).collect();
        let exp_crc32 = HashAlgorithm::Crc32(raw_crc32);

        let raw_sha1 = [2u8; SHA1_LEN];
        let raw_sha1_req: Vec<u8> = [SHA1].into_iter().chain(raw_sha1).collect();
        let exp_sha1 = HashAlgorithm::Sha1(raw_sha1);

        for (raw, raw_req, exp) in [
            (raw_crc16.as_ref(), raw_crc16_req.as_ref(), exp_crc16),
            (raw_crc32.as_ref(), raw_crc32_req.as_ref(), exp_crc32),
            (raw_sha1.as_ref(), raw_sha1_req.as_ref(), exp_sha1),
        ] {
            let hash = HashAlgorithm::from_request(raw_req)?;
            assert_eq!(hash, exp);
            assert_eq!(exp.as_bytes(), raw);

            let mut out = vec![0u8; raw_req.len()];
            assert!(exp.to_request(&mut out).is_ok());
            assert_eq!(out, raw_req);

            assert_eq!(exp.into_request(), raw_req);

            let null_hash = HashAlgorithm::from_request(raw_req[..1].as_ref())?;
            assert_eq!(null_hash.algorithm_number(), exp.algorithm_number());
            assert_eq!(null_hash.algorithm_number().into_u8(), raw_req[0]);
            assert!(null_hash.as_bytes().iter().all(|b| b == &0));
        }

        Ok(())
    }

    #[test]
    fn test_hash_algorithm_invalid() -> Result<()> {
        // Assert that too-short request buffers throw an error.
        [
            [CRC16; CRC16_LEN].as_slice(),
            [CRC32; CRC32_LEN].as_slice(),
            [SHA1; SHA1_LEN].as_slice(),
        ]
        .into_iter()
        .for_each(|invalid| {
            assert!(HashAlgorithm::from_request(invalid).is_err());
        });

        // Assert that invalid algorithm numbers throw an error.
        (0..=255)
            .filter(|&i| i != CRC16 && i != CRC32 && i != SHA1)
            .for_each(|invalid| {
                assert!(HashAlgorithm::from_request([invalid; SHA1_REQ_LEN].as_ref()).is_err());
            });

        Ok(())
    }

    #[test]
    fn test_algorithm_number() {
        [
            (CRC16, AlgorithmNumber::Crc16),
            (CRC32, AlgorithmNumber::Crc32),
            (SHA1, AlgorithmNumber::Sha1),
        ]
        .into_iter()
        .for_each(|(raw, exp)| {
            assert_eq!(AlgorithmNumber::from_u8(raw), exp);
        });
    }

    #[test]
    fn test_algorithm_number_invalid() {
        (0..=255)
            .filter(|&i| i != CRC16 && i != CRC32 && i != SHA1)
            .for_each(|invalid| {
                assert_eq!(AlgorithmNumber::from_u8(invalid), AlgorithmNumber::Reserved);
                assert_eq!(
                    AlgorithmNumber::try_from(invalid),
                    Err(Error::InvalidAlgorithmNumber(invalid))
                );
            });
    }
}
