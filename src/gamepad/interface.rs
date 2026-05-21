use serde::Serialize;

use crate::{
    app::AppCommandEnum,
    gamepad::{buttons::Buttons, sticks::Stick},
};

pub trait CommandReceiver: Send + Sync {
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
