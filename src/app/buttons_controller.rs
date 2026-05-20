use std::{
    sync::{Arc, nonpoison::Mutex},
    time::Duration,
};

use tokio::time::{sleep, timeout};
use tokio_util::sync::{CancellationToken, DropGuard};

use crate::{
    gamepad::{buttons::ButtonEnum, event::GamepadEvent},
    skin_manager::state_machine::{SkinSwitchStateEnum, SkinSwitchStateEvent},
};

pub struct ButtonsControllerAction {
    start_pressed: bool,
    select_pressed: bool,
    skin_switch_manager: Arc<Mutex<SkinSwitchStateEnum>>,
    cancel_token: Option<CancellationToken>,
    active_timer: Option<DropGuard>,
}

impl ButtonsControllerAction {
    pub fn new() -> Self {
        Self {
            select_pressed: false,
            start_pressed: false,
            skin_switch_manager: Arc::new(Mutex::new(SkinSwitchStateEnum::Idle)),
            cancel_token: None,
            active_timer: None,
        }
    }
    pub fn execute(&mut self, event: &GamepadEvent) {
        let old_state = self.skin_switch_manager.lock().clone();
        let mut buttons_changed = false;
        match event {
            GamepadEvent::ButtonPressed(button) => match button {
                ButtonEnum::Select => {
                    self.select_pressed = true;
                    buttons_changed = true;
                }
                ButtonEnum::Start => {
                    self.start_pressed = true;
                    buttons_changed = true;
                }
                _ => {}
            },
            GamepadEvent::ButtonReleased(button) => match button {
                ButtonEnum::Select => {
                    self.select_pressed = false;
                    buttons_changed = true;
                }
                ButtonEnum::Start => {
                    self.start_pressed = false;
                    buttons_changed = true;
                }
                _ => {}
            },
            _ => {
                return;
            }
        }
        if !buttons_changed {
            return;
        }
        match (self.start_pressed, self.select_pressed) {
            (true, true) => {
                self.skin_switch_manager
                    .lock()
                    .handle_event(SkinSwitchStateEvent::EnterModeButtonsPressed);
            }
            (false, false) => {
                self.skin_switch_manager
                    .lock()
                    .handle_event(SkinSwitchStateEvent::EnterModeButtonsReleased);
            }
            _ => {
                self.skin_switch_manager
                    .lock()
                    .handle_event(SkinSwitchStateEvent::LeaveModeButtonPressed);
            }
        }

        let new_state = self.skin_switch_manager.lock().clone();
        if old_state != new_state {
            self.handle_side_effects(self.skin_switch_manager.clone());
        }
    }

    fn handle_side_effects(&mut self, new_state: Arc<Mutex<SkinSwitchStateEnum>>) {
        let cancel_token = CancellationToken::new();
        let task_cancel_token = cancel_token.clone();
        self.active_timer = Some(cancel_token.drop_guard());

        let match_state = { new_state.lock().clone() };
        if let SkinSwitchStateEnum::Pending = match_state {
            tokio::spawn(async move {
                if timeout(Duration::from_secs(2), task_cancel_token.cancelled())
                    .await
                    .is_err()
                {
                    let mut state = new_state.lock();
                    state.handle_event(SkinSwitchStateEvent::TimerExpired);
                }
            });
        }
    }
}
