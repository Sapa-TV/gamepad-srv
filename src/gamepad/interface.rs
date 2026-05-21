use crate::app::AppCommandEnum;

pub trait CommandReceiver: Send + Sync {
    fn receive_command(&mut self, command: AppCommandEnum);
}
