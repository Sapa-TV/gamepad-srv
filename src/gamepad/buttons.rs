use serde::{Serialize, Serializer};
use strum::{AsRefStr, EnumIter, EnumMessage, VariantNames};

#[derive(
    AsRefStr,
    EnumIter,
    VariantNames,
    EnumMessage,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum ButtonEnum {
    #[strum(serialize = "A", message = "South")]
    South,
    #[strum(serialize = "B", message = "East")]
    East,
    #[strum(serialize = "Y", message = "North")]
    North,
    #[strum(serialize = "X", message = "West")]
    West,
    #[strum(serialize = "LB", message = "LeftBar")]
    LeftBar,
    #[strum(serialize = "RB", message = "RightBar")]
    RightBar,
    #[strum(serialize = "LT", message = "LeftTrigger")]
    LeftTrigger,
    #[strum(serialize = "RT", message = "RightTrigger")]
    RightTrigger,
    #[strum(serialize = "LS", message = "LeftStick")]
    LeftStick,
    #[strum(serialize = "RS", message = "RightStick")]
    RightStick,
    #[strum(serialize = "DU", message = "DPadUp")]
    DPadUp,
    #[strum(serialize = "DD", message = "DPadDown")]
    DPadDown,
    #[strum(serialize = "DL", message = "DPadLeft")]
    DPadLeft,
    #[strum(serialize = "DR", message = "DPadRight")]
    DPadRight,
    #[strum(serialize = "ST", message = "Start")]
    Start,
    #[strum(serialize = "SE", message = "Select")]
    Select,
    #[strum(serialize = "MN", message = "Menu")]
    Menu,
    #[strum(serialize = "LSP", message = "LeftStickPressed")]
    LeftStickPressed,
    #[strum(serialize = "RSP", message = "RightStickPressed")]
    RightStickPressed,
    #[strum(serialize = "UNK", message = "Unknown")]
    Unknown,
}

impl From<gilrs::Button> for ButtonEnum {
    fn from(btn: gilrs::Button) -> Self {
        match btn {
            gilrs::Button::South => ButtonEnum::South,
            gilrs::Button::East => ButtonEnum::East,
            gilrs::Button::North => ButtonEnum::North,
            gilrs::Button::West => ButtonEnum::West,
            gilrs::Button::LeftTrigger => ButtonEnum::LeftBar,
            gilrs::Button::RightTrigger => ButtonEnum::RightBar,
            gilrs::Button::LeftTrigger2 => ButtonEnum::LeftTrigger,
            gilrs::Button::RightTrigger2 => ButtonEnum::RightTrigger,
            gilrs::Button::LeftThumb => ButtonEnum::LeftStick,
            gilrs::Button::RightThumb => ButtonEnum::RightStick,
            gilrs::Button::DPadUp => ButtonEnum::DPadUp,
            gilrs::Button::DPadDown => ButtonEnum::DPadDown,
            gilrs::Button::DPadLeft => ButtonEnum::DPadLeft,
            gilrs::Button::DPadRight => ButtonEnum::DPadRight,
            gilrs::Button::Start => ButtonEnum::Start,
            gilrs::Button::Select => ButtonEnum::Select,
            gilrs::Button::Mode => ButtonEnum::Menu,
            _ => ButtonEnum::Unknown,
        }
    }
}

impl Serialize for ButtonEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl ButtonEnum {
    pub fn index(&self) -> u32 {
        *self as u32
    }

    pub fn bit(&self) -> u32 {
        1 << self.index()
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct Buttons(u32);

impl Buttons {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn press(&mut self, button: &ButtonEnum) {
        self.0 |= button.bit();
    }

    pub fn release(&mut self, button: &ButtonEnum) {
        self.0 &= !button.bit();
    }

    pub fn is_pressed(&self, button: &ButtonEnum) -> bool {
        self.0 & button.bit() != 0
    }
}
