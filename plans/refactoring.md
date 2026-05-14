# Инкрементальный план рефакторинга Gamepad Overlay Server

Каждый шаг — законченная единица работы, после которой код компилируется и работает.

---

## Цель

Переход от "Screaming Architecture" (вариант B) со следующей структурой:

```
src/
  gamepad/           # gamepad_state, input driver
    mod.rs
    state.rs         # GamepadState, GamepadEvent, button_name
    event_processor.rs
    input.rs         # gilrs loop
  skin_manager/      # Управление списком скинов
    mod.rs
    manager.rs       # SkinManager: next(), prev(), get_current()
    discovery.rs     # discover_skins(), validate_skin(), load_skin_info()
  skin_switch/       # State machine переключения скинов
    mod.rs
    state.rs         # AppSkinState, Direction, SkinChangeState
    commands.rs      # Command enum
    machine.rs       # handle_event() — логика переходов
  websocket/          # Сетевой вывод
    mod.rs
    handler.rs       # handle_socket()
  config/
    mod.rs
  events.rs          # AppEvent enum
  handlers.rs        # HTTP handlers
  app.rs             # AppState, Channels
  tasks.rs           # Orchestration задач
  main.rs            # Точка входа
```

---

## Шаги

### Шаг 0: Создать структуру папок

- [x] **Создать:**
  - `src/gamepad/mod.rs`
  - `src/skin_manager/mod.rs`
  - `src/skin_switch/mod.rs`
  - `src/websocket/mod.rs`
  - `src/config/mod.rs`

**Действие:** Просто создать пустые mod.rs файлы

**Проверка:** `cargo check` ✓

---

### Шаг 1: Перенести `gamepad/state.rs`

- [x] **Создать:** `src/gamepad/state.rs` — скопировать содержимое `gamepad_state.rs`

- [x] **Создать:** `src/gamepad/input.rs` — пока пустой

- [x] **Обновить:** `gamepad_state.rs` → оставить только `pub use crate::gamepad::state::*;` (re-export)

**Проверка:** `cargo check` ✓

---

### Шаг 2: Перенести `skin_manager/discovery.rs`

- [ ] **Создать:** `src/skin_manager/discovery.rs` — скопировать `skin.rs`

- [ ] **Обновить:** `skin.rs` → `pub use crate::skin_manager::discovery::*;`

**Проверка:** `cargo check`

---

### Шаг 3: Перенести `skin_manager/manager.rs`

- [ ] **Создать:** `src/skin_manager/manager.rs`

```rust
use super::discovery::{SkinEntry, SkinInfo, load_skin_info};

pub struct SkinManager {
    skins: Vec<SkinEntry>,
    current_idx: usize,
}

impl SkinManager {
    pub fn new(skins: Vec<SkinEntry>) -> Self { ... }
    pub fn get_current(&self) -> Option<&SkinEntry> { ... }
    pub fn next(&mut self) -> &SkinEntry { ... }
    pub fn prev(&mut self) -> &SkinEntry { ... }
}
```

**Проверка:** `cargo check`

---

### Шаг 4: Перенести `skin_switch/state.rs`

- [ ] **Создать:** `src/skin_switch/state.rs` — скопировать `skin_change_state.rs`

- [ ] **Обновить:** `skin_change_state.rs` → `pub use crate::skin_switch::state::*;`

**Проверка:** `cargo check`

---

### Шаг 5: Перенести `websocket/handler.rs`

- [ ] **Создать:** `src/websocket/handler.rs` — скопировать `ws.rs`

- [ ] **Обновить:** `ws.rs` → `pub use crate::websocket::handler::*;`

**Проверка:** `cargo check`

---

### Шаг 6: Перенести `config/mod.rs`

- [ ] **Создать:** `src/config/mod.rs` — содержимое `config.rs`

- [ ] **Обновить:** `config.rs` → `pub use crate::config::mod::*;`

**Проверка:** `cargo check`

---

### Шаг 7: Создать `commands.rs` в skin_switch

- [ ] **Создать:** `src/skin_switch/commands.rs`

```rust
use crate::skin_switch::state::Direction;

#[derive(Debug)]
pub enum Command {
    SkinChange(Direction),
    NotifySkinChanging(bool),
    SkinSwitchReady,
}
```

- [ ] **Обновить:** `skin_switch/mod.rs` → `pub mod state; pub mod commands;`

**Проверка:** `cargo check`

---

### Шаг 8: Создать `skin_switch/machine.rs` (логика переходов)

- [ ] **Создать:** `src/skin_switch/machine.rs` — пока простой skeleton:

```rust
use crate::events::AppEvent;
use crate::skin_switch::commands::Command;

pub struct SkinSwitchMachine;

impl SkinSwitchMachine {
    pub fn handle(&mut self, event: &AppEvent) -> Option<Command> { None }
}
```

- [ ] **Обновить:** `skin_switch/mod.rs` → `pub mod machine;`

**Проверка:** `cargo check`

---

### Шаг 9: Перенести `event_processor.rs`

- [ ] **Создать:** `src/gamepad/event_processor.rs` — скопировать содержимое

- [ ] **Обновить:** `event_processor.rs` → `pub use crate::gamepad::event_processor::*;`

**Проверка:** `cargo check`

---

### Шаг 10: Обновить `handlers.rs`

- [ ] **Обновить импорты в `handlers.rs`:**
  - `use crate::ws::handle_socket` → `use crate::websocket::handler::handle_socket`

- [ ] **Убрать:** `ws.rs` ре-экспорт (теперь handlers.rs будет использовать напрямую)

**Проверка:** `cargo check`

---

### Шаг 11: Обновить `app.rs`

- [ ] **Обновить импорты в `app.rs`:**
  - Убрать `use crate::skin::...` → `use crate::skin_manager::discovery::...`
  - Убрать `use crate::gamepad_state::...` → `use crate::gamepad::state::...`

**Проверка:** `cargo check`

---

### Шаг 12: Обновить `tasks.rs`

- [ ] **Обновить импорты в `tasks.rs`:**
  - `use crate::gamepad_state::...` → `use crate::gamepad::state::...`
  - `use crate::skin_change_state::...` → `use crate::skin_switch::state::...`
  - `use crate::event_processor::...` → `use crate::gamepad::event_processor::...`

**Проверка:** `cargo check`

---

### Шаг 13: Обновить `button_actions.rs`

- [ ] **Обновить импорты:**
  - `use crate::skin::...` → `use crate::skin_manager::discovery::...`
  - `use crate::skin_change_state::...` → `use crate::skin_switch::state::...`

**Проверка:** `cargo check`

---

### Шаг 14: Обновить `main.rs`

- [ ] **Обновить импорты:**
  - `use crate::app::{Channels, create_app_state}` — пока оставить как есть
  - `use crate::tasks::spawn_stick_tick` — пока оставить

**Проверка:** `cargo check`

---

### Шаг 15: Удалить re-export файлы

- [ ] **Удалить:**
  - `src/gamepad_state.rs`
  - `src/skin.rs`
  - `src/skin_change_state.rs`
  - `src/ws.rs`
  - `src/config.rs`
  - `src/event_processor.rs`

- [ ] **Обновить:** `main.rs` — убрать `mod gamepad_state;`, `mod skin;`, etc.

**Проверка:** `cargo check`

---

### Шаг 16: Обновить `main.rs` — убрать лишние импорты

- [ ] **Убрать из main.rs:**
  - `mod app; mod button_actions; mod config; mod event_processor; mod events; mod gamepad_state; mod handlers; mod skin; mod skin_change_state; mod tasks; mod ws;`
  - Заменить на новые `mod gamepad; mod skin_manager; mod skin_switch; mod websocket; mod config; mod events; mod handlers; mod tasks;`

**Проверка:** `cargo check`

---

### Шаг 17: Реализовать `SkinSwitchMachine::handle()`

- [ ] Перенести логику из `tasks.rs::spawn_skin_change_tracker` в `skin_switch/machine.rs`:
  - Метод `handle(&mut self, event: &AppEvent) -> Option<Command>`
  - Все переходы состояний из текущего loop

**Проверка:** `cargo check`

---

### Шаг 18: Обновить `tasks.rs` — использовать новый machine

- [ ] **Обновить `spawn_skin_change_tracker`:**
  - Создать экземпляр `SkinSwitchMachine`
  - В цикле вызывать `machine.handle(&event)` вместо direct state manipulation
  - Убрать прямой доступ к `button_state.lock()`

**Проверка:** `cargo check`

---

### Шаг 19: Оптимизировать таймер

- [ ] **В `skin_switch/machine.rs`:**
  - При переходе в `SkinSwitchPending` сохранять `deadline = Instant::now() + 2secs`
  - В loop использовать `time::sleep_until(deadline)` вместо `sleep(100)`

**Проверка:** `cargo run` — убедиться что всё работает

---

### Шаг 20: (Опционально) Очистить button_actions.rs

- [ ] **Если `button_actions.rs` больше не нужен** — удалить и перенести функциональность в machine или skin_manager

**Проверка:** `cargo check && cargo run`

---

## Статус

| Шаг | Описание                            | Статус |
| --- | ----------------------------------- | ------ |
| 0   | Создать структуру папок             | [x]    |
| 1   | Перенести gamepad/state.rs          | [x]    |
| 2   | Перенести skin_manager/discovery.rs | - [ ]  |
| 3   | Перенести skin_manager/manager.rs   | - [ ]  |
| 4   | Перенести skin_switch/state.rs      | - [ ]  |
| 5   | Перенести websocket/handler.rs      | - [ ]  |
| 6   | Перенести config/mod.rs             | - [ ]  |
| 7   | Создать commands.rs                 | - [ ]  |
| 8   | Создать machine.rs                  | - [ ]  |
| 9   | Перенести event_processor.rs        | - [ ]  |
| 10  | Обновить handlers.rs                | - [ ]  |
| 11  | Обновить app.rs                     | - [ ]  |
| 12  | Обновить tasks.rs                   | - [ ]  |
| 13  | Обновить button_actions.rs          | - [ ]  |
| 14  | Обновить main.rs                    | - [ ]  |
| 15  | Удалить re-export файлы             | - [ ]  |
| 16  | Обновить main.rs импорты            | - [ ]  |
| 17  | Реализовать machine::handle()       | - [ ]  |
| 18  | Обновить tasks.rs machine usage     | - [ ]  |
| 19  | Оптимизировать таймер               | - [ ]  |
| 20  | Очистить button_actions.rs          | - [ ]  |

---

## Всего 20 шагов

Каждый шаг оставляет код в рабочем состоянии. После каждого шага выполнять `cargo check` для верификации.
