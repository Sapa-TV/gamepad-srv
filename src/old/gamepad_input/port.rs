use crate::gamepad::button::ButtonEvent;

pub trait GamepadInput {
    fn next_button_event(&mut self) -> Option<ButtonEvent>;
}
