use crate::gamepad_state::{axis_name, button_name, GamepadEvent, GamepadState};
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
            let axis_name = axis_name(axis);
            let value = (value * 127.0) as i8;
            let changed = match axis {
                Axis::LeftStickX => {
                    if state.left_x != value {
                        state.left_x = value;
                        true
                    } else {
                        false
                    }
                }
                Axis::LeftStickY => {
                    if state.left_y != value {
                        state.left_y = value;
                        true
                    } else {
                        false
                    }
                }
                Axis::RightStickX => {
                    if state.right_x != value {
                        state.right_x = value;
                        true
                    } else {
                        false
                    }
                }
                Axis::RightStickY => {
                    if state.right_y != value {
                        state.right_y = value;
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };
            if changed {
                return Some(GamepadEvent::AxisChanged {
                    axis: axis_name.to_string(),
                    value: value,
                });
            }
        }
        _ => {}
    }
    None
}