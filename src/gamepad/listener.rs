use crate::gamepad::{
    button_data::ButtonDataState, event::GamepadEvent, gamepad_store::GamepadStoreExt,
    mapper::InputMapper,
};

pub trait InputListener: Send + Sync + 'static {
    fn handle_raw(&mut self, event: gilrs::Event);
    fn tick(&mut self);
    fn process(&mut self, events: Vec<GamepadEvent>);
}

#[non_exhaustive]
pub struct AppInputListener<M: InputMapper, S: GamepadStoreExt> {
    mapper: M,
    state: S,
    buttons: ButtonDataState,
}

impl<M: InputMapper, S: GamepadStoreExt> InputListener for AppInputListener<M, S> {
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

    fn process(&mut self, events: Vec<GamepadEvent>) {
        for input in events {
            self.mapper.map(&input);
            self.state.update(&input);
        }
    }
}

impl<M: InputMapper, S: GamepadStoreExt> AppInputListener<M, S> {
    pub fn build(mapper: M, state: S) -> Self {
        Self {
            mapper,
            state,
            buttons: ButtonDataState::new(),
        }
    }
}
