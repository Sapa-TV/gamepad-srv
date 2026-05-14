# План рефакторинга skin_switch/machine.rs

---

## Цель

Упростить machine.rs: убрать дублирование, добавить простые методы в SkinChangeState для изменения состояния (без логики).

---

## Шаги

### Шаг 1: Добавить `From<ButtonName> for Direction` в state.rs

- [x] Добавить `use crate::skin_switch::buttons::ButtonName;`
- [x] Добавить impl From для Direction

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 2: Добавить простые методы в SkinChangeState

**Действие:**

- [x] Добавить `use crate::skin_switch::buttons::ButtonName;`
- [x] Добавить методы:
  - `press_start()`, `release_start()`
  - `press_select()`, `release_select()`
  - `set_pending()` — `state = SkinSwitchPending; pending_since = Some(now)`
  - `clear_pending()` — `pending_since = None`
  - `set_normal()` — `state = Normal; clear_pending()`
  - `set_skin_switch_ready()` — `state = SkinSwitchReady; clear_pending()`
  - `set_skin_switch()` — `state = SkinSwitch`

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 3: Упростить machine.rs — использовать методы state

**Действие:**

- [x] DPad: использовать `button.into()` для Direction
- [x] Start/Select pressed: использовать `press_start()`, `press_select()`, `set_pending()`
- [x] Start/Select released: использовать `release_start()`, `release_select()`, `set_normal()`, `set_skin_switch()`
- [x] Убрать дублирование

**Проверка:**

- [x] `cargo check`
- [x] `cargo fmt`

---

### Шаг 4: Финальная проверка

**Действие:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

## Статус

| Шаг | Описание                           | Выполнен |
| --- | ---------------------------------- | -------- |
| 1   | From<ButtonName> for Direction     | [x]      |
| 2   | Простые методы в SkinChangeState   | [x]      |
| 3   | Упростить machine.rs               | [x]      |
| 4   | Финальная проверка                 | [ ]      |

---

## Всего 4 шага