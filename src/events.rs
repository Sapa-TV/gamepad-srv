use gilrs::Event;

use crate::skin_switch::state::Direction;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Gilrs(Event),
    SkinChange(Direction),
}
