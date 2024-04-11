use std::{fmt, mem};

use crate::{Error, FuncId, Result};

mod major_minor_status;
#[cfg(test)]
mod tests;

/// Convenience alias for [FuncId] for consistency with the specification.
pub type FunctionMode = FuncId;
pub use major_minor_status::*;

/// Represents the JCM device status.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeviceStatus {
    function_mode: FunctionMode,
    major_minor_status: MajorMinorStatus,
}

impl DeviceStatus {
    /// Creates a new [DeviceStatus].
    pub const fn new() -> Self {
        Self {
            function_mode: FunctionMode::new(),
            major_minor_status: MajorMinorStatus::new(),
        }
    }

    /// Creates a new [DeviceStatus] from the provided parameters.
    pub const fn create(function_mode: FunctionMode, major_minor_status: MajorMinorStatus) -> Self {
        Self {
            function_mode,
            major_minor_status,
        }
    }

    /// Gets the [FunctionMode] of the [DeviceStatus].
    pub const fn function_mode(&self) -> FunctionMode {
        self.function_mode
    }

    /// Sets the [FunctionMode] of the [DeviceStatus].
    pub fn set_function_mode(&mut self, val: FunctionMode) {
        self.function_mode = val;
    }

    /// Builder function that sets the [FunctionMode] of the [DeviceStatus].
    pub fn with_function_mode(mut self, val: FunctionMode) -> Self {
        self.set_function_mode(val);
        self
    }

    /// Gets the [MajorMinorStatus] of the [DeviceStatus].
    pub const fn major_minor_status(&self) -> MajorMinorStatus {
        self.major_minor_status
    }

    /// Sets the [MajorMinorStatus] of the [DeviceStatus].
    pub fn set_major_minor_status(&mut self, val: MajorMinorStatus) {
        self.major_minor_status = val;
    }

    /// Builder function that sets the [MajorMinorStatus] of the [DeviceStatus].
    pub fn with_major_minor_status(mut self, val: MajorMinorStatus) -> Self {
        self.set_major_minor_status(val);
        self
    }

    /// Infallible conversion from a [`u16`] into a [DeviceStatus].
    pub const fn from_u16(val: u16) -> Self {
        Self {
            function_mode: FunctionMode::from_u16(val),
            major_minor_status: MajorMinorStatus::from_u16(val),
        }
    }

    /// Converts the [DeviceStatus] into a byte array.
    pub fn to_bytes(&self) -> [u8; 2] {
        u16::from(self).to_le_bytes()
    }

    /// Gets the length of the [DeviceStatus].
    pub const fn len() -> usize {
        mem::size_of::<u16>()
    }

    /// Gets whether the [DeviceStatus] is empty.
    pub const fn is_empty(&self) -> bool {
        self.function_mode.is_empty() && self.major_minor_status.is_empty()
    }

    /// Gets whether the [DeviceStatus] has a valid combination of [FunctionMode] and
    /// [MajorMinorStatus] variants.
    pub fn is_valid(&self) -> bool {
        match self.function_mode {
            FunctionMode::Common => {
                matches!(
                    self.major_minor_status,
                    MajorMinorStatus::PowerUp
                        | MajorMinorStatus::PowerUpAcceptor
                        | MajorMinorStatus::PowerUpStacker
                        | MajorMinorStatus::Normal
                        | MajorMinorStatus::NormalActive
                        | MajorMinorStatus::NormalRejected
                        | MajorMinorStatus::NormalCollected
                        | MajorMinorStatus::Abnormal
                        | MajorMinorStatus::AbnormalOperationError
                        | MajorMinorStatus::WarningNoteStay
                ) || matches!(
                    self.major_minor_status,
                    MajorMinorStatus::AbnormalFailure(_err)
                )
            }
            FunctionMode::Acceptor => {
                matches!(
                    self.major_minor_status,
                    MajorMinorStatus::PowerUpAcceptorAccepting
                        | MajorMinorStatus::PowerUpStackerAccepting
                        | MajorMinorStatus::Normal
                        | MajorMinorStatus::NormalIdle
                        | MajorMinorStatus::NormalActive
                        | MajorMinorStatus::NormalEscrow
                        | MajorMinorStatus::NormalVendValid
                        | MajorMinorStatus::NormalRejected
                        | MajorMinorStatus::NormalReturned
                        | MajorMinorStatus::NormalCollected
                        | MajorMinorStatus::NormalInsert
                        | MajorMinorStatus::NormalConditionalVend
                        | MajorMinorStatus::NormalPause
                        | MajorMinorStatus::NormalResume
                        | MajorMinorStatus::Abnormal
                        | MajorMinorStatus::AbnormalOperationError
                        | MajorMinorStatus::WarningNoteStay
                        | MajorMinorStatus::WarningFunctionAbeyance
                ) || matches!(
                    self.major_minor_status,
                    MajorMinorStatus::AbnormalFailure(_err)
                )
            }
            _ => false,
        }
    }
}

impl Default for DeviceStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&DeviceStatus> for u16 {
    fn from(val: &DeviceStatus) -> Self {
        (*val).into()
    }
}

impl From<DeviceStatus> for u16 {
    fn from(val: DeviceStatus) -> Self {
        u16::from(val.function_mode) | u16::from(val.major_minor_status)
    }
}

impl TryFrom<u16> for DeviceStatus {
    type Error = Error;

    fn try_from(val: u16) -> Result<Self> {
        let res = Self {
            function_mode: FunctionMode::try_from(val)?,
            major_minor_status: MajorMinorStatus::try_from(val)?,
        };

        if res.is_valid() {
            Ok(res)
        } else {
            Err(Error::InvalidDeviceStatus(val))
        }
    }
}

impl TryFrom<[u8; 2]> for DeviceStatus {
    type Error = Error;

    fn try_from(val: [u8; 2]) -> Result<Self> {
        u16::from_le_bytes(val).try_into()
    }
}

impl TryFrom<&[u8]> for DeviceStatus {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        let len = Self::len();
        let val_len = val.len();

        if val_len < len {
            Err(Error::InvalidDeviceStatusLen((val_len, len)))
        } else {
            [val[0], val[1]].try_into()
        }
    }
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""function_mode": {}, "#, self.function_mode)?;
        write!(f, r#""major_minor_status": {}"#, self.major_minor_status)?;
        write!(f, "}}")
    }
}
