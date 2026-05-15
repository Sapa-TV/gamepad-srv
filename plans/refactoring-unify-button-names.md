# План рефакторинга: Унификация имен кнопок

## Цель

Привести все имена и названия кнопок к одному варианту через strum::EnumMessage + detailed, сделать ButtonName enum единственным источником истины для имён кнопок.

## Анализ текущего состояния

### ButtonName enum (src/gamepad/button.rs)

Использует кастомные `#[strum(serialize = "X")]` атрибуты для коротких имён:

- Variant name: South, East, West, North, LeftBar, RightBar, LeftTrigger, RightTrigger, LeftStick, RightStick, DPadUp/Down/Left/Right, Start, Select, **Mode**
- Short name (serialize): A, B, Y, X, LB, RB, LT, RT, LS, RS, DU, DD, DL, DR, ST, SE, **MN**

Variant Mode надо переименовать в Menu.

### VALID_BUTTONS (src/skin_manager/discovery.rs)

Дублирует имена кнопок как `&[&str]` массив. Источник несоответствий:

- Содержит `LeftShoulder/RightShoulder` (надо `LeftBar/RightBar`)
- Содержит `Menu` — правильное имя, всё ок
- Содержит `LeftStickPressed/RightStickPressed` (надо `LeftStick/RightStick`)

### skin.json файлы

Используют неправильные имена. Надо привести к variant names из ButtonName:

- `LeftShoulder` → `LeftBar`
- `RightShoulder` → `RightBar`
- `Menu` → `Menu` (уже правильно)
- `LeftStickPressed/RightStickPressed` → `LeftStick/RightStick`

## Шаги

### Шаг 1: Добавить MESSAGE атрибуты к ButtonName

- [x] К каждому variant добавить `#[strum(message = "...", detailed)]`

**Проверка:**

- [x] cargo check
- [x] cargo fmt

---

### Шаг 2: Обновить skin.json и VALID_BUTTONS

Объединены т.к. без этого приложение не будет работать (валидация скинов не пройдёт).

- [ ] Найти все skin.json файлы
- [ ] Заменить LeftShoulder → LeftBar, RightShoulder → RightBar
- [ ] Заменить LeftStickPressed → LeftStick, RightStickPressed → RightStick
- [ ] Удалить константу VALID_BUTTONS
- [ ] Переписать `button_name_valid(name: &str) -> bool` через `ButtonName::VARIANTS.contains(&name)`

**Проверка:**

- [ ] cargo check
- [ ] cargo fmt

---

### Шаг 3: Переименовать Mode → Menu

- [ ] Переименовать variant Mode в Menu в button.rs
- [ ] Переименовать Mode в Menu в converter.rs
- [ ] Найти и исправить все использования

**Проверка:**

- [ ] cargo check
- [ ] cargo fmt

---

### Шаг 4: Удалить неиспользуемые методы

- [ ] Удалить `ButtonName::from_index()`
- [ ] Удалить `ButtonEvent::button_name()`
- [ ] Удалить `ButtonName::to_string()` (заменён на EnumMessage)

**Проверка:**

- [ ] cargo check
- [ ] cargo fmt
- [ ] Нет warnings

## Статус

| Шаг | Описание                           | Выполнен |
| --- | ---------------------------------- | -------- |
| 1   | MESSAGE атрибуты                   | [x]      |
| 2   | Обновить skin.json и VALID_BUTTONS | [x]      |
| 3   | Mode → Menu                        | [x]      |
| 4   | Удалить методы                     | [x]      |

## Ключевые решения

1. **LB/LeftBar** - оставляем LeftBar (enum variant) с serialize = "LB", message = "Left Shoulder"
2. **Menu** - переименовываем Mode в Menu, serialize = "MN", message = "Menu"
3. **LeftStickPressed/RightStickPressed** - добавлены фантомными вариантами для skin.json валидации

## Ожидаемый результат

- `ButtonName::get_detailed_message()` возвращает полное имя кнопки
- `ButtonName::VARIANTS` - массив коротких имён для skin.json валидации
- Нет дублирования имён кнопок
- Единственный источник истины - ButtonName enum
