use crate::gamepad::{ButtonDataState, GamepadEvent, mapper::InputMapper};

pub trait InputListener: Send + Sync {
    fn handle_raw(&mut self, event: gilrs::Event);
    fn tick(&mut self);
    fn process(&self, events: Vec<GamepadEvent>);
}

#[non_exhaustive]
pub struct AppInputListener<M: InputMapper> {
    mapper: M,
    buttons: ButtonDataState,
}

impl<M: InputMapper> InputListener for AppInputListener<M> {
    fn handle_raw(&mut self, raw_event: gilrs::Event) {
        let gamepad_event: GamepadEvent = raw_event.into();
        if gamepad_event == GamepadEvent::Ignored {
            return;
        }
        let processed = self.buttons.update(&gamepad_event);
        self.process(processed);
    }

    fn tick(&mut self) {
        let processed = self.buttons.tick();
        self.process(processed);
    }

    fn process(&self, events: Vec<GamepadEvent>) {
        for input in events {
            self.mapper.map(&input);
        }
    }
}

impl<M: InputMapper> AppInputListener<M> {
    pub fn build(mapper: M) -> Self {
        Self {
            mapper,
            buttons: ButtonDataState::new(),
        }
    }
}
