use crate::events::AppEvent;
use crate::skin_switch::commands::Command;

pub struct SkinSwitchMachine;

impl SkinSwitchMachine {
    pub fn handle(&mut self, event: &AppEvent) -> Option<Command> {
        None
    }
}
