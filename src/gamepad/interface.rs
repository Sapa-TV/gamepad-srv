use crate::app::AppCommandEnum;
use serde::Serialize;

pub use super::buttons::ButtonEnum;
use super::{buttons::Buttons, sticks::Stick};

pub trait CommandReceiver: Send + Sync + 'static {
    fn receive_command(&mut self, command: AppCommandEnum);
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct GamepadState {
    #[serde(rename = "ls")]
    pub left_stick: Stick,
    #[serde(rename = "rs")]
    pub right_stick: Stick,
    #[serde(rename = "b")]
    pub buttons: Buttons,
}
