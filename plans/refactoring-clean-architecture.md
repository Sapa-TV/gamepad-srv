# План рефакторинга: Clean Architecture ports

---

## Цель

Разделить domain от infrastructure, соблюдая Dependency Rule:

- Domain не должен зависеть от `gilrs`, `tokio::time`
- Создать порты (traits) для внешних зависимостей
- Вынести magic numbers в константы

---

## Структура после рефакторинга

```
src/
├── gamepad/           # Domain (state, вынести из gilrs зависимость)
├── gamepad_input/     # NEW - infrastructure: port trait + gilrs adapter
├── skin_switch/       # Domain (machine, state, commands - чистые)
├── skin_manager/      # Domain + infra
├── application/       # events, button_actions, tasks
└── ...
```

---

## Шаги

### Этап 1: Создать gamepad_input/ с портом и адаптером

#### Шаг 1.1: Создать `src/gamepad_input/mod.rs`

- [x] Создать модуль `gamepad_input`

#### Шаг 1.2: Создать `src/gamepad_input/port.rs`

- [x] Создать trait `GamepadInput`:

```rust
pub trait GamepadInput: Send + Sync {
    fn next_event(&self) -> Option<GamepadEvent>;
}
```

#### Шаг 1.3: Создать `src/gamepad_input/converter.rs`

- [x] Перенести `button_name()` из `gamepad/state.rs` как `GamepadButton::name()`
- [x] Перенести логику конвертации из `gamepad/event_processor.rs`
- [x] Создать enum `GamepadButton` (вместо `gilrs::Button`)
- [x] Создать enum `GamepadEventType` (Pressed/Released)

#### Шаг 1.4: Создать `src/gamepad_input/adapter.rs`

- [x] Создать `GilrsAdapter` реализующий `GamepadInput`
- [x] Перенести `spawn_gilrs_task()` из `gamepad/input.rs`

#### Шаг 1.5: Обновить `gamepad/mod.rs`

- [x] Перенести экспорты в `gamepad_input/mod.rs`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Этап 2: Очистить domain от infrastructure зависимостей

#### Шаг 2.1: Очистить `src/gamepad/state.rs`

- [ ] Убрать `use gilrs::Button`
- [ ] Убрать `button_name()` (перенесён в gamepad_input)
- [ ] `GamepadState` и `GamepadEvent` остаются как domain

#### Шаг 2.2: Очистить `src/skin_switch/machine.rs`

- [ ] Заменить `tokio::time::Instant` на `std::time::Instant`
- [ ] `deadline()` возвращает `Option<std::time::Instant>`

#### Шаг 2.3: Очистить `src/events.rs`

- [ ] Убрать `use gilrs::Event`
- [ ] `AppEvent::Gilrs` использовать domain тип вместо `gilrs::Event`

#### Шаг 2.4: Удалить `src/skin_switch/buttons.rs`

- [ ] Конвертация перенесена в `gamepad_input/converter.rs`

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Этап 3: Magic numbers в константы

#### Шаг 3.1: Создать `src/constants.rs`

- [ ] `SKIN_SWITCH_TIMEOUT_MS: u64 = 2000`
- [ ] `GAMEPAD_POLL_INTERVAL_MS: u64 = 16`
- [ ] `STICK_TICK_INTERVAL_MS: u64 = 50`
- [ ] `WS_CHANNEL_CAPACITY: usize = 100`
- [ ] `SAVE_CHANNEL_CAPACITY: usize = 32`
- [ ] `AXIS_SCALE: i8 = 127`

#### Шаг 3.2: Заменить magic numbers

- [ ] `skin_switch/machine.rs:67,80` → `SKIN_SWITCH_TIMEOUT_MS`
- [ ] `gamepad_input/adapter.rs` → `GAMEPAD_POLL_INTERVAL_MS`
- [ ] `tasks.rs:24` → `STICK_TICK_INTERVAL_MS`
- [ ] `app.rs:20,21` → `WS_CHANNEL_CAPACITY`
- [ ] `main.rs:62` → `SAVE_CHANNEL_CAPACITY`
- [ ] `event_processor.rs:22` → `AXIS_SCALE`

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Этап 4: Исправить не-mutex unwrap

#### Шаг 4.1: Исправить `src/button_actions.rs:24`

- [ ] `sm.get_current_full().unwrap()` заменить на match:

```rust
let (skin, info) = match sm.get_current_full() {
    Some(result) => result,
    None => {
        tracing::warn!("No current skin loaded");
        return;
    }
};
```

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

## Статус

| Этап | Описание                  | Выполнен |
| ---- | ------------------------- | -------- |
| 1    | Создать gamepad_input/    | [x]      |
| 2    | Очистить domain от infra  | [ ]      |
| 3    | Magic numbers в константы | [ ]      |
| 4    | Исправить не-mutex unwrap | [ ]      |

---

## Всего 4 этапа, 13 задач
