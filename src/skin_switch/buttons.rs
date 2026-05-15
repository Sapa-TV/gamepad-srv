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
