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
pub enum ButtonName {
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
}

impl Serialize for ButtonName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl ButtonName {
    pub fn index(&self) -> u32 {
        *self as u32
    }

    pub fn bit(&self) -> u32 {
        1 << self.index()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonEvent {
    Pressed(ButtonName),
    Released(ButtonName),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonMask(pub u32);

impl Serialize for ButtonMask {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let button_names = ButtonName::VARIANTS;
        let buf = (0..button_names.len())
            .filter(|i| self.0 & (1 << i) != 0)
            .fold(String::new(), |mut acc, i| {
                if !acc.is_empty() {
                    acc.push(',');
                }
                acc.push_str(button_names[i]);
                acc
            });

        serializer.serialize_str(&buf)
    }
}
