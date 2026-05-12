use gilrs::Event;

use crate::skin_change_state::Direction;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Gilrs(Event),
    SkinChange(Direction),
}
