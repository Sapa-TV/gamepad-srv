use crate::gamepad::button::ButtonEvent;
use crate::skin_switch::state::Direction;

#[derive(Debug, Clone)]
pub enum AppEvent {
    ButtonEvent(ButtonEvent),
    SkinChange(Direction),
}
