use crate::skin_switch::state::Direction;

#[derive(Debug)]
pub enum Command {
    SkinChange(Direction),
    NotifySkinChanging(bool),
    SkinSwitchReady,
}
