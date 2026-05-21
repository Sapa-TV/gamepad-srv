use crate::{
    gamepad::{GamepadState, buttons::ButtonEnum, event::GamepadEvent, sticks::AxisEnum},
    server::GamepadStateSender,
};

pub trait GamepadStoreExt: Send + Sync {
    fn update(&mut self, event: &GamepadEvent);
}

pub struct GamepadStore<GSS> {
    gamepad_state: GamepadState,
    state_sender: GSS,
}

impl<GSS: GamepadStateSender> GamepadStore<GSS> {
    pub fn new(state_sender: GSS) -> Self {
        Self {
            gamepad_state: GamepadState::default(),
            state_sender,
        }
    }
}

impl<GSS: GamepadStateSender> GamepadStoreExt for GamepadStore<GSS> {
    fn update(&mut self, event: &GamepadEvent) {
        self.gamepad_state.update(event);
        self.state_sender
            .send_gamepad_state(self.gamepad_state.clone());
    }
}

trait GamepadStateExt {
    fn button_press(&mut self, button: &ButtonEnum);
    fn button_release(&mut self, button: &ButtonEnum);
    fn stick_update(&mut self, axis: &AxisEnum);
    fn update(&mut self, event: &GamepadEvent);
}

impl GamepadStateExt for GamepadState {
    fn stick_update(&mut self, axis: &AxisEnum) {
        match axis {
            AxisEnum::LeftStickX(value) => self.left_stick.update_x(*value),
            AxisEnum::LeftStickY(value) => self.left_stick.update_y(*value),
            AxisEnum::RightStickX(value) => self.right_stick.update_x(*value),
            AxisEnum::RightStickY(value) => self.right_stick.update_y(*value),
            _ => {}
        }
    }

    fn button_press(&mut self, button: &ButtonEnum) {
        self.buttons.press(button);
    }

    fn button_release(&mut self, button: &ButtonEnum) {
        self.buttons.release(button);
    }
    fn update(&mut self, event: &GamepadEvent) {
        // debug!("Gamepad event: {:?}", event);
        match event {
            GamepadEvent::ButtonPressed(button) => self.button_press(button),
            GamepadEvent::ButtonReleased(button) => self.button_release(button),
            GamepadEvent::AxisMoved(axis_change) => self.stick_update(axis_change),
            _ => {}
        }
    }
}
