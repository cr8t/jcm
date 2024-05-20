use crate::{
    Error, Message, MessageCode, MessageData, MessageType, RequestCode, RequestType, Result,
    MAX_DATA_LEN,
};

mod denomination_disable;
mod denomination_disable_mode;

pub use denomination_disable::*;
pub use denomination_disable_mode::*;

/// Represents a `Denomination Disable` request message.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenominationDisableRequest {
    mode: DenominationDisableMode,
    denoms: DenominationDisableList,
}

impl DenominationDisableRequest {
    /// Creates a new [DenominationDisableRequest].
    pub const fn new() -> Self {
        Self {
            mode: DenominationDisableMode::new(),
            denoms: DenominationDisableList::new(),
        }
    }

    /// Gets the [MessageType] for the [DenominationDisableRequest].
    pub const fn message_type(&self) -> MessageType {
        MessageType::Request(self.request_type())
    }

    /// Gets the [RequestType] for the [DenominationDisableRequest].
    pub const fn request_type(&self) -> RequestType {
        self.mode.to_request_type()
    }

    /// Gets the [DenominationDisableMode] for the [DenominationDisableRequest].
    ///
    /// Indirection type for setting the [RequestType].
    pub const fn mode(&self) -> DenominationDisableMode {
        self.mode
    }

    /// Sets the [DenominationDisableMode] for the [DenominationDisableRequest].
    ///
    /// Indirection type for setting the [RequestType].
    pub fn set_mode(&mut self, mode: DenominationDisableMode) {
        self.mode = mode;
    }

    /// Sets the [DenominationDisableMode] for the [DenominationDisableRequest].
    ///
    /// Indirection type for setting the [RequestType].
    pub fn with_mode(mut self, mode: DenominationDisableMode) -> Self {
        self.set_mode(mode);
        self
    }

    /// Gets the [MessageCode] for the [DenominationDisableRequest].
    pub const fn message_code(&self) -> MessageCode {
        MessageCode::Request(self.request_code())
    }

    /// Gets the [RequestCode] for the [DenominationDisableRequest].
    pub const fn request_code(&self) -> RequestCode {
        RequestCode::DenominationDisable
    }

    /// Gets the maximum denomination index.
    pub fn max_denom() -> usize {
        Self::max_denom_len() - 1
    }

    /// Gets the maximum number of denominations.
    pub fn max_denom_len() -> usize {
        MAX_DATA_LEN.saturating_div(DenominationDisable::denom_len())
    }

    /// Gets the current maximum denomination index
    pub fn cur_max_denom(&self) -> usize {
        self.cur_max_denom_len() - 1
    }

    /// Gets the current maximum denomination index
    pub fn cur_max_denom_len(&self) -> usize {
        self.denoms
            .len()
            .saturating_mul(DenominationDisable::denom_len())
    }

    /// Gets a reference to the list of [DenominationDisable] items.
    pub fn denominations(&self) -> &[DenominationDisable] {
        self.denoms.items()
    }

    /// Sets the list of [DenominationDisable] items.
    pub fn set_denominations(&mut self, denoms: &[DenominationDisable]) -> Result<()> {
        let denom_len = denoms.len();
        let max_len = Self::max_denom_len();
        if denom_len > max_len {
            Err(Error::InvalidDenominationLen((denom_len, max_len)))
        } else {
            self.denoms = denoms.into();
            Ok(())
        }
    }

    /// Builder function that sets the list of [DenominationDisable] items.
    pub fn with_denominations(mut self, denoms: &[DenominationDisable]) -> Result<Self> {
        self.set_denominations(denoms)?;
        Ok(self)
    }

    /// Sets a denomination disabled status.
    ///
    /// ## Parameters
    ///
    /// - `idx`: denomination index to set.
    /// - `disable`: whether to disable the denomination.
    pub(crate) fn set_denomination(&mut self, idx: usize, disable: bool) -> Result<()> {
        const DENOM_LEN: usize = DenominationDisable::denom_len();

        if idx > Self::max_denom() {
            Err(Error::InvalidDenominationLen((idx, Self::max_denom())))
        } else if idx > self.cur_max_denom() {
            // get the number of blank denomination sets to add
            let add = (idx / DENOM_LEN).saturating_sub(self.denoms.len());
            let denom_idx = idx % DENOM_LEN;

            self.denoms.append(
                &mut (0..add)
                    .map(|_| DenominationDisable::new())
                    .chain([DenominationDisable::new().with_set(denom_idx, disable)])
                    .collect(),
            );

            Ok(())
        } else {
            let item_idx = idx / DENOM_LEN;
            let denom_idx = idx % DENOM_LEN;
            let len = self.denoms.items().len();

            self.denoms
                .iter_mut()
                .nth(item_idx)
                .ok_or(Error::InvalidDenominationLen((item_idx, len)))?
                .set(denom_idx, disable);

            Ok(())
        }
    }

    /// Disables a denomination at the given index.
    pub fn disable_denomination(&mut self, idx: usize) -> Result<()> {
        self.set_denomination(idx, true)
    }

    /// Enables a denomination at the given index.
    pub fn enable_denomination(&mut self, idx: usize) -> Result<()> {
        self.set_denomination(idx, false)
    }
}

impl Default for DenominationDisableRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&DenominationDisableRequest> for Message {
    fn from(val: &DenominationDisableRequest) -> Self {
        MessageData::from(val).into()
    }
}

impl From<DenominationDisableRequest> for Message {
    fn from(val: DenominationDisableRequest) -> Self {
        (&val).into()
    }
}

impl From<&DenominationDisableRequest> for MessageData {
    fn from(val: &DenominationDisableRequest) -> Self {
        let data = Self::new()
            .with_message_type(val.message_type())
            .with_message_code(val.message_code());
        match val.mode() {
            DenominationDisableMode::Set => data.with_additional(
                val.denoms
                    .iter()
                    .flat_map(|d| d.to_bytes())
                    .collect::<Vec<u8>>()
                    .as_ref(),
            ),
            _ => data,
        }
    }
}

impl From<DenominationDisableRequest> for MessageData {
    fn from(val: DenominationDisableRequest) -> Self {
        (&val).into()
    }
}

impl TryFrom<&Message> for DenominationDisableRequest {
    type Error = Error;

    fn try_from(val: &Message) -> Result<Self> {
        val.data().try_into()
    }
}

impl TryFrom<Message> for DenominationDisableRequest {
    type Error = Error;

    fn try_from(val: Message) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&MessageData> for DenominationDisableRequest {
    type Error = Error;

    fn try_from(val: &MessageData) -> Result<Self> {
        let (exp_type, exp_code) = (
            MessageType::Request(RequestType::Status),
            MessageCode::Request(RequestCode::DenominationDisable),
        );

        // Be generous with data returned by the device.
        // Don't worry about data that isn't a multiple of `DenominationDisable::len()` bytes.
        let denoms = DenominationDisableList::from_bytes(val.additional());

        match (val.message_type().request_type(), val.message_code()) {
            (Ok(msg_type), msg_code)
                if matches!(msg_type, RequestType::Status | RequestType::SetFeature)
                    && msg_code == exp_code =>
            {
                Ok(Self {
                    mode: msg_type.try_into()?,
                    denoms,
                })
            }
            (_, msg_code) => Err(Error::InvalidMessage((
                (val.message_type().into(), msg_code.into()),
                (exp_type.into(), exp_code.into()),
            ))),
        }
    }
}

impl TryFrom<MessageData> for DenominationDisableRequest {
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
        let exp_code = RequestCode::DenominationDisable;
        let exp_denom = DenominationDisable::new();

        for exp_type in [RequestType::Status, RequestType::SetFeature] {
            let (msg_data, exp_req) = if exp_type == RequestType::SetFeature {
                (
                    MessageData::new()
                        .with_message_type(MessageType::Request(exp_type))
                        .with_message_code(MessageCode::Request(exp_code))
                        .with_additional(exp_denom.to_bytes().as_ref()),
                    DenominationDisableRequest::new()
                        .with_mode(DenominationDisableMode::from_request_type(exp_type))
                        .with_denominations(&[exp_denom])?,
                )
            } else {
                (
                    MessageData::new()
                        .with_message_type(MessageType::Request(exp_type))
                        .with_message_code(MessageCode::Request(exp_code)),
                    DenominationDisableRequest::new()
                        .with_mode(DenominationDisableMode::from_request_type(exp_type)),
                )
            };

            let msg = Message::new().with_data(msg_data);

            assert_eq!(exp_req.message_type().request_type(), Ok(exp_type));
            assert_eq!(exp_req.request_type(), exp_type);

            assert_eq!(exp_req.message_code().request_code(), Ok(exp_code));
            assert_eq!(exp_req.request_code(), exp_code);

            assert_eq!(Message::from(&exp_req), msg);
            assert_eq!(
                DenominationDisableRequest::try_from(&msg).as_ref(),
                Ok(&exp_req)
            );
        }

        Ok(())
    }

    #[test]
    fn test_status_request_invalid() -> Result<()> {
        let invalid_types = [MessageType::Reserved]
            .into_iter()
            .chain((0x80..=0x8f).map(|m| MessageType::Event(EventType::from_u8(m))))
            .chain([RequestType::Operation, RequestType::Reserved].map(MessageType::Request))
            .collect::<Vec<MessageType>>();

        let invalid_codes = [
            RequestCode::Uid,
            RequestCode::ProgramSignature,
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
            RequestCode::Status,
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
                    .with_message_code(DenominationDisableRequest::new().message_code());

                let inval_code = MessageData::new()
                    .with_message_type(DenominationDisableRequest::new().message_type())
                    .with_message_code(msg_code);

                for stack_data in [inval_data, inval_type, inval_code] {
                    assert!(DenominationDisableRequest::try_from(&stack_data).is_err());
                    assert!(DenominationDisableRequest::try_from(
                        Message::new().with_data(stack_data)
                    )
                    .is_err());
                }
            }
        }

        Ok(())
    }
}
