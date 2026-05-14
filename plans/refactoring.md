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

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 1: Перенести `gamepad/state.rs`

- [x] **Создать:** `src/gamepad/state.rs` — скопировать содержимое `gamepad_state.rs`

- [x] **Создать:** `src/gamepad/input.rs` — пока пустой

- [x] **Обновить:** `gamepad_state.rs` → оставить только `pub use crate::gamepad::state::*;` (re-export)

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 2: Перенести `skin_manager/discovery.rs`

- [x] **Создать:** `src/skin_manager/discovery.rs` — скопировать `skin.rs`

- [x] **Обновить:** `skin.rs` → `pub use crate::skin_manager::discovery::*;`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 3: Перенести `skin_manager/manager.rs`

- [x] **Создать:** `src/skin_manager/manager.rs`

```rust
use super::discovery::SkinEntry;

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

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 4: Перенести `skin_switch/state.rs`

- [x] **Создать:** `src/skin_switch/state.rs` — скопировать `skin_change_state.rs`

- [x] **Обновить:** `skin_change_state.rs` → `pub use crate::skin_switch::state::*;`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 5: Перенести `websocket/handler.rs`

- [x] **Создать:** `src/websocket/handler.rs` — скопировать `ws.rs`

- [x] **Обновить:** `ws.rs` → `pub use crate::websocket::handler::handle_socket;`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 6: Перенести `config/mod.rs`

- [x] **Создать:** `src/config/mod.rs` — содержимое `config.rs`

- [x] **Удалить:** `config.rs` (нельзя иметь одновременно `config.rs` и `config/mod.rs`)

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 7: Создать `commands.rs` в skin_switch

- [x] **Создать:** `src/skin_switch/commands.rs`

```rust
use crate::skin_switch::state::Direction;

#[derive(Debug)]
pub enum Command {
    SkinChange(Direction),
    NotifySkinChanging(bool),
    SkinSwitchReady,
}
```

- [x] **Обновить:** `skin_switch/mod.rs` → `pub mod state; pub mod commands;`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 8: Создать `skin_switch/machine.rs` (логика переходов)

- [x] **Создать:** `src/skin_switch/machine.rs` — пока простой skeleton:

```rust
use crate::events::AppEvent;
use crate::skin_switch::commands::Command;

pub struct SkinSwitchMachine;

impl SkinSwitchMachine {
    pub fn handle(&mut self, event: &AppEvent) -> Option<Command> { None }
}
```

- [x] **Обновить:** `skin_switch/mod.rs` → `pub mod machine;`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 9: Перенести `event_processor.rs`

- [x] **Создать:** `src/gamepad/event_processor.rs` — скопировать содержимое

- [x] **Обновить:** `event_processor.rs` → `pub use crate::gamepad::event_processor::*;`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 10: Обновить `handlers.rs`

- [x] **Обновить импорты в `handlers.rs`:**
  - `use crate::ws::handle_socket` → `use crate::websocket::handler::handle_socket`

- [x] **Убрать:** `ws.rs` ре-экспорт (теперь handlers.rs будет использовать напрямую)

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 11: Обновить `app.rs`

- [x] **Обновить импорты в `app.rs`:**
  - Убрать `use crate::skin::...` → `use crate::skin_manager::discovery::...`
  - Убрать `use crate::gamepad_state::...` → `use crate::gamepad::state::...`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 12: Обновить `tasks.rs`

- [x] **Обновить импорты в `tasks.rs`:**
  - `use crate::gamepad_state::...` → `use crate::gamepad::state::...`
  - `use crate::skin_change_state::...` → `use crate::skin_switch::state::...`
  - `use crate::event_processor::...` → `use crate::gamepad::event_processor::...`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 13: Обновить `button_actions.rs`

- [x] **Обновить импорты:**
  - `use crate::skin::...` → `use crate::skin_manager::discovery::...`
  - `use crate::skin_change_state::...` → `use crate::skin_switch::state::...`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 14: Обновить `main.rs`

- [x] **Проверить что импорты работают** — код компилируется, менять пока не нужно

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 15: Удалить re-export файлы

- [x] **Удалить:**
  - `src/gamepad_state.rs`
  - `src/skin.rs`
  - `src/skin_change_state.rs`
  - `src/ws.rs`
  - `src/event_processor.rs`

- [x] **Обновить:** `main.rs` — убрать `mod gamepad_state;`, `mod skin;`, etc.

- [x] **Обновить:** файлы которые использовали старые пути (`events.rs`, `handlers.rs`, `tasks.rs`)

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 16: Обновить `main.rs` — убрать лишние импорты

- [x] В шаге 15 уже обновлены `mod` декларации в main.rs:
  - Удалены: `mod event_processor; mod gamepad_state; mod skin; mod skin_change_state; mod ws;`
  - Сохранены: `mod app; mod button_actions; mod config; mod events; mod gamepad; mod handlers; mod skin_manager; mod skin_switch; mod tasks; mod websocket;`
- [x] Файлы `events.rs`, `handlers.rs`, `tasks.rs` обновлены для использования новых путей импортов

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 17: Реализовать `SkinSwitchMachine::handle()`

- [x] Перенести логику из `tasks.rs::spawn_skin_change_tracker` в `skin_switch/machine.rs`:
  - Структура `SkinSwitchMachine` владеет своим состоянием `SkinChangeState`
  - Метод `handle(&mut self, event: &AppEvent) -> Option<Command>` — обработка событий кнопок и переходы состояний
  - Метод `check_timeout(&mut self) -> Option<Command>` — проверка таймаута SkinSwitchPending → SkinSwitchReady
  - Метод `state(&self) -> &SkinChangeState` — доступ к состоянию

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 18: Обновить `tasks.rs` — использовать новый machine

- [x] Переписан `spawn_skin_change_tracker`:
  - Теперь создаёт `SkinSwitchMachine::new()` вместо `Arc<Mutex<SkinChangeState>>`
  - В цикле вызывает `machine.handle(&event)` вместо direct state manipulation
  - Обрабатывает возвращённые `Command`ы и отправляет соответствующие события
- [x] Убран параметр `button_state` из `spawn_all_tasks` — больше не нужен
- [x] Убрана логика таймера из machine в tasks (пока оставлен sleep(100) polling)

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 19: Оптимизировать таймер

- [x] Добавлен метод `deadline()` в `SkinSwitchMachine` — возвращает `Option<tokio::time::Instant>` для следующего таймаута
- [x] Переписан `spawn_skin_change_tracker`:
  - Использует `tokio::time::sleep_until(timeout)` вместо polling `sleep(100)`
  - При переходе в `SkinSwitchPending` вычисляется конкретный deadline
  - Таймер активируется только когда есть ожидающий переход

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

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
| 2   | Перенести skin_manager/discovery.rs | [x]    |
| 3   | Перенести skin_manager/manager.rs   | [x]    |
| 4   | Перенести skin_switch/state.rs      | [x]    |
| 5   | Перенести websocket/handler.rs      | [x]    |
| 6   | Перенести config/mod.rs             | [x]    |
| 7   | Создать commands.rs                 | [x]    |
| 8   | Создать machine.rs                  | [x]    |
| 9   | Перенести event_processor.rs        | [x]    |
| 10  | Обновить handlers.rs                | [x]    |
| 11  | Обновить app.rs                     | [x]    |
| 12  | Обновить tasks.rs                   | [x]    |
| 13  | Обновить button_actions.rs          | [x]    |
| 14  | Обновить main.rs                    | [x]    |
| 15  | Удалить re-export файлы             | [x]    |
| 16  | Обновить main.rs импорты            | [x]    |
| 17  | Реализовать machine::handle()       | [x]    |
| 18  | Обновить tasks.rs machine usage     | [x]    |
| 19  | Оптимизировать таймер               | [x]    |
| 20  | Очистить button_actions.rs          | [ ]    |

---

## Всего 20 шагов

Каждый шаг оставляет код в рабочем состоянии. После каждого шага выполнять `cargo check` и `cargo fmt` для верификации.
