use gilrs::Event;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Gilrs(Event),
}
