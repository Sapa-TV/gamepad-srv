use std::{
    sync::{Arc, atomic::Ordering},
    time::Duration,
};

use atomic_enum::atomic_enum;
use tokio::{sync::mpsc::Sender, time::timeout};
use tokio_util::sync::{CancellationToken, DropGuard};
use tracing::{debug, error};

use crate::gamepad::{ButtonEnum, event::GamepadEvent};

#[atomic_enum]
#[derive(PartialEq, Eq)]
pub enum AppModeEnum {
    Normal,
    SkinChangePending,
    SkinChangeReady,
    SkinChangeActive,
}

#[derive(Debug, Clone, Copy)]
enum AppModeEvent {
    EnterModePressed,
    EnterModeReleased,
    // Phantom event, direct changes via timer
    #[allow(dead_code)]
    TimerExpired,
    LeaveMode,
}

#[non_exhaustive]
#[derive(Debug)]
pub struct AppModeStateMachine {
    start_pressed: bool,
    select_pressed: bool,
    current_mode: Arc<AtomicAppModeEnum>,
    timer: Option<DropGuard>,
    change_tx: Sender<AppModeEnum>,
}

impl AppModeStateMachine {
    pub fn new(change_tx: Sender<AppModeEnum>) -> Self {
        Self {
            start_pressed: false,
            select_pressed: false,
            current_mode: Arc::new(AtomicAppModeEnum::new(AppModeEnum::Normal)),
            timer: None,
            change_tx,
        }
    }

    fn ctrl_btn_changed(&mut self, event: &GamepadEvent) -> bool {
        let mut buttons_changed = false;
        match event {
            GamepadEvent::ButtonPressed(btn) => match btn {
                ButtonEnum::Start => {
                    self.start_pressed = true;
                    buttons_changed = true;
                }
                ButtonEnum::Select => {
                    self.select_pressed = true;
                    buttons_changed = true;
                }
                _ => {}
            },
            GamepadEvent::ButtonReleased(btn) => match btn {
                ButtonEnum::Start => {
                    self.start_pressed = false;
                    buttons_changed = true;
                }
                ButtonEnum::Select => {
                    self.select_pressed = false;
                    buttons_changed = true;
                }
                _ => {}
            },
            _ => {}
        }

        buttons_changed
    }

    pub fn update(&mut self, event: &GamepadEvent) {
        debug!(
            "Current mode: {:?}",
            self.current_mode.load(Ordering::SeqCst)
        );
        if !self.ctrl_btn_changed(event) {
            return;
        }

        match (self.start_pressed, self.select_pressed) {
            (true, true) => {
                self.handle_event(AppModeEvent::EnterModePressed);
            }
            (false, false) => {
                self.handle_event(AppModeEvent::EnterModeReleased);
            }
            _ => self.handle_event(AppModeEvent::LeaveMode),
        }
    }

    fn start_timer(&mut self) {
        let cancel_token = CancellationToken::new();
        let task_token = cancel_token.clone();
        self.timer = Some(cancel_token.drop_guard());
        let mode_arc = Arc::clone(&self.current_mode);
        let change_tx = self.change_tx.clone();

        tokio::spawn(async move {
            if timeout(Duration::from_secs(2), task_token.cancelled())
                .await
                .is_err()
            {
                if let Ok(_) = mode_arc.compare_exchange(
                    AppModeEnum::SkinChangePending,
                    AppModeEnum::SkinChangeReady,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ) {
                    debug!("Timer expired");
                    if let Err(err) = change_tx.try_send(AppModeEnum::SkinChangeReady) {
                        error!("Internal channel error: {err}");
                    };
                };
            }
        });
    }

    fn stop_timer(&mut self) {
        self.timer = None;
    }

    fn handle_event(&mut self, event: AppModeEvent) {
        debug!("AppModeEvent: {:?}", event);
        use AppModeEnum::*;
        use AppModeEvent::*;
        self.stop_timer();
        let current_mode = *&self.current_mode.load(Ordering::SeqCst);
        let next_mode;
        match (current_mode, event) {
            (Normal, EnterModePressed) => {
                next_mode = SkinChangePending;
                self.start_timer();
            }
            (SkinChangePending, TimerExpired) => {
                next_mode = SkinChangeReady;
            }
            (SkinChangeReady, EnterModeReleased) => {
                next_mode = SkinChangeActive;
            }
            (SkinChangeActive, LeaveMode) => {
                next_mode = Normal;
            }
            _ => {
                next_mode = current_mode;
            }
        }
        debug!("Next mode: {:?}", next_mode);

        if current_mode == next_mode {
            return;
        }
        self.current_mode.store(next_mode, Ordering::SeqCst);

        if let Err(err) = self.change_tx.try_send(next_mode) {
            error!("Internal channel error: {err}");
        };
    }
}
