use strum::VariantNames;
use serde::{Serialize, Serializer};

#[derive(strum::VariantNames, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
pub enum ButtonName {
    #[strum(serialize = "A")]
    South,
    #[strum(serialize = "B")]
    East,
    #[strum(serialize = "Y")]
    North,
    #[strum(serialize = "X")]
    West,
    #[strum(serialize = "LB")]
    LeftBar,
    #[strum(serialize = "RB")]
    RightBar,
    #[strum(serialize = "LT")]
    LeftTrigger,
    #[strum(serialize = "RT")]
    RightTrigger,
    #[strum(serialize = "LS")]
    LeftStick,
    #[strum(serialize = "RS")]
    RightStick,
    #[strum(serialize = "DU")]
    DPadUp,
    #[strum(serialize = "DD")]
    DPadDown,
    #[strum(serialize = "DL")]
    DPadLeft,
    #[strum(serialize = "DR")]
    DPadRight,
    #[strum(serialize = "ST")]
    Start,
    #[strum(serialize = "SE")]
    Select,
    #[strum(serialize = "MN")]
    Mode,
}

impl ButtonName {
    pub fn index(&self) -> u32 {
        match self {
            ButtonName::South => 0,
            ButtonName::East => 1,
            ButtonName::North => 2,
            ButtonName::West => 3,
            ButtonName::LeftBar => 4,
            ButtonName::RightBar => 5,
            ButtonName::LeftTrigger => 6,
            ButtonName::RightTrigger => 7,
            ButtonName::LeftStick => 8,
            ButtonName::RightStick => 9,
            ButtonName::DPadUp => 10,
            ButtonName::DPadDown => 11,
            ButtonName::DPadLeft => 12,
            ButtonName::DPadRight => 13,
            ButtonName::Start => 14,
            ButtonName::Select => 15,
            ButtonName::Mode => 16,
        }
    }

    pub fn bit(&self) -> u32 {
        1 << self.index()
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonName::South => "A",
            ButtonName::East => "B",
            ButtonName::North => "Y",
            ButtonName::West => "X",
            ButtonName::LeftBar => "LB",
            ButtonName::RightBar => "RB",
            ButtonName::LeftTrigger => "LT",
            ButtonName::RightTrigger => "RT",
            ButtonName::LeftStick => "LS",
            ButtonName::RightStick => "RS",
            ButtonName::DPadUp => "DU",
            ButtonName::DPadDown => "DD",
            ButtonName::DPadLeft => "DL",
            ButtonName::DPadRight => "DR",
            ButtonName::Start => "ST",
            ButtonName::Select => "SE",
            ButtonName::Mode => "MN",
        }
    }

    pub fn from_index(index: u32) -> Option<Self> {
        match index {
            0 => Some(ButtonName::South),
            1 => Some(ButtonName::East),
            2 => Some(ButtonName::North),
            3 => Some(ButtonName::West),
            4 => Some(ButtonName::LeftBar),
            5 => Some(ButtonName::RightBar),
            6 => Some(ButtonName::LeftTrigger),
            7 => Some(ButtonName::RightTrigger),
            8 => Some(ButtonName::LeftStick),
            9 => Some(ButtonName::RightStick),
            10 => Some(ButtonName::DPadUp),
            11 => Some(ButtonName::DPadDown),
            12 => Some(ButtonName::DPadLeft),
            13 => Some(ButtonName::DPadRight),
            14 => Some(ButtonName::Start),
            15 => Some(ButtonName::Select),
            16 => Some(ButtonName::Mode),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonEvent {
    Pressed(ButtonName),
    Released(ButtonName),
}

impl ButtonEvent {
    pub fn button_name(&self) -> ButtonName {
        match self {
            ButtonEvent::Pressed(btn) => *btn,
            ButtonEvent::Released(btn) => *btn,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonMask(pub u32);

impl Serialize for ButtonMask {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let names = ButtonName::VARIANTS;
        let buf = (0..names.len())
            .filter(|i| self.0 & (1 << i) != 0)
            .fold(String::new(), |mut acc, i| {
                if !acc.is_empty() {
                    acc.push(',');
                }
                acc.push_str(names[i]);
                acc
            });

        serializer.serialize_str(&buf)
    }
}
