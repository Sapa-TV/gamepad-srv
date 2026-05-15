use crate::skin_switch::buttons::ButtonEvent;

pub trait GamepadInput: Send + Sync {
    fn next_button_event(&mut self) -> Option<ButtonEvent>;
}
