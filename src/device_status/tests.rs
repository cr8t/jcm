use super::*;
use crate::FailureCode;

// Convenience aliases for readability.
type DS = DeviceStatus;
type FC = FailureCode;
type FM = FunctionMode;
type MS = MajorMinorStatus;

#[test]
fn test_device_status() {
    let common = u16::from(FM::Common);
    let acceptor = u16::from(FM::Acceptor);

    let raw_vals = [
        common | u16::from(MS::PowerUp),
        common | u16::from(MS::PowerUpAcceptor),
        common | u16::from(MS::PowerUpStacker),
        common | u16::from(MS::Normal),
        common | u16::from(MS::NormalActive),
        common | u16::from(MS::NormalRejected),
        common | u16::from(MS::NormalCollected),
        common | u16::from(MS::Abnormal),
        common | u16::from(MS::AbnormalOperationError),
        common | u16::from(MS::WarningNoteStay),
        common | u16::from(MS::AbnormalFailure(FC::TransportMotor)),
        common | u16::from(MS::AbnormalFailure(FC::StackMotor)),
        common | u16::from(MS::AbnormalFailure(FC::AntiStringingMechanism)),
        common | u16::from(MS::AbnormalFailure(FC::Sensor)),
        common | u16::from(MS::AbnormalFailure(FC::AcceptorHardware)),
        common | u16::from(MS::AbnormalFailure(FC::RecyclerMotor)),
        common | u16::from(MS::AbnormalFailure(FC::RecyclerSensor)),
        common | u16::from(MS::AbnormalFailure(FC::RecyclyHardware)),
        common | u16::from(MS::AbnormalFailure(FC::Rom)),
        common | u16::from(MS::AbnormalFailure(FC::Ram)),
        common | u16::from(MS::AbnormalFailure(FC::Communication)),
        common | u16::from(MS::AbnormalFailure(FC::Abnormal)),
        acceptor | u16::from(MS::PowerUpAcceptorAccepting),
        acceptor | u16::from(MS::PowerUpStackerAccepting),
        acceptor | u16::from(MS::Normal),
        acceptor | u16::from(MS::NormalIdle),
        acceptor | u16::from(MS::NormalActive),
        acceptor | u16::from(MS::NormalEscrow),
        acceptor | u16::from(MS::NormalVendValid),
        acceptor | u16::from(MS::NormalRejected),
        acceptor | u16::from(MS::NormalReturned),
        acceptor | u16::from(MS::NormalCollected),
        acceptor | u16::from(MS::NormalInsert),
        acceptor | u16::from(MS::NormalConditionalVend),
        acceptor | u16::from(MS::NormalPause),
        acceptor | u16::from(MS::NormalResume),
        acceptor | u16::from(MS::Abnormal),
        acceptor | u16::from(MS::AbnormalOperationError),
        acceptor | u16::from(MS::WarningNoteStay),
        acceptor | u16::from(MS::WarningFunctionAbeyance),
        acceptor | u16::from(MS::AbnormalFailure(FC::TransportMotor)),
        acceptor | u16::from(MS::AbnormalFailure(FC::StackMotor)),
        acceptor | u16::from(MS::AbnormalFailure(FC::AntiStringingMechanism)),
        acceptor | u16::from(MS::AbnormalFailure(FC::Sensor)),
        acceptor | u16::from(MS::AbnormalFailure(FC::AcceptorHardware)),
        acceptor | u16::from(MS::AbnormalFailure(FC::RecyclerMotor)),
        acceptor | u16::from(MS::AbnormalFailure(FC::RecyclerSensor)),
        acceptor | u16::from(MS::AbnormalFailure(FC::RecyclyHardware)),
        acceptor | u16::from(MS::AbnormalFailure(FC::Rom)),
        acceptor | u16::from(MS::AbnormalFailure(FC::Ram)),
        acceptor | u16::from(MS::AbnormalFailure(FC::Communication)),
        acceptor | u16::from(MS::AbnormalFailure(FC::Abnormal)),
    ];

    let expected = [
        DS::create(FM::Common, MS::PowerUp),
        DS::create(FM::Common, MS::PowerUpAcceptor),
        DS::create(FM::Common, MS::PowerUpStacker),
        DS::create(FM::Common, MS::Normal),
        DS::create(FM::Common, MS::NormalActive),
        DS::create(FM::Common, MS::NormalRejected),
        DS::create(FM::Common, MS::NormalCollected),
        DS::create(FM::Common, MS::Abnormal),
        DS::create(FM::Common, MS::AbnormalOperationError),
        DS::create(FM::Common, MS::WarningNoteStay),
        DS::create(FM::Common, MS::AbnormalFailure(FC::TransportMotor)),
        DS::create(FM::Common, MS::AbnormalFailure(FC::StackMotor)),
        DS::create(FM::Common, MS::AbnormalFailure(FC::AntiStringingMechanism)),
        DS::create(FM::Common, MS::AbnormalFailure(FC::Sensor)),
        DS::create(FM::Common, MS::AbnormalFailure(FC::AcceptorHardware)),
        DS::create(FM::Common, MS::AbnormalFailure(FC::RecyclerMotor)),
        DS::create(FM::Common, MS::AbnormalFailure(FC::RecyclerSensor)),
        DS::create(FM::Common, MS::AbnormalFailure(FC::RecyclyHardware)),
        DS::create(FM::Common, MS::AbnormalFailure(FC::Rom)),
        DS::create(FM::Common, MS::AbnormalFailure(FC::Ram)),
        DS::create(FM::Common, MS::AbnormalFailure(FC::Communication)),
        DS::create(FM::Common, MS::AbnormalFailure(FC::Abnormal)),
        DS::create(FM::Acceptor, MS::PowerUpAcceptorAccepting),
        DS::create(FM::Acceptor, MS::PowerUpStackerAccepting),
        DS::create(FM::Acceptor, MS::Normal),
        DS::create(FM::Acceptor, MS::NormalIdle),
        DS::create(FM::Acceptor, MS::NormalActive),
        DS::create(FM::Acceptor, MS::NormalEscrow),
        DS::create(FM::Acceptor, MS::NormalVendValid),
        DS::create(FM::Acceptor, MS::NormalRejected),
        DS::create(FM::Acceptor, MS::NormalReturned),
        DS::create(FM::Acceptor, MS::NormalCollected),
        DS::create(FM::Acceptor, MS::NormalInsert),
        DS::create(FM::Acceptor, MS::NormalConditionalVend),
        DS::create(FM::Acceptor, MS::NormalPause),
        DS::create(FM::Acceptor, MS::NormalResume),
        DS::create(FM::Acceptor, MS::Abnormal),
        DS::create(FM::Acceptor, MS::AbnormalOperationError),
        DS::create(FM::Acceptor, MS::WarningNoteStay),
        DS::create(FM::Acceptor, MS::WarningFunctionAbeyance),
        DS::create(FM::Acceptor, MS::AbnormalFailure(FC::TransportMotor)),
        DS::create(FM::Acceptor, MS::AbnormalFailure(FC::StackMotor)),
        DS::create(
            FM::Acceptor,
            MS::AbnormalFailure(FC::AntiStringingMechanism),
        ),
        DS::create(FM::Acceptor, MS::AbnormalFailure(FC::Sensor)),
        DS::create(FM::Acceptor, MS::AbnormalFailure(FC::AcceptorHardware)),
        DS::create(FM::Acceptor, MS::AbnormalFailure(FC::RecyclerMotor)),
        DS::create(FM::Acceptor, MS::AbnormalFailure(FC::RecyclerSensor)),
        DS::create(FM::Acceptor, MS::AbnormalFailure(FC::RecyclyHardware)),
        DS::create(FM::Acceptor, MS::AbnormalFailure(FC::Rom)),
        DS::create(FM::Acceptor, MS::AbnormalFailure(FC::Ram)),
        DS::create(FM::Acceptor, MS::AbnormalFailure(FC::Communication)),
        DS::create(FM::Acceptor, MS::AbnormalFailure(FC::Abnormal)),
    ];

    for (raw, exp) in raw_vals.into_iter().zip(expected.into_iter()) {
        assert_eq!(DS::try_from(raw), Ok(exp));
        assert_eq!(DS::from_u16(raw), exp);
    }

    for stat in (0..=0x1fffu16).filter(|s| !raw_vals.iter().any(|d| d == s)) {
        assert!(DS::try_from(stat).is_err());
        assert!(!DS::from_u16(stat).is_valid());
    }
}
