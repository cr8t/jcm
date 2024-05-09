use std::ffi::CStr;
use std::{fmt, mem};

use crate::{Error, Result};

/// Represents the firmware version from a [VersionResponse](crate::VersionResponse).
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirmwareVersion {
    firmware_name: String,
    interface_number: String,
    version: String,
    date: String,
}

impl FirmwareVersion {
    /// Creates a new [FirmwareVersion].
    pub const fn new() -> Self {
        Self {
            firmware_name: String::new(),
            interface_number: String::new(),
            version: String::new(),
            date: String::new(),
        }
    }

    /// Gets the firmware name of the [FirmwareVersion].
    pub fn firmware_name(&self) -> &str {
        &self.firmware_name
    }

    /// Sets the firmware name of the [FirmwareVersion].
    pub fn set_firmware_name(&mut self, val: &str) {
        self.firmware_name = val.into();
    }

    /// Builder function that sets the firmware name of the [FirmwareVersion].
    pub fn with_firmware_name(mut self, val: &str) -> Self {
        self.set_firmware_name(val);
        self
    }

    /// Gets the interface number of the [FirmwareVersion].
    pub fn interface_number(&self) -> &str {
        &self.interface_number
    }

    /// Sets the interface number of the [FirmwareVersion].
    pub fn set_interface_number(&mut self, val: &str) {
        self.interface_number = val.into();
    }

    /// Builder function that sets the interface number of the [FirmwareVersion].
    pub fn with_interface_number(mut self, val: &str) -> Self {
        self.set_interface_number(val);
        self
    }

    /// Gets the version of the [FirmwareVersion].
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Sets the version of the [FirmwareVersion].
    pub fn set_version(&mut self, val: &str) {
        self.version = val.into();
    }

    /// Builder function that sets the version of the [FirmwareVersion].
    pub fn with_version(mut self, val: &str) -> Self {
        self.set_version(val);
        self
    }

    /// Gets the date of the [FirmwareVersion].
    pub fn date(&self) -> &str {
        &self.date
    }

    /// Sets the date of the [FirmwareVersion].
    pub fn set_date(&mut self, val: &str) {
        self.date = val.into();
    }

    /// Builder function that sets the date of the [FirmwareVersion].
    pub fn with_date(mut self, val: &str) -> Self {
        self.set_date(val);
        self
    }

    /// Gets the length of the [FirmwareVersion].
    pub fn len(&self) -> usize {
        self.firmware_name.len()
            + self.interface_number.len()
            + self.version.len()
            + self.date.len()
            + (4 * mem::size_of::<u8>())
    }

    /// Gets whether the [FirmwareVersion] is empty.
    pub fn is_empty(&self) -> bool {
        self.firmware_name.is_empty()
            && self.interface_number.is_empty()
            && self.version.is_empty()
            && self.date.is_empty()
    }

    /// Attempts to convert a byte buffer into a [FirmwareVersion].
    pub fn from_bytes(val: &[u8]) -> Result<Self> {
        let mut ver_iter = CStr::from_bytes_until_nul(val)?.to_str()?.split(' ');

        let firmware_name = ver_iter.next().ok_or(Error::InvalidFirmwareVersion)?.into();
        let interface_number = ver_iter.next().ok_or(Error::InvalidFirmwareVersion)?.into();
        let version = ver_iter.next().ok_or(Error::InvalidFirmwareVersion)?.into();
        let date = ver_iter.next().ok_or(Error::InvalidFirmwareVersion)?.into();

        Ok(Self {
            firmware_name,
            interface_number,
            version,
            date,
        })
    }

    /// Converts the [FirmwareVersion] into a nul-terminated byte iterator.
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.firmware_name
            .as_bytes()
            .iter()
            .cloned()
            .chain([b' '])
            .chain(self.interface_number.as_bytes().iter().cloned())
            .chain([b' '])
            .chain(self.version.as_bytes().iter().cloned())
            .chain([b' '])
            .chain(self.date.as_bytes().iter().cloned())
            .chain([0])
    }

    /// Attempts to convert a [FirmwareVersion] into a nul-terminated byte buffer.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = self.len();
        let buf_len = buf.len();
        if buf_len < len {
            Err(Error::InvalidFirmwareVersionLen((buf_len, len)))
        } else {
            buf.iter_mut()
                .take(len)
                .zip(self.iter())
                .for_each(|(dst, src)| *dst = src);

            Ok(())
        }
    }

    /// Converts the [FirmwareVersion] into a nul-terminated byte vector.
    pub fn into_bytes(&self) -> Vec<u8> {
        self.iter().collect()
    }
}

impl TryFrom<&[u8]> for FirmwareVersion {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::from_bytes(val)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for FirmwareVersion {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for FirmwareVersion {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl Default for FirmwareVersion {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FirmwareVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""firmware_name": "{}", "#, self.firmware_name())?;
        write!(f, r#""interface_number": "{}", "#, self.interface_number())?;
        write!(f, r#""version": "{}", "#, self.version())?;
        write!(f, r#""date": "{}""#, self.date())?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firmware_version() -> Result<()> {
        let raw = b"i(JPY)-100-SS 1 SomeVersion 01-25-01\0";
        let exp = FirmwareVersion::try_from(raw.as_ref())?;

        assert_eq!(FirmwareVersion::from_bytes(raw.as_ref())?, exp);

        let out = exp.into_bytes();
        assert_eq!(out, raw);

        Ok(())
    }

    #[test]
    fn test_firmware_version_invalid() -> Result<()> {
        let bad_vers = [
            // no date
            b"i(JPY)-100-SS 1 SomeVersion\0".as_ref(),
            // no date and version
            b"i(JPY)-100-SS 1\0".as_ref(),
            // no date, version, and interface number
            b"i(JPY)-100-SS\0".as_ref(),
            // no date, version, interface number, and name
            b"\0".as_ref(),
        ];

        let exp = FirmwareVersion::from_bytes(b"i(JPY)-100-SS 1 SomeVersion 01-25-01\0")?;
        let exp_len = exp.len();
        let mut out = vec![0u8; exp_len];

        for ver in bad_vers.into_iter() {
            assert!(FirmwareVersion::from_bytes(ver).is_err());
            assert!(exp.to_bytes(out[..ver.len()].as_mut()).is_err());
            assert!(exp.to_bytes(&mut []).is_err());
        }

        Ok(())
    }
}
