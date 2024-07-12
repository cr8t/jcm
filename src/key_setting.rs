use std::fmt;

use crate::{Error, Result};

/// Represents `Key Setting` information for a given device key.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeySetting {
    /// The key is disabled.
    Disabled = 0,
    /// The key is enabled.
    Enabled = 1,
}

impl KeySetting {
    /// Creates a new [KeySetting].
    pub const fn new() -> Self {
        Self::Disabled
    }

    /// Attempts to convert a [`u8`] into a [KeySetting].
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Disabled),
            1 => Some(Self::Enabled),
            _ => None,
        }
    }

    /// Converts the [KeySetting] into a [`u8`].
    pub const fn into_u8(self) -> u8 {
        self as u8
    }
}

impl Default for KeySetting {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u8> for KeySetting {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        Self::from_u8(val).ok_or(Error::InvalidValue {
            field: "key_setting",
            value: val as usize,
        })
    }
}

impl TryFrom<&u8> for KeySetting {
    type Error = Error;

    fn try_from(val: &u8) -> Result<Self> {
        (*val).try_into()
    }
}

impl From<KeySetting> for u8 {
    fn from(val: KeySetting) -> Self {
        val.into_u8()
    }
}

impl From<&KeySetting> for u8 {
    fn from(val: &KeySetting) -> Self {
        (*val).into()
    }
}

impl From<KeySetting> for &'static str {
    fn from(val: KeySetting) -> Self {
        match val {
            KeySetting::Disabled => "disabled",
            KeySetting::Enabled => "enabled",
        }
    }
}

impl From<&KeySetting> for &'static str {
    fn from(val: &KeySetting) -> Self {
        (*val).into()
    }
}

impl fmt::Display for KeySetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

/// Represents a list of [KeySetting] information.
///
/// One [KeySetting] is used for each `Key` on the device.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeySettingList(Vec<KeySetting>);

impl KeySettingList {
    /// Creates a new [KeySettingList].
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new [KeySettingList] from the provided parameter.
    pub const fn create(list: Vec<KeySetting>) -> Self {
        Self(list)
    }

    /// Gets a reference to the list of [KeySetting] items.
    pub fn items(&self) -> &[KeySetting] {
        self.0.as_slice()
    }

    /// Pushes a [KeySetting] onto the end of the [KeySettingList].
    pub fn push(&mut self, setting: KeySetting) {
        self.0.push(setting)
    }

    /// Gets the length of the [KeySettingList].
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Gets whether the [KeySettingList] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Converts the [KeySettingList] into a [`u8`] iterator.
    pub fn iter_bytes(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.iter().map(|s| s.into_u8())
    }

    /// Converts the [KeySettingList] into a [`u8`] iterator.
    pub fn into_iter_bytes(self) -> impl Iterator<Item = u8> {
        self.0.into_iter().map(|s| s.into_u8())
    }
}

impl Default for KeySettingList {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&[KeySetting]> for KeySettingList {
    fn from(val: &[KeySetting]) -> Self {
        Self(val.into())
    }
}

impl TryFrom<&[u8]> for KeySettingList {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        let mut res = Ok(Self::new());

        let list = Self(
            val.iter()
                .copied()
                .filter_map(|b| match (res.is_ok(), KeySetting::try_from(b)) {
                    (true, Ok(s)) => Some(s),
                    (true, Err(err)) => {
                        res = Err(err);
                        None
                    }
                    (false, _) => None,
                })
                .collect(),
        );

        res.map(|_| list)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for KeySettingList {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for KeySettingList {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl fmt::Display for KeySettingList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for (i, s) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{s}")?;
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_setting() {
        [0, 1]
            .into_iter()
            .zip([KeySetting::Disabled, KeySetting::Enabled])
            .for_each(|(raw, exp)| {
                assert_eq!(KeySetting::try_from(raw), Ok(exp));
                assert_eq!(exp.into_u8(), raw);
                assert_eq!(u8::from(exp), raw);
            });
    }

    #[test]
    fn test_key_setting_invalid() {
        (2..=u8::MAX).for_each(|invalid| {
            assert_eq!(
                KeySetting::try_from(invalid),
                Err(Error::InvalidValue {
                    field: "key_setting",
                    value: invalid as usize,
                })
            );
        });
    }

    #[test]
    fn test_key_setting_list() {
        let raw = [0, 1, 1, 0];
        let list = vec![
            KeySetting::Disabled,
            KeySetting::Enabled,
            KeySetting::Enabled,
            KeySetting::Disabled,
        ];
        let exp = KeySettingList(list.clone());

        assert_eq!(KeySettingList::try_from(raw.as_ref()).as_ref(), Ok(&exp));
        assert_eq!(KeySettingList::from(list.as_slice()), exp);
    }

    #[test]
    fn test_key_setting_list_invalid() {
        assert_eq!(
            KeySettingList::try_from([0, 1, 2, 3]),
            Err(Error::InvalidValue {
                field: "key_setting",
                value: 2
            })
        );
    }
}
