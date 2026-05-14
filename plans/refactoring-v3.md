# План рефакторинга skin_switch/machine.rs

---

## Цель

Упростить machine.rs: убрать дублирование, вынести логику обработки кнопок в `SkinChangeState`.

---

## Шаги

### Шаг 1: Добавить `From<ButtonName> for Direction` в state.rs

- [ ] Добавить `use crate::skin_switch::buttons::ButtonName;`
- [ ] Добавить impl From для Direction

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 2: Добавить `handle_pressed` в SkinChangeState

- [ ] Добавить `use crate::skin_switch::commands::Command;`
- [ ] Добавить `use crate::skin_switch::buttons::ButtonName;`
- [ ] Добавить функцию `handle_pressed`

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 3: Добавить `handle_released` в SkinChangeState

- [ ] Добавить функцию `handle_released`

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 4: Упростить `handle_button` в machine.rs

- [ ] Обновить `handle_button` — вызвать `state.handle_pressed/handle_released`

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 5: Добавить `check_timeout` и `deadline` в SkinChangeState

- [ ] Добавить `use std::time::Duration;`
- [ ] Добавить функцию `check_timeout`
- [ ] Добавить функцию `deadline`

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 6: Упростить machine.rs — убрать лишние методы

- [ ] Удалить `check_timeout` и `deadline` из machine.rs
- [ ] Удалить неиспользуемые импорты

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 7: Обновить tasks.rs

- [ ] Заменить `machine.check_timeout()` на `machine.state.check_timeout()`
- [ ] Заменить `machine.deadline()` на `machine.state.deadline()`

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

### Шаг 8: Финальная проверка

- [ ] Проверить что все импорты используются

**Проверка:**

- [ ] `cargo check`
- [ ] `cargo fmt`

---

## Статус

| Шаг | Описание                          | Выполнен |
| --- | --------------------------------- | -------- |
| 1   | From<ButtonName> for Direction    | [ ]      |
| 2   | handle_pressed в SkinChangeState  | [ ]      |
| 3   | handle_released в SkinChangeState | [ ]      |
| 4   | Упростить handle_button           | [ ]      |
| 5   | check_timeout и deadline в state  | [ ]      |
| 6   | Упростить machine.rs              | [ ]      |
| 7   | Обновить tasks.rs                 | [ ]      |
| 8   | Финальная проверка                | [ ]      |

---

## Всего 8 шагов

Каждый шаг — компилируемый код. После каждого шага `cargo check` и `cargo fmt`.