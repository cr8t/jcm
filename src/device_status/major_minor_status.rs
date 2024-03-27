use std::{fmt, mem};

use crate::{Error, FailureCode, Result};

const POWER_UP: u16 = 0x0000;
const POWER_UP_ACCEPTOR: u16 = 0x0001;
const POWER_UP_STACKER: u16 = 0x0002;
const POWER_UP_ACCEPTOR_ACCEPTING: u16 = 0x0011;
const POWER_UP_STACKER_ACCEPTING: u16 = 0x0012;

const NORMAL: u16 = 0x0100;
const NORMAL_IDLE: u16 = 0x0101;
const NORMAL_ACTIVE: u16 = 0x0102;
const NORMAL_ESCROW: u16 = 0x0103;
const NORMAL_VEND_VALID: u16 = 0x0104;
const NORMAL_REJECTED: u16 = 0x0105;
const NORMAL_RETURNED: u16 = 0x0106;
const NORMAL_COLLECTED: u16 = 0x0108;
const NORMAL_INSERT: u16 = 0x010a;
const NORMAL_CONDITIONAL_VEND: u16 = 0x010b;
const NORMAL_PAUSE: u16 = 0x010c;
const NORMAL_RESUME: u16 = 0x010d;

const ABNORMAL: u16 = 0x0200;
const ABNORMAL_OPERATION_ERROR: u16 = 0x0201;

const WARNING: u16 = 0x0300;
const WARNING_NOTE_STAY: u16 = 0x0301;
const WARNING_FUNCTION_ABEYANCE: u16 = 0x0302;

const MAJOR_MASK: u16 = 0x0f00;
const MAJOR_SHIFT: u16 = 8;
const MINOR_MASK: u16 = 0x00ff;
const MAJOR_MINOR_MASK: u16 = MAJOR_MASK | MINOR_MASK;

const RESERVED: u16 = 0xffff;

/// Represents the major-minor status of the JCM device status.
#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MajorMinorStatus {
    /// Power up status: device is in normal power up status.
    PowerUp = POWER_UP,
    /// Power up status: device has returnable note on power up.
    PowerUpAcceptor = POWER_UP_ACCEPTOR,
    /// Power up status: device has non-returnable note on power up.
    PowerUpStacker = POWER_UP_STACKER,
    /// Power up status: device received note before power down, and has returnable note on power up.
    PowerUpAcceptorAccepting = POWER_UP_ACCEPTOR_ACCEPTING,
    /// Power up status: device received note before power down, and has non-returnable note on power up.
    PowerUpStackerAccepting = POWER_UP_STACKER_ACCEPTING,
    /// Normal status: device is disabled
    Normal = NORMAL,
    /// Normal status: acceptor function is in stand-by mode.
    NormalIdle = NORMAL_IDLE,
    /// Normal status: device is in operation.
    NormalActive = NORMAL_ACTIVE,
    /// Normal status: inserted note passed validation check.
    NormalEscrow = NORMAL_ESCROW,
    /// Normal status: stacking the note is allowed.
    NormalVendValid = NORMAL_VEND_VALID,
    /// Normal status:
    ///
    /// - `common`: note left in transport is removed during `Power Up`
    /// - `acceptor`: returning note before `Escrow Status` is notified
    NormalRejected = NORMAL_REJECTED,
    /// Normal status: returning an inserted note after the `Escrow Status` is notified.
    NormalReturned = NORMAL_RETURNED,
    /// Normal status:
    ///
    /// - `common`: note left in transport is collected during `Power Up`
    /// - `acceptor`: collecting a scanned note completed
    NormalCollected = NORMAL_COLLECTED,
    /// Normal status: note has been inserted.
    NormalInsert = NORMAL_INSERT,
    /// Normal status: `Conditional Stacking` is processing.
    NormalConditionalVend = NORMAL_CONDITIONAL_VEND,
    /// Normal status: device has stopped.
    NormalPause = NORMAL_PAUSE,
    /// Normal status: device has resumed.
    NormalResume = NORMAL_RESUME,
    /// Abnormal: controversial error has been resolved.
    Abnormal = ABNORMAL,
    /// Abnormal: unable to operate, `Operation Message` is not processed.
    AbnormalOperationError = ABNORMAL_OPERATION_ERROR,
    /// Abnormal: fatal error occurred.
    AbnormalFailure(FailureCode),
    /// Warning status.
    Warning = WARNING,
    /// Warning status: note has stayed in the `Insertion` position for an extended period.
    WarningNoteStay = WARNING_NOTE_STAY,
    /// Warning status: mode enable by `Function Mode` is not functional.
    WarningFunctionAbeyance = WARNING_FUNCTION_ABEYANCE,
    /// Reserved function.
    Reserved = RESERVED,
}

impl MajorMinorStatus {
    /// Creates a new [MajorMinorStatus].
    pub const fn new() -> Self {
        Self::PowerUp
    }

    /// Infallible conversion from a [`u8`] into a [MajorMinorStatus].
    pub const fn from_u8(val: u8) -> Self {
        Self::from_u16((val as u16) << MAJOR_SHIFT)
    }

    /// Infallible conversion from a [`u16`] into a [MajorMinorStatus].
    pub const fn from_u16(val: u16) -> Self {
        match val & MAJOR_MINOR_MASK {
            POWER_UP => Self::PowerUp,
            POWER_UP_ACCEPTOR => Self::PowerUpAcceptor,
            POWER_UP_STACKER => Self::PowerUpStacker,
            POWER_UP_ACCEPTOR_ACCEPTING => Self::PowerUpAcceptorAccepting,
            POWER_UP_STACKER_ACCEPTING => Self::PowerUpStackerAccepting,
            NORMAL => Self::Normal,
            NORMAL_IDLE => Self::NormalIdle,
            NORMAL_ACTIVE => Self::NormalActive,
            NORMAL_ESCROW => Self::NormalEscrow,
            NORMAL_VEND_VALID => Self::NormalVendValid,
            NORMAL_REJECTED => Self::NormalRejected,
            NORMAL_RETURNED => Self::NormalReturned,
            NORMAL_COLLECTED => Self::NormalCollected,
            NORMAL_INSERT => Self::NormalInsert,
            NORMAL_CONDITIONAL_VEND => Self::NormalConditionalVend,
            NORMAL_PAUSE => Self::NormalPause,
            NORMAL_RESUME => Self::NormalResume,
            ABNORMAL => Self::Abnormal,
            ABNORMAL_OPERATION_ERROR => Self::AbnormalOperationError,
            v if v & MAJOR_MASK == ABNORMAL => match FailureCode::from_u8((val & MINOR_MASK) as u8)
            {
                FailureCode::Reserved => Self::Reserved,
                code => Self::AbnormalFailure(code),
            },
            WARNING => Self::Warning,
            WARNING_NOTE_STAY => Self::WarningNoteStay,
            WARNING_FUNCTION_ABEYANCE => Self::WarningFunctionAbeyance,
            _ => Self::Reserved,
        }
    }

    /// Gets the length of the [MajorMinorStatus].
    pub const fn len() -> usize {
        mem::size_of::<u16>()
    }

    /// Gets whether the [MajorMinorStatus] contains a reserved variant.
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Reserved)
    }
}

impl TryFrom<u8> for MajorMinorStatus {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidMajorMinorStatus((val as u16) << MAJOR_SHIFT)),
            v => Ok(v),
        }
    }
}

impl TryFrom<u16> for MajorMinorStatus {
    type Error = Error;

    fn try_from(val: u16) -> Result<Self> {
        match Self::from_u16(val) {
            Self::Reserved => Err(Error::InvalidMajorMinorStatus(val)),
            v => Ok(v),
        }
    }
}

impl From<MajorMinorStatus> for u16 {
    fn from(val: MajorMinorStatus) -> Self {
        match val {
            MajorMinorStatus::PowerUp => POWER_UP,
            MajorMinorStatus::PowerUpAcceptor => POWER_UP_ACCEPTOR,
            MajorMinorStatus::PowerUpStacker => POWER_UP_STACKER,
            MajorMinorStatus::PowerUpAcceptorAccepting => POWER_UP_ACCEPTOR_ACCEPTING,
            MajorMinorStatus::PowerUpStackerAccepting => POWER_UP_STACKER_ACCEPTING,
            MajorMinorStatus::Normal => NORMAL,
            MajorMinorStatus::NormalIdle => NORMAL_IDLE,
            MajorMinorStatus::NormalActive => NORMAL_ACTIVE,
            MajorMinorStatus::NormalEscrow => NORMAL_ESCROW,
            MajorMinorStatus::NormalVendValid => NORMAL_VEND_VALID,
            MajorMinorStatus::NormalRejected => NORMAL_REJECTED,
            MajorMinorStatus::NormalReturned => NORMAL_RETURNED,
            MajorMinorStatus::NormalCollected => NORMAL_COLLECTED,
            MajorMinorStatus::NormalInsert => NORMAL_INSERT,
            MajorMinorStatus::NormalConditionalVend => NORMAL_CONDITIONAL_VEND,
            MajorMinorStatus::NormalPause => NORMAL_PAUSE,
            MajorMinorStatus::NormalResume => NORMAL_RESUME,
            MajorMinorStatus::Abnormal => ABNORMAL,
            MajorMinorStatus::AbnormalOperationError => ABNORMAL_OPERATION_ERROR,
            MajorMinorStatus::AbnormalFailure(c) => ABNORMAL | (c as u16),
            MajorMinorStatus::Warning => WARNING,
            MajorMinorStatus::WarningNoteStay => WARNING_NOTE_STAY,
            MajorMinorStatus::WarningFunctionAbeyance => WARNING_FUNCTION_ABEYANCE,
            MajorMinorStatus::Reserved => RESERVED,
        }
    }
}

impl From<&MajorMinorStatus> for &'static str {
    fn from(val: &MajorMinorStatus) -> Self {
        match val {
            MajorMinorStatus::PowerUp => "normal `Power Up` status",
            MajorMinorStatus::PowerUpAcceptor => "device has returnable note on `Power Up`",
            MajorMinorStatus::PowerUpStacker => "device has non-returnable note on `Power Up`", 
            MajorMinorStatus::PowerUpAcceptorAccepting => "device received note before power down, and has returnable note on power up",
            MajorMinorStatus::PowerUpStackerAccepting => "device received note before power down, and has non-returnable note on power up",
            MajorMinorStatus::Normal => "device is disabled",
            MajorMinorStatus::NormalIdle => "acceptor function is in stand-by mode",
            MajorMinorStatus::NormalActive => "device is operational",
            MajorMinorStatus::NormalEscrow => "inserted note passed validation",
            MajorMinorStatus::NormalVendValid => "stacking the note is allowed",
            MajorMinorStatus::NormalRejected => "note left in transport removed during `Power UP`, or returned before `Escrow Status`",
            MajorMinorStatus::NormalReturned => "returning an inserted note after the `Escrow Status` is notified",
            MajorMinorStatus::NormalCollected => "note left in transport collected during `Power Up`, or scanned not collection note completed",
            MajorMinorStatus::NormalInsert => "note has been inserted",
            MajorMinorStatus::NormalConditionalVend => "`Conditional Stacking` is processing",
            MajorMinorStatus::NormalPause => "device has stopped",
            MajorMinorStatus::NormalResume => "device has resumed",
            MajorMinorStatus::Abnormal => "controversial error has been resolved",
            MajorMinorStatus::AbnormalOperationError => "unable to operate, `Operation Message` is not processed",
            MajorMinorStatus::AbnormalFailure(_err) => "fatal error occured",
            MajorMinorStatus::Warning => "warning",
            MajorMinorStatus::WarningNoteStay => "note has stayed in the `Insertion` position for an extended period",
            MajorMinorStatus::WarningFunctionAbeyance => "mode enabled by `Function Mode` is not functional",
            MajorMinorStatus::Reserved => "reserved",
        }
    }
}

impl From<MajorMinorStatus> for &'static str {
    fn from(val: MajorMinorStatus) -> Self {
        (&val).into()
    }
}

impl fmt::Display for MajorMinorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AbnormalFailure(err) => write!(f, r#""{}: {err}""#, <&str>::from(self)),
            _ => write!(f, r#""{}""#, <&str>::from(self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_minor_status() {
        let raw_denom = [
            POWER_UP,
            POWER_UP_ACCEPTOR,
            POWER_UP_STACKER,
            POWER_UP_ACCEPTOR_ACCEPTING,
            POWER_UP_STACKER_ACCEPTING,
            NORMAL,
            NORMAL_IDLE,
            NORMAL_ACTIVE,
            NORMAL_ESCROW,
            NORMAL_VEND_VALID,
            NORMAL_REJECTED,
            NORMAL_RETURNED,
            NORMAL_COLLECTED,
            NORMAL_INSERT,
            NORMAL_CONDITIONAL_VEND,
            NORMAL_PAUSE,
            NORMAL_RESUME,
            ABNORMAL,
            ABNORMAL_OPERATION_ERROR,
            ABNORMAL | (FailureCode::TransportMotor as u16),
            ABNORMAL | (FailureCode::StackMotor as u16),
            ABNORMAL | (FailureCode::AntiStringingMechanism as u16),
            ABNORMAL | (FailureCode::Sensor as u16),
            ABNORMAL | (FailureCode::AcceptorHardware as u16),
            ABNORMAL | (FailureCode::RecyclerMotor as u16),
            ABNORMAL | (FailureCode::RecyclerSensor as u16),
            ABNORMAL | (FailureCode::RecyclyHardware as u16),
            ABNORMAL | (FailureCode::Rom as u16),
            ABNORMAL | (FailureCode::Ram as u16),
            ABNORMAL | (FailureCode::Communication as u16),
            ABNORMAL | (FailureCode::Abnormal as u16),
            WARNING,
            WARNING_NOTE_STAY,
            WARNING_FUNCTION_ABEYANCE,
        ];
        let expected = [
            MajorMinorStatus::PowerUp,
            MajorMinorStatus::PowerUpAcceptor,
            MajorMinorStatus::PowerUpStacker,
            MajorMinorStatus::PowerUpAcceptorAccepting,
            MajorMinorStatus::PowerUpStackerAccepting,
            MajorMinorStatus::Normal,
            MajorMinorStatus::NormalIdle,
            MajorMinorStatus::NormalActive,
            MajorMinorStatus::NormalEscrow,
            MajorMinorStatus::NormalVendValid,
            MajorMinorStatus::NormalRejected,
            MajorMinorStatus::NormalReturned,
            MajorMinorStatus::NormalCollected,
            MajorMinorStatus::NormalInsert,
            MajorMinorStatus::NormalConditionalVend,
            MajorMinorStatus::NormalPause,
            MajorMinorStatus::NormalResume,
            MajorMinorStatus::Abnormal,
            MajorMinorStatus::AbnormalOperationError,
            MajorMinorStatus::AbnormalFailure(FailureCode::TransportMotor),
            MajorMinorStatus::AbnormalFailure(FailureCode::StackMotor),
            MajorMinorStatus::AbnormalFailure(FailureCode::AntiStringingMechanism),
            MajorMinorStatus::AbnormalFailure(FailureCode::Sensor),
            MajorMinorStatus::AbnormalFailure(FailureCode::AcceptorHardware),
            MajorMinorStatus::AbnormalFailure(FailureCode::RecyclerMotor),
            MajorMinorStatus::AbnormalFailure(FailureCode::RecyclerSensor),
            MajorMinorStatus::AbnormalFailure(FailureCode::RecyclyHardware),
            MajorMinorStatus::AbnormalFailure(FailureCode::Rom),
            MajorMinorStatus::AbnormalFailure(FailureCode::Ram),
            MajorMinorStatus::AbnormalFailure(FailureCode::Communication),
            MajorMinorStatus::AbnormalFailure(FailureCode::Abnormal),
            MajorMinorStatus::Warning,
            MajorMinorStatus::WarningNoteStay,
            MajorMinorStatus::WarningFunctionAbeyance,
        ];

        for (raw, exp) in raw_denom.into_iter().zip(expected.into_iter()) {
            assert_eq!(MajorMinorStatus::try_from(raw), Ok(exp));
            assert!(MajorMinorStatus::try_from((raw >> MAJOR_SHIFT) as u8).is_ok());

            assert_eq!(MajorMinorStatus::from_u16(raw), exp);
            assert_ne!(
                MajorMinorStatus::from_u8((raw >> MAJOR_SHIFT) as u8),
                MajorMinorStatus::Reserved
            );
        }

        for stat in
            (0..=0x1fffu16).filter(|s| !raw_denom.iter().any(|&d| d == (*s & MAJOR_MINOR_MASK)))
        {
            assert!(MajorMinorStatus::try_from(stat).is_err());
            assert_eq!(MajorMinorStatus::from_u16(stat), MajorMinorStatus::Reserved);
        }
    }
}
