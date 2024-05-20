use std::fmt;

use crate::DirectionInhibit;

const FACE_DOWN_RIGHT_SIDE_SHIFT: u8 = 3;
const FACE_DOWN_RIGHT_SIDE: u8 = 1 << FACE_DOWN_RIGHT_SIDE_SHIFT;
const FACE_DOWN_LEFT_SIDE_SHIFT: u8 = 2;
const FACE_DOWN_LEFT_SIDE: u8 = 1 << FACE_DOWN_LEFT_SIDE_SHIFT;
const FACE_UP_RIGHT_SIDE_SHIFT: u8 = 1;
const FACE_UP_RIGHT_SIDE: u8 = 1 << FACE_UP_RIGHT_SIDE_SHIFT;
const FACE_UP_LEFT_SIDE_SHIFT: u8 = 0;
const FACE_UP_LEFT_SIDE: u8 = 1 << FACE_UP_LEFT_SIDE_SHIFT;

/// Represents the bitfield mask for [InhibitDirection].
pub const INHIBIT_DIRECTION_MASK: u8 = 0xf;

/// Represents denomination direction to inhibit.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InhibitDirection(u8);

impl InhibitDirection {
    /// Creates a new [InhibitDirection].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [InhibitDirection] from the provided parameter.
    ///
    /// **Note**: Only bits in the [INHIBIT_DIRECTION_MASK] will be set.
    pub const fn create(val: u8) -> Self {
        Self(val & INHIBIT_DIRECTION_MASK)
    }

    /// Gets the bitfield value of the [InhibitDirection].
    pub const fn bits(&self) -> u8 {
        self.0 & INHIBIT_DIRECTION_MASK
    }

    /// Sets the bitfield value of the [InhibitDirection].
    ///
    /// **Note**: Only bits in the [INHIBIT_DIRECTION_MASK] will be set.
    pub fn set(&mut self, val: u8) {
        self.0 = val & INHIBIT_DIRECTION_MASK;
    }

    /// Gets whether the `face-down-right-side-first` direction is inhibited.
    pub const fn face_down_right_side(&self) -> DirectionInhibit {
        DirectionInhibit::from_bool(self.0 & FACE_DOWN_RIGHT_SIDE != 0)
    }

    /// Sets whether the `face-down-right-side-first` direction is inhibited.
    pub fn set_face_down_right_side(&mut self, val: DirectionInhibit) {
        self.0 &= !FACE_DOWN_RIGHT_SIDE;
        self.0 |= u8::from(val) << FACE_DOWN_RIGHT_SIDE_SHIFT;
    }

    /// Builder function that sets whether the `face-down-right-side-first` direction is inhibited.
    pub fn with_face_down_right_side(mut self, val: DirectionInhibit) -> Self {
        self.set_face_down_right_side(val);
        self
    }

    /// Gets whether the `face-down-left-side-first` direction is inhibited.
    pub const fn face_down_left_side(&self) -> DirectionInhibit {
        DirectionInhibit::from_bool(self.0 & FACE_DOWN_LEFT_SIDE != 0)
    }

    /// Sets whether the `face-down-left-side-first` direction is inhibited.
    pub fn set_face_down_left_side(&mut self, val: DirectionInhibit) {
        self.0 &= !FACE_DOWN_LEFT_SIDE;
        self.0 |= u8::from(val) << FACE_DOWN_LEFT_SIDE_SHIFT;
    }

    /// Builder function that sets whether the `face-down-left-side-first` direction is inhibited.
    pub fn with_face_down_left_side(mut self, val: DirectionInhibit) -> Self {
        self.set_face_down_left_side(val);
        self
    }

    /// Gets whether the `face-up-right-side-first` direction is inhibited.
    pub const fn face_up_right_side(&self) -> DirectionInhibit {
        DirectionInhibit::from_bool(self.0 & FACE_UP_RIGHT_SIDE != 0)
    }

    /// Sets whether the `face-up-right-side-first` direction is inhibited.
    pub fn set_face_up_right_side(&mut self, val: DirectionInhibit) {
        self.0 &= !FACE_UP_RIGHT_SIDE;
        self.0 |= u8::from(val) << FACE_UP_RIGHT_SIDE_SHIFT;
    }

    /// Builder function that sets whether the `face-up-right-side-first` direction is inhibited.
    pub fn with_face_up_right_side(mut self, val: DirectionInhibit) -> Self {
        self.set_face_up_right_side(val);
        self
    }

    /// Gets whether the `face-up-left-side-first` direction is inhibited.
    pub const fn face_up_left_side(&self) -> DirectionInhibit {
        DirectionInhibit::from_bool(self.0 & FACE_UP_LEFT_SIDE != 0)
    }

    /// Sets whether the `face-up-left-side-first` direction is inhibited.
    pub fn set_face_up_left_side(&mut self, val: DirectionInhibit) {
        self.0 &= !FACE_UP_LEFT_SIDE;
        self.0 |= u8::from(val) << FACE_UP_LEFT_SIDE_SHIFT;
    }

    /// Builder function that sets whether the `face-up-left-side-first` direction is inhibited.
    pub fn with_face_up_left_side(mut self, val: DirectionInhibit) -> Self {
        self.set_face_up_left_side(val);
        self
    }

    /// Gets the length of the [InhibitDirection].
    pub const fn len() -> usize {
        std::mem::size_of::<u8>()
    }

    /// Gets whether the [InhibitDirection] is empty.
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for InhibitDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(
            f,
            r#""face_down_right_side": {}, "#,
            self.face_down_right_side()
        )?;
        write!(
            f,
            r#""face_down_left_side": {}, "#,
            self.face_down_left_side()
        )?;
        write!(
            f,
            r#""face_up_right_side": {}, "#,
            self.face_up_right_side()
        )?;
        write!(f, r#""face_up_left_side": {}"#, self.face_up_left_side())?;
        write!(f, "}}")
    }
}

impl Default for InhibitDirection {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u8> for InhibitDirection {
    fn from(val: u8) -> Self {
        Self::create(val)
    }
}

impl From<InhibitDirection> for u8 {
    fn from(val: InhibitDirection) -> Self {
        val.bits()
    }
}

impl From<&InhibitDirection> for u8 {
    fn from(val: &InhibitDirection) -> Self {
        val.bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inhibit_direction() {
        let raw_face_down_right = 1 << 3;
        let raw_face_down_left = 1 << 2;
        let raw_face_up_right = 1 << 1;
        let raw_face_up_left = 1 << 0;
        let raw_all =
            raw_face_down_right | raw_face_down_left | raw_face_up_right | raw_face_up_left;

        let no_inhibit = InhibitDirection::new();

        assert_eq!(no_inhibit.face_down_right_side(), DirectionInhibit::Accept);
        assert_eq!(no_inhibit.face_down_left_side(), DirectionInhibit::Accept);
        assert_eq!(no_inhibit.face_up_right_side(), DirectionInhibit::Accept);
        assert_eq!(no_inhibit.face_up_left_side(), DirectionInhibit::Accept);

        assert_eq!(no_inhibit.bits(), 0);

        let all_inhibit = InhibitDirection::new()
            .with_face_down_right_side(DirectionInhibit::Inhibit)
            .with_face_down_left_side(DirectionInhibit::Inhibit)
            .with_face_up_right_side(DirectionInhibit::Inhibit)
            .with_face_up_left_side(DirectionInhibit::Inhibit);

        assert_eq!(
            all_inhibit.face_down_right_side(),
            DirectionInhibit::Inhibit
        );
        assert_eq!(all_inhibit.face_down_left_side(), DirectionInhibit::Inhibit);
        assert_eq!(all_inhibit.face_up_right_side(), DirectionInhibit::Inhibit);
        assert_eq!(all_inhibit.face_up_left_side(), DirectionInhibit::Inhibit);

        assert_eq!(all_inhibit.bits(), raw_all);

        assert_eq!(InhibitDirection::create(raw_all), all_inhibit);
        assert_eq!(
            InhibitDirection::create(raw_face_down_right).bits(),
            raw_face_down_right
        );
        assert_eq!(
            InhibitDirection::create(raw_face_down_left).bits(),
            raw_face_down_left
        );
        assert_eq!(
            InhibitDirection::create(raw_face_up_right).bits(),
            raw_face_up_right
        );
        assert_eq!(
            InhibitDirection::create(raw_face_up_left).bits(),
            raw_face_up_left
        );
    }
}
