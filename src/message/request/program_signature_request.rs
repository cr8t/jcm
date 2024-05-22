use crate::{
    AlgorithmNumber, Error, HashAlgorithm, Message, MessageCode, MessageData, MessageType,
    RequestCode, RequestType, Result,
};

mod mode;

pub use mode::*;

/// Represents a `ProgramSignature` request message.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProgramSignatureRequest {
    mode: ProgramSignatureMode,
    hash_algorithm: HashAlgorithm,
}

impl ProgramSignatureRequest {
    /// Creates a new [ProgramSignatureRequest].
    pub const fn new() -> Self {
        Self {
            mode: ProgramSignatureMode::new(),
            hash_algorithm: HashAlgorithm::new(),
        }
    }

    /// Gets the [MessageType] for the [ProgramSignatureRequest].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Request(self.request_type())
    }

    /// Gets the [RequestType] for the [ProgramSignatureRequest].
    pub const fn request_type(&self) -> RequestType {
        self.mode.into_request_type()
    }

    /// Gets the [MessageCode] for the [ProgramSignatureRequest].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Request(self.request_code())
    }

    /// Gets the [RequestCode] for the [ProgramSignatureRequest].
    pub const fn request_code(&self) -> RequestCode {
        RequestCode::ProgramSignature
    }

    /// Gets the [ProgramSignatureMode] for the [ProgramSignatureRequest].
    pub const fn mode(&self) -> ProgramSignatureMode {
        self.mode
    }

    /// Sets the [ProgramSignatureMode] for the [ProgramSignatureRequest].
    pub fn set_mode(&mut self, mode: ProgramSignatureMode) {
        self.mode = mode;
    }

    /// Builder function that sets the [ProgramSignatureMode] for the [ProgramSignatureRequest].
    pub fn with_mode(mut self, mode: ProgramSignatureMode) -> Self {
        self.set_mode(mode);
        self
    }

    /// Gets the [AlgorithmNumber] for the [ProgramSignatureRequest].
    pub const fn algorithm_number(&self) -> AlgorithmNumber {
        self.hash_algorithm.algorithm_number()
    }

    /// Sets the [AlgorithmNumber] for the [ProgramSignatureRequest].
    pub fn set_algorithm_number(&mut self, algo: AlgorithmNumber) {
        self.hash_algorithm = HashAlgorithm::from_u8(algo.into_u8()).unwrap_or_default();
    }

    /// Builder function that sets the [AlgorithmNumber] for the [ProgramSignatureRequest].
    pub fn with_algorithm_number(mut self, algo: AlgorithmNumber) -> Self {
        self.set_algorithm_number(algo);
        self
    }

    /// Gets a reference to the [HashAlgorithm] for the [ProgramSignatureRequest].
    pub const fn hash_algorithm(&self) -> &HashAlgorithm {
        &self.hash_algorithm
    }

    /// Sets the [HashAlgorithm] for the [ProgramSignatureRequest].
    pub fn set_hash_algorithm(&mut self, hash: HashAlgorithm) {
        self.hash_algorithm = hash;
    }

    /// Builder function that sets the [HashAlgorithm] for the [ProgramSignatureRequest].
    pub fn with_hash_algorithm(mut self, hash: HashAlgorithm) -> Self {
        self.set_hash_algorithm(hash);
        self
    }
}

impl Default for ProgramSignatureRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ProgramSignatureRequest> for Message {
    fn from(val: ProgramSignatureRequest) -> Self {
        MessageData::from(val).into()
    }
}

impl From<&ProgramSignatureRequest> for Message {
    fn from(val: &ProgramSignatureRequest) -> Self {
        (*val).into()
    }
}

impl From<ProgramSignatureRequest> for MessageData {
    fn from(val: ProgramSignatureRequest) -> Self {
        let additional = match val.mode {
            ProgramSignatureMode::Get => vec![val.algorithm_number().into_u8()],
            ProgramSignatureMode::Set => val.hash_algorithm.into_request(),
        };

        Self::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code())
            .with_additional(additional.as_ref())
    }
}

impl From<&ProgramSignatureRequest> for MessageData {
    fn from(val: &ProgramSignatureRequest) -> Self {
        (*val).into()
    }
}

impl TryFrom<&Message> for ProgramSignatureRequest {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for ProgramSignatureRequest {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&MessageData> for ProgramSignatureRequest {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        let (exp_type, exp_code) = (
            MessageType::Request(RequestType::Status),
            MessageCode::Request(RequestCode::ProgramSignature),
        );

        match val.message_code() {
            MessageCode::Request(RequestCode::ProgramSignature) => Ok(Self {
                mode: val.message_type().try_into()?,
                hash_algorithm: val.additional().try_into()?,
            }),
            msg_code => Err(Error::InvalidMessage((
                (val.message_type().into(), msg_code.into()),
                (exp_type.into(), exp_code.into()),
            ))),
        }
    }
}

impl TryFrom<MessageData> for ProgramSignatureRequest {
    type Error = Error;

    fn try_from(val: MessageData) -> Result<Self> {
        (&val).try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EventCode, EventType};

    #[test]
    fn test_status_request() -> Result<()> {
        let exp_code = MessageCode::Request(RequestCode::ProgramSignature);

        for exp_type in [
            MessageType::Request(RequestType::Status),
            MessageType::Request(RequestType::Operation),
        ] {
            let additional = match exp_type.request_type()? {
                RequestType::Status => vec![AlgorithmNumber::Crc16.into_u8()],
                RequestType::Operation => HashAlgorithm::new().into_request(),
                _ => Vec::new(),
            };

            let msg_data = MessageData::new()
                .with_message_type(exp_type)
                .with_message_code(exp_code)
                .with_additional(additional.as_ref());
            let msg = Message::new().with_data(msg_data);

            let exp_req = ProgramSignatureRequest::new().with_mode(exp_type.try_into()?);

            assert_eq!(exp_req.message_type(), exp_type);
            assert_eq!(exp_type.request_type(), Ok(exp_req.request_type()));

            assert_eq!(exp_req.message_code(), exp_code);
            assert_eq!(exp_code.request_code(), Ok(exp_req.request_code()));

            assert_eq!(Message::from(exp_req), msg);
            assert_eq!(ProgramSignatureRequest::try_from(&msg), Ok(exp_req));
        }

        Ok(())
    }

    #[test]
    fn test_status_request_invalid() -> Result<()> {
        let invalid_types = [MessageType::Reserved]
            .into_iter()
            .chain((0x80..=0x8f).map(|m| MessageType::Event(EventType::from_u8(m))))
            .chain([RequestType::SetFeature, RequestType::Reserved].map(MessageType::Request))
            .collect::<Vec<MessageType>>();

        let invalid_codes = [
            RequestCode::Uid,
            RequestCode::Status,
            RequestCode::Version,
            RequestCode::SerialNumber,
            RequestCode::ModelName,
            RequestCode::Reset,
            RequestCode::Stack,
            RequestCode::Inhibit,
            RequestCode::Collect,
            RequestCode::Key,
            RequestCode::EventResendInterval,
            RequestCode::Idle,
            RequestCode::Reject,
            RequestCode::Hold,
            RequestCode::AcceptorCollect,
            RequestCode::DenominationDisable,
            RequestCode::DirectionDisable,
            RequestCode::CurrencyAssign,
            RequestCode::CashBoxSize,
            RequestCode::NearFull,
            RequestCode::BarCode,
            RequestCode::Insert,
            RequestCode::ConditionalVend,
            RequestCode::Pause,
            RequestCode::NoteDataInfo,
            RequestCode::RecyclerCollect,
            RequestCode::Reserved,
        ]
        .map(MessageCode::Request)
        .into_iter()
        .chain(
            [
                EventCode::PowerUp,
                EventCode::PowerUpAcceptor,
                EventCode::PowerUpStacker,
                EventCode::Inhibit,
                EventCode::ProgramSignature,
                EventCode::Rejected,
                EventCode::Collected,
                EventCode::Clear,
                EventCode::OperationError,
                EventCode::Failure,
                EventCode::NoteStay,
                EventCode::PowerUpAcceptorAccepting,
                EventCode::PowerUpStackerAccepting,
                EventCode::Idle,
                EventCode::Escrow,
                EventCode::VendValid,
                EventCode::AcceptorRejected,
                EventCode::Returned,
                EventCode::AcceptorCollected,
                EventCode::Insert,
                EventCode::ConditionalVend,
                EventCode::Pause,
                EventCode::Resume,
                EventCode::AcceptorClear,
                EventCode::AcceptorOperationError,
                EventCode::AcceptorFailure,
                EventCode::AcceptorNoteStay,
                EventCode::FunctionAbeyance,
                EventCode::Reserved,
            ]
            .map(MessageCode::Event),
        )
        .collect::<Vec<MessageCode>>();

        for &msg_type in invalid_types.iter() {
            for &msg_code in invalid_codes.iter() {
                let inval_data = MessageData::new()
                    .with_message_type(msg_type)
                    .with_message_code(msg_code);

                let inval_type = MessageData::new()
                    .with_message_type(msg_type)
                    .with_message_code(ProgramSignatureRequest::new().message_code());

                let inval_code = MessageData::new()
                    .with_message_type(ProgramSignatureRequest::new().message_type())
                    .with_message_code(msg_code);

                for stack_data in [inval_data, inval_type, inval_code] {
                    assert!(ProgramSignatureRequest::try_from(&stack_data).is_err());
                    assert!(ProgramSignatureRequest::try_from(
                        Message::new().with_data(stack_data)
                    )
                    .is_err());
                }
            }
        }

        Ok(())
    }
}
