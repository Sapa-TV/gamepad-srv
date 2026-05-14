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

`Arc<Mutex<Option<mpsc::Sender>>>` — `Option` используется только для `take()` один раз.

**Действие:**

- Изменить `tasks.rs::spawn_button_actions`:
  - `save_tx: Arc<std::sync::Mutex<Option<mpsc::Sender<String>>>>` -> `save_tx: tokio::sync::oneshot::Sender<String>`
- Изменить `button_actions.rs`:
  - `save_tx: Arc<Mutex<Option<...>>>` -> `save_tx: oneshot::Sender<String>`
  - Убрать `tx_guard.lock().unwrap()` и `if let Some(ref tx)` — просто `save_tx.send(skin.dir_name)`
- Изменить `main.rs`:
  - Создать `let (save_tx, save_rx) = tokio::sync::oneshot::channel();`
  - Передать `save_tx` напрямую, без Arc/Mutex/Option

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

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
pub fn spawn_all_tasks(&self, gamepad_state: Arc<Mutex<GamepadState>>, skin_manager: SkinManager, save_tx: oneshot::Sender<String>) {
    let tick_state = gamepad_state.clone();
    spawn_stick_tick(tick_state, self.ws_sender());

    let gilrs_state = gamepad_state.clone();
    spawn_gilrs_task(gilrs_state, ...);
    ...
}
```

- Убрать `spawn_stick_tick` из main.rs — вызывать внутри `spawn_all_tasks`

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 4: Убрать двойную загрузку конфига

**Проблема:** main.rs:37 загружает конфиг, потом main.rs:55 загружает снова

```rust
let config = config::load_or_create_config()...    // строка 37
...
let mut cfg = config::load_or_create_config()...   // строка 55 - повтор
```

**Действие:**

- В `main.rs` использовать уже загруженный `config`:

```rust
if let Some(skin) = app_state.skin_manager.get_current() {
    let mut cfg = config.clone();  // используем уже загруженный
    cfg.skin = Some(skin.dir_name.clone());
    let _ = config::save_config(&cfg);
}
```

- Или: не сохранять при старте — skin всё равно загружен в SkinManager

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 5: Консолидировать ws_tx в одном месте

**Проблема:** `Channels` имеет `ws_tx`, `AppState` тоже имеет `ws_tx` — источник правды размыт.

**Действие:**

- В `app.rs` убрать `ws_tx` из `AppState`
- `AppState` содержит только `gamepad_state`, `shutting_down`, `skin_manager`
- HTTP handlers получают `ws_tx` через `State(Arc<broadcast::Sender<GamepadEvent>>)` через Channels
- Или: `AppState` владеет `Channels`, HTTP handlers используют `state.channels.ws_sender()`

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

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

## Статус

| Шаг | Описание                               | Выполнен |
| --- | -------------------------------------- | -------- |
| 1   | Удалить пустой файл input.rs           | [x]      |
| 2   | Упростить паттерн сохранения (oneshot) | [ ]      |
| 3   | Убрать лишний клон gamepad_state       | [ ]      |
| 4   | Убрать двойную загрузку конфига        | [ ]      |
| 5   | Консолидировать ws_tx                  | [ ]      |
| 6   | Улучшить обработку ошибок (unwrap)     | [ ]      |
| 7   | Абстрагировать ButtonEvent от gilrs    | [ ]      |
| 8   | Выровнять владение в handle()          | [ ]      |
| 9   | Убрать неиспользуемый импорт           | [ ]      |

---

## Всего 9 шагов

Каждый шаг - законченная единица работы, после которой код компилируется и работает.
После каждого шага выполнять `cargo check` и `cargo fmt` для верификации.
