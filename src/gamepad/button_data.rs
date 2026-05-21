use std::time::Instant;
use strum::IntoEnumIterator;

use crate::gamepad::{buttons::ButtonEnum, event::GamepadEvent};

const COMBINATIONS: &[&[ButtonEnum]] = &[&[
    ButtonEnum::StartSelect,
    ButtonEnum::Start,
    ButtonEnum::Select,
]];

#[non_exhaustive]
#[derive(Debug, Clone)]
struct ButtonData {
    btn: ButtonEnum,
    pressed: bool,
    hold_triggered: bool,
    hold_time: Option<Instant>,
}

impl ButtonData {
    pub fn new(btn: ButtonEnum) -> Self {
        Self {
            btn,
            pressed: false,
            hold_triggered: false,
            hold_time: None,
        }
    }
}

#[non_exhaustive]
pub struct ButtonDataState {
    inner: Vec<ButtonData>,
}

const HOLD_TIME_MILLISECONDS: u128 = 500;

impl ButtonDataState {
    pub fn new() -> Self {
        let inner: Vec<ButtonData> = ButtonEnum::iter().map(|btn| ButtonData::new(btn)).collect();
        Self { inner }
    }

    pub fn update(&mut self, event: &GamepadEvent) -> Vec<GamepadEvent> {
        let mut result: Vec<GamepadEvent> = Vec::new();
        match *event {
            GamepadEvent::ButtonPressed(button) => {
                let button_data = &mut self.inner[button as usize];
                button_data.pressed = true;
                button_data.hold_time = Some(Instant::now());
                result.push(GamepadEvent::ButtonPressed(button));

                let combo_result = Self::get_combos(button);
                for combo in combo_result {
                    if let Some((combo_btn, combo_vec)) = combo.split_first() {
                        let combo_pressed = combo_vec
                            .iter()
                            .all(|btn| self.inner[*btn as usize].pressed);

                        if combo_pressed {
                            result.insert(0, GamepadEvent::ButtonPressed(*combo_btn));
                            let combo_data = &mut self.inner[*combo_btn as usize];
                            combo_data.pressed = true;
                            combo_data.hold_time = Some(Instant::now());
                        }
                    };
                }
            }
            GamepadEvent::ButtonReleased(button) => {
                let button_data = &mut self.inner[button as usize];
                button_data.pressed = false;
                button_data.hold_time = None;
                result.push(GamepadEvent::ButtonReleased(button));

                let combo_result = Self::get_combos(button);
                for combo in combo_result {
                    if self.inner[combo[0] as usize].pressed {
                        result.insert(0, GamepadEvent::ButtonReleased(combo[0]));
                        let button_data = &mut self.inner[combo[0] as usize];
                        button_data.pressed = false;
                        button_data.hold_time = None;
                    }
                }
            }
            _ => {}
        }

        result
    }

    fn get_combos(btn: ButtonEnum) -> Vec<&'static [ButtonEnum]> {
        COMBINATIONS
            .iter()
            .filter(|combo| {
                combo
                    .split_first()
                    .map_or(false, |(_, rest)| rest.contains(&btn))
            })
            .copied() // или .copied(), если элементы дешево копируются
            .collect()
    }

    pub fn tick(&mut self) -> Vec<GamepadEvent> {
        let mut events: Vec<GamepadEvent> = Vec::new();
        for button_data in self.inner.iter_mut() {
            if !button_data.pressed {
                button_data.hold_time = None;
                button_data.hold_triggered = false;
                continue;
            }
            if button_data.hold_triggered {
                continue;
            }
            if let Some(hold_time) = button_data.hold_time {
                let elapsed = hold_time.elapsed().as_millis();
                if elapsed >= HOLD_TIME_MILLISECONDS {
                    events.push(GamepadEvent::ButtonHold(button_data.btn));
                    button_data.hold_triggered = true;
                }
            }
        }

        events
    }
}
