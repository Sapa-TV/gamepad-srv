use gilrs::{Event, EventType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonName {
    DPadRight,
    DPadLeft,
    Start,
    Select,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonEvent {
    Pressed(ButtonName),
    Released(ButtonName),
}

pub fn gilrs_event_to_button_event(event: &Event) -> Option<ButtonEvent> {
    match event.event {
        EventType::ButtonPressed(btn, _) => {
            let name = match btn {
                gilrs::Button::DPadRight => ButtonName::DPadRight,
                gilrs::Button::DPadLeft => ButtonName::DPadLeft,
                gilrs::Button::Start => ButtonName::Start,
                gilrs::Button::Select => ButtonName::Select,
                _ => return None,
            };
            Some(ButtonEvent::Pressed(name))
        }
        EventType::ButtonReleased(btn, _) => {
            let name = match btn {
                gilrs::Button::DPadRight => ButtonName::DPadRight,
                gilrs::Button::DPadLeft => ButtonName::DPadLeft,
                gilrs::Button::Start => ButtonName::Start,
                gilrs::Button::Select => ButtonName::Select,
                _ => return None,
            };
            Some(ButtonEvent::Released(name))
        }
        _ => None,
    }
}
