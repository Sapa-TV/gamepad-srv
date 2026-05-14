# План рефакторинга Gamepad Overlay Server (v2)

Устранение технического долга и улучшение архитектуры после завершения шагов 0-25.

---

## Цель

Упростить кодовую базу, убрать избыточные абстракции, улучшить обработку ошибок.

---

## Шаги

### Шаг 1: Удалить пустой файл

- [x] **Удалить:** src/gamepad/input.rs (1 пустая строка - placeholder, не используется)

**Проверка:**

- [x] cargo check
- [x] cargo fmt

---

### Шаг 2: Упростить паттерн сохранения конфига

**Проблема:** main.rs:64-65

```rust
let save_tx = Arc::new(std::sync::Mutex::new(Some(save_tx)));
```

`Arc<Mutex<Option<mpsc::Sender>>>` — лишняя обёртка над mpsc::Sender.

**Действие:**

- [x] Изменить `tasks.rs::spawn_button_actions`:
  - `save_tx: Arc<std::sync::Mutex<Option<mpsc::Sender<String>>>>` -> `save_tx: tokio::sync::mpsc::Sender<String>`
- [x] Изменить `button_actions.rs`:
  - `save_tx: Arc<Mutex<Option<...>>>` -> `save_tx: mpsc::Sender<String>`
  - Убрать `tx_guard.lock().unwrap()` и `if let Some(ref tx)` — просто `save_tx.send(...).await`
- [x] Изменить `main.rs`:
  - Создать `let (save_tx, save_rx) = tokio::sync::mpsc::channel(32);`
  - Передать `save_tx` напрямую, без Arc/Mutex/Option
  - Изменить receiver на `while let` для множественных сохранений

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 2.1: Исправить баг с one-shot (был шаг 2)

**Проблема:** После шага 2 использовался `oneshot::Sender`, который позволяет отправить только ОДИН раз. Но смена скинов может происходить многократно за время работы приложения.

**Исследование:**

- `oneshot::Sender` закрывает канал после первого `send()`
- Пользователь может нажимать DPad Left/Right много раз в состоянии `SkinSwitchReady`
- Каждое нажатие должно сохранять новый skin в конфиг

**Действие (исправление):**

- [x] Изменить обратно на `mpsc::channel` (был `oneshot::channel`)
- [x] Убрать `Option`/`take()` в `button_actions.rs` — mpsc позволяет множественные send
- [x] Изменить receiver в `main.rs` на `while let` вместо `if let`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 3: Убрать лишний клон gamepad_state

**Проблема:** main.rs:60 и main.rs:67 - два клона одного состояния

```rust
let tick_state = app_state.gamepad_state.clone();   // для spawn_stick_tick
let gilrs_state = app_state.gamepad_state.clone();  // для spawn_all_tasks
```

**Действие:**

- В `main.rs` оставить только `tick_state = app_state.gamepad_state.clone()` для `spawn_stick_tick`
- В `tasks.rs::spawn_all_tasks` клонировать `gilrs_state` внутри из `app_state.gamepad_state.clone()`:

```rust
pub fn spawn_all_tasks(&self, gamepad_state: Arc<Mutex<GamepadState>>, skin_manager: SkinManager, save_tx: mpsc::Sender<String>) {
    let tick_state = gamepad_state.clone();
    spawn_stick_tick(tick_state, self.ws_sender());

    let gilrs_state = gamepad_state.clone();
    spawn_gilrs_task(gilrs_state, ...);
    ...
}
```

- Убрать `spawn_stick_tick` из main.rs — вызывать внутри `spawn_all_tasks`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

**Статус:** Действия выполнены автоматически при шаге 2 (перенос spawn_stick_tick внутрь spawn_all_tasks).

---

### Шаг 4: Убрать двойную загрузку конфига

**Проблема:** main.rs:37 загружает конфиг, потом main.rs:55 загружает снова

```rust
let config = config::load_or_create_config()...    // строка 37
...
let mut cfg = config::load_or_create_config()...   // строка 55 - повтор
```

**Действие:**

- [x] В `main.rs` использовать уже загруженный `config`:

```rust
if let Some(skin) = app_state.skin_manager.get_current() {
    let mut cfg = config.clone();  // используем уже загруженный
    cfg.skin = Some(skin.dir_name.clone());
    let _ = config::save_config(&cfg);
}
```

- Альтернатива (не выбрана): не сохранять при старте — skin всё равно загружен в SkinManager

**Заметка:** Сохранение конфига при старте нужно чтобы записать skin в файл для persistence между перезапусками. Простое клонирование + сохранение дешевле полной загрузки.

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 5: Консолидировать ws_tx в одном месте

**Проблема:** `Channels` имеет `ws_tx`, `AppState` тоже имеет `ws_tx` — источник правды размыт.

**Действие:**

- В `app.rs` убрать `ws_tx` из `AppState`
- `AppState` содержит только `gamepad_state`, `shutting_down`, `skin_manager`
- HTTP handlers получают `ws_tx` через `State(Arc<broadcast::Sender<GamepadEvent>>)` через Channels
- Или: `AppState` владеет `Channels`, HTTP handlers используют `state.channels.ws_sender()`

**Выполнено:**
- [x] В `app.rs` заменено `ws_tx` на `channels: Channels` в `AppState`
- [x] Добавлено `#[derive(Clone)]` к `Channels`
- [x] В `handlers.rs` используется `state.channels.ws_sender().subscribe()`
- [x] В `main.rs` передаётся `channels` (move) при создании `AppState`
- [x] `spawn_all_tasks` вызывается через `app_state.channels.spawn_all_tasks()`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 6: Улучшить обработку ошибок

**Проблема:** main.rs:44-46 и websocket/handler.rs - unwrap может вызвать panic

**Действие 6.1:** main.rs:42-46

```rust
let addr: SocketAddr = format!("0.0.0.0:{}", config.port)
    .to_socket_addrs()
    .unwrap()   // может panic
    .next()
    .unwrap();
```

Заменить на:

```rust
let addr: SocketAddr = format!("0.0.0.0:{}", config.port)
    .to_socket_addrs()
    .map_err(|e| anyhow::anyhow!("Failed to parse address: {}", e))?
    .next()
    .ok_or_else(|| anyhow::anyhow!("No addresses found"))?;
```

**Действие 6.2:** websocket/handler.rs:23,34

```rust
let _ = socket.send(to_string(&output).unwrap().into()).await;
```

Заменить на обработку ошибки:

```rust
match to_string(&output) {
    Ok(json) => {
        if socket.send(json.into()).await.is_err() {
            break;
        }
    }
    Err(e) => {
        tracing::error!("Failed to serialize: {}", e);
        break;
    }
}
```

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 7: Абстрагировать ButtonEvent от gilrs

**Проблема:** `skin_switch/machine.rs` напрямую паттерн-матчит на `gilrs::EventType::ButtonPressed(btn, _)`. Machine знает про внутренности gilrs — это инфраструктурная деталь.

**Действие:**

- Создать `src/skin_switch/buttons.rs`:

```rust
#[derive(Debug, Clone)]
pub enum ButtonName {
    DPadRight,
    DPadLeft,
    Start,
    Select,
}

#[derive(Debug, Clone)]
pub enum ButtonEvent {
    Pressed(ButtonName),
    Released(ButtonName),
}
```

- В `machine.rs`:
  - `handle(&mut self, event: &AppEvent)` -> `handle_button(&mut self, event: ButtonEvent)`
  - Machine принимает `ButtonEvent` вместо `AppEvent::Gilrs`

- В `tasks.rs::spawn_skin_change_tracker`:
  - Конвертировать `AppEvent::Gilrs` в `ButtonEvent` перед вызовом machine

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 8: Выровнять владение в handle()

**Проблема:** machine.rs:20

```rust
pub fn handle(&mut self, event: &AppEvent) -> Option<Command>
```

Принимает `&AppEvent`, но внутри мутирует `self.state`. Непоследовательно.

**Действие:**

- Изменить на `pub fn handle(mut self, event: AppEvent) -> Option<Command>`
- Или: `pub fn handle(&mut self, event: AppEvent)` — event тоже мутируемый (но это хуже)

- После шага 7: `pub fn handle(mut self, event: ButtonEvent) -> Option<Command>`

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 9: Убрать неиспользуемый импорт

**Действие:**

- `main.rs`: убрать `use std::net::ToSocketAddrs;` — не используется напрямую (`format!().to_socket_addrs()` скрывает это)

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 10: Восстановить gamepad/input.rs (перенести spawn_gilrs_task)

**Проблема:**

- В шаге 1 был удалён пустой `input.rs`, но по плану он должен содержать "input driver" — цикл опроса геймпада
- `spawn_gilrs_task` currently lives in `tasks.rs`, which is orchestration, not input driver
- Это нарушает архитектуру: `gamepad/` должен содержать input driver, `tasks.rs` — только orchestration

**Исследование:**

- `spawn_gilrs_task` (tasks.rs:40-68) инициализирует `Gilrs`, polling loop, отправляет `GamepadEvent` и `AppEvent::Gilrs`
- Использует `gilrs::Gilrs`, `process_event`, `GamepadState`
- Логически принадлежит `gamepad/` как "input driver"

**Действие:**

- [ ] Создать `src/gamepad/input.rs`:
  - Перенести `spawn_gilrs_task` из `tasks.rs`
  - Добавить `pub fn spawn_input_task(...)` — обёртка для удобства
- [ ] Обновить `gamepad/mod.rs` → `pub mod input;`
- [ ] Обновить `tasks.rs`:
  - Убрать `spawn_gilrs_task` определение
  - Добавить `use crate::gamepad::input::spawn_gilrs_task;`

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 11: Исследовать зачем нужен Clone на Channels и копирование State в Axum

**Проблема/Вопрос:**
- `#[derive(Clone)]` добавлен к `Channels` чтобы `AppState` мог быть `Clone`
- Axum требует `Clone` для `.with_state()`
- Но в текущей реализации `channels` передаётся через move, не клонируется
- Возможно, это архитектурный костыль

**Исследование:**
- Зачем axum копирует State? Это для каждого запроса new state или shared?
- Можно ли избежать Clone на Channels?
- Какие есть альтернативы (Arc<Channels>, не Clone State, etc.)

**Действие:**
- [ ] Исследовать как axum использует State
- [ ] Определить оптимальный дизайн

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

## Статус

| Шаг | Описание                            | Выполнен |
| --- | ----------------------------------- | -------- |
| 1   | Удалить пустой файл input.rs        | [x]      |
| 2   | Упростить паттерн сохранения (mpsc) | [x]      |
| 2.1 | Исправить баг с one-shot sender     | [x]      |
| 3   | Убрать лишний клон gamepad_state    | [x]      |
| 4   | Убрать двойную загрузку конфига     | [x]      |
| 5   | Консолидировать ws_tx               | [x]      |
| 6   | Улучшить обработку ошибок (unwrap)  | [ ]      |
| 7   | Абстрагировать ButtonEvent от gilrs | [ ]      |
| 8   | Выровнять владение в handle()       | [ ]      |
| 9   | Убрать неиспользуемый импорт        | [ ]      |
| 10  | Восстановить gamepad/input.rs       | [ ]      |
| 11  | Исследовать Clone на Channels       | [ ]      |

---

## Всего 11 шагов

Каждый шаг - законченная единица работы, после которой код компилируется и работает.
После каждого шага выполнять `cargo check` и `cargo fmt` для верификации.

Шаг 2.1 добавлен как исправление бага, обнаруженного при выполнении шага 2.
Шаг 10 добавлен для восстановления чистой архитектуры: `gamepad/input.rs` должен содержать input driver (gilrs loop).
Шаг 11 добавлен для исследования архитектурного вопроса: зачем нужен Clone на Channels и как axum использует State.
