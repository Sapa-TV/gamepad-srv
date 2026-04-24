use crate::gamepad_state::{button_name, GamepadEvent, GamepadState};
use gilrs::{Axis, Event, EventType};

pub fn process_event(state: &mut GamepadState, event: Event) -> Option<GamepadEvent> {
    match event.event {
        EventType::ButtonPressed(btn, _) => {
            let name = button_name(btn).to_string();
            if !state.buttons.contains(&name) {
                state.buttons.push(name.clone());
                state.buttons.sort();
                return Some(GamepadEvent::ButtonPressed(name));
            }
        }
        EventType::ButtonReleased(btn, _) => {
            let name = button_name(btn).to_string();
            if state.buttons.contains(&name) {
                state.buttons.retain(|b| b != &name);
                return Some(GamepadEvent::ButtonReleased(name));
            }
        }
        EventType::AxisChanged(axis, value, _) => {
            let value = (value * 127.0) as i8;
            match axis {
                Axis::LeftStickX => { state.left_x = value; }
                Axis::LeftStickY => { state.left_y = value; }
                Axis::RightStickX => { state.right_x = value; }
                Axis::RightStickY => { state.right_y = value; }
                _ => {}
            };
        }
        _ => {}
    }
    None
}