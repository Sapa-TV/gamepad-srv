use serde::Serialize;

const AXIS_MULTIPLIER: f32 = 127.0;

#[derive(Debug, Clone, Copy)]
pub enum AxisEnum {
    LeftStickX(i8),
    LeftStickY(i8),
    RightStickX(i8),
    RightStickY(i8),
    Unknown,
}

impl From<gilrs::EventType> for AxisEnum {
    fn from(axis_event: gilrs::EventType) -> Self {
        use gilrs::EventType::*;
        match axis_event {
            AxisChanged(gilrs::Axis::LeftStickX, x, _) => {
                Self::LeftStickX((AXIS_MULTIPLIER * x).round() as i8)
            }
            AxisChanged(gilrs::Axis::LeftStickY, y, _) => {
                Self::LeftStickY((AXIS_MULTIPLIER * y).round() as i8)
            }
            AxisChanged(gilrs::Axis::RightStickX, x, _) => {
                Self::RightStickX((AXIS_MULTIPLIER * x).round() as i8)
            }
            AxisChanged(gilrs::Axis::RightStickY, y, _) => {
                Self::RightStickY((AXIS_MULTIPLIER * y).round() as i8)
            }
            _ => Self::Unknown,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct Stick {
    x: i8,
    y: i8,
}

impl Stick {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn update_x(&mut self, x: i8) {
        self.x = x;
    }

    pub fn update_y(&mut self, y: i8) {
        self.y = y;
    }

    pub fn get(&self) -> (i8, i8) {
        (self.x, self.y)
    }
}
