use std::fmt;

use crate::{Error, Result};

const TRANSPORT_MOTOR: u8 = 0x11;
const STACK_MOTOR: u8 = 0x12;
const ANTI_STRINGING_MECHANISM: u8 = 0x13;
const SENSOR: u8 = 0x14;
const ACCEPTOR_HARDWARE: u8 = 0x1f;
const RECYCLER_MOTOR: u8 = 0x22;
const RECYCLER_SENSOR: u8 = 0x24;
const RECYCLER_HARDWARE: u8 = 0x2f;
const ROM: u8 = 0xb1;
const RAM: u8 = 0xb2;
const COMMUNICATION: u8 = 0xb5;
const ABNORMAL: u8 = 0xb6;
const RESERVED: u8 = 0xff;

/// Represents JCM device failure codes.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureCode {
    TransportMotor = TRANSPORT_MOTOR,
    StackMotor = STACK_MOTOR,
    AntiStringingMechanism = ANTI_STRINGING_MECHANISM,
    Sensor = SENSOR,
    AcceptorHardware = ACCEPTOR_HARDWARE,
    RecyclerMotor = RECYCLER_MOTOR,
    RecyclerSensor = RECYCLER_SENSOR,
    RecyclyHardware = RECYCLER_HARDWARE,
    Rom = ROM,
    Ram = RAM,
    Communication = COMMUNICATION,
    Abnormal = ABNORMAL,
    Reserved = RESERVED,
}

impl FailureCode {
    /// Creates a new [FailureCode].
    pub const fn new() -> Self {
        Self::TransportMotor
    }

    /// Infallible conversion from a [`u8`] into a [FailureCode].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            TRANSPORT_MOTOR => Self::TransportMotor,
            STACK_MOTOR => Self::StackMotor,
            ANTI_STRINGING_MECHANISM => Self::AntiStringingMechanism,
            SENSOR => Self::Sensor,
            ACCEPTOR_HARDWARE => Self::AcceptorHardware,
            RECYCLER_MOTOR => Self::RecyclerMotor,
            RECYCLER_SENSOR => Self::RecyclerSensor,
            RECYCLER_HARDWARE => Self::RecyclyHardware,
            ROM => Self::Rom,
            RAM => Self::Ram,
            COMMUNICATION => Self::Communication,
            ABNORMAL => Self::Abnormal,
            _ => Self::Reserved,
        }
    }
}

impl Default for FailureCode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&FailureCode> for &'static str {
    fn from(val: &FailureCode) -> Self {
        match val {
            FailureCode::TransportMotor => "transport motor related error",
            FailureCode::StackMotor => "stack motor related error",
            FailureCode::AntiStringingMechanism => "anti-stringing mechanism error",
            FailureCode::Sensor => "sensor adjustment related error",
            FailureCode::AcceptorHardware => "acceptor hardware related error",
            FailureCode::RecyclerMotor => "recycler motor related error",
            FailureCode::RecyclerSensor => "recycler sensor adjustement related error",
            FailureCode::RecyclyHardware => "recycler hardware related error",
            FailureCode::Rom => "ROM error",
            FailureCode::Ram => "RAM error",
            FailureCode::Communication => "communication failure (no response to message)",
            FailureCode::Abnormal => "abnormal operation sequences (interruption due to communication error or function error due to unconfigured settings)",
            FailureCode::Reserved => "reserved",
        }
    }
}

impl From<FailureCode> for &'static str {
    fn from(val: FailureCode) -> Self {
        (&val).into()
    }
}

impl TryFrom<u8> for FailureCode {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        match Self::from_u8(val) {
            Self::Reserved => Err(Error::InvalidFailureCode(val)),
            code => Ok(code),
        }
    }
}

impl fmt::Display for FailureCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failure_code() {
        let raw_denom = [
            TRANSPORT_MOTOR,
            STACK_MOTOR,
            ANTI_STRINGING_MECHANISM,
            SENSOR,
            ACCEPTOR_HARDWARE,
            RECYCLER_MOTOR,
            RECYCLER_SENSOR,
            RECYCLER_HARDWARE,
            ROM,
            RAM,
            COMMUNICATION,
            ABNORMAL,
        ];
        let expected = [
            FailureCode::TransportMotor,
            FailureCode::StackMotor,
            FailureCode::AntiStringingMechanism,
            FailureCode::Sensor,
            FailureCode::AcceptorHardware,
            FailureCode::RecyclerMotor,
            FailureCode::RecyclerSensor,
            FailureCode::RecyclyHardware,
            FailureCode::Rom,
            FailureCode::Ram,
            FailureCode::Communication,
            FailureCode::Abnormal,
        ];

        for (raw, exp) in raw_denom.into_iter().zip(expected.into_iter()) {
            assert_eq!(FailureCode::try_from(raw), Ok(exp));
            assert_eq!(FailureCode::from_u8(raw), exp);
        }

        for stat in (0..=255u8).filter(|s| !raw_denom.iter().any(|d| d == s)) {
            assert!(FailureCode::try_from(stat).is_err());
            assert_eq!(FailureCode::from_u8(stat), FailureCode::Reserved);
        }
    }
}
