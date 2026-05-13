# План рефакторинга Gamepad Overlay Server

Этот документ описывает стратегию улучшения архитектуры проекта для повышения поддерживаемости, тестируемости и разделения ответственности.

## 1. Проблемы текущей реализации

- **Сильная связанность (Tight Coupling):** Состояние приложения (`AppState`, `Channels`) прокидывается через `Arc<Mutex<T>>` во все задачи [cite: 1, 8].
- **Размазанная логика (Logic Leakage):** Логика переключения скинов распределена между `tasks.rs` (отслеживание нажатий) и `button_actions.rs` (изменение индекса) [cite: 2, 10].
- **Сложность тестирования:** Почти невозможно протестировать логику смены состояний без запуска `gilrs` или `tokio` [cite: 10].
- **Активное ожидание:** В `tasks.rs` используется `sleep` внутри цикла для проверки таймеров, что менее эффективно, чем событийная модель [cite: 10].

## 2. Целевая архитектура (Actor-like / Command Bus)

Переход к модели, где компоненты общаются через сообщения, а не общее состояние.

### А. Слой доменной логики (Domain)

Создать чистые структуры данных без зависимостей от `tokio` или `axum`:

- **`SkinManager`**: Отвечает за список скинов и навигацию по ним (next/prev).
- **`InputStateMachine`**: Конечный автомат, принимающий события кнопок и возвращающий команды (например, `ChangeSkin`, `NotifyUI`).

### Б. Слой драйверов (Infrastructure)

- **Input Driver**: Только чтение из `gilrs` и отправка в канал `events_tx`.
- **Output Driver (WebSocket)**: Слушает канал уведомлений и транслирует их клиентам.

## 3. Этапы реализации

### Этап 1: Инкапсуляция состояний

1.  **Создать `src/skin/manager.rs`**: Перенести туда `Vec<SkinEntry>` и `current_index` [cite: 6]. Добавить методы `get_current()`, `next()`, `prev()`.
2.  **Создать `src/state_machine.rs`**: Перенести логику из `spawn_skin_change_tracker` в метод `handle_input(&mut self, event: AppEvent) -> Vec<Command>` [cite: 10].

### Этап 2: Рефакторинг обмена сообщениями

1.  Ввести перечисление `Command`:
    ```rust
    enum Command {
        UpdateVisuals(GamepadEvent),
        ReloadSkin(SkinInfo),
    }
    ```
2.  Убрать прямой доступ к `ws_tx` из логических задач. Только центральный процессор событий должен решать, что отправлять в сокет.

### Этап 3: Очистка `tasks.rs` и `main.rs`

1.  Разбить `spawn_all_tasks` на логические модули.
2.  В `main.rs` инициализировать каналы и передавать только необходимые `Sender/Receiver` в каждую задачу вместо всего `AppState` [cite: 7, 8].

### Этап 4: Оптимизация таймеров

1.  Заменить `time::sleep(100)` в трекере на `tokio::time::sleep_until` при переходе в состояние `SkinSwitchPending`, чтобы избавиться от лишних итераций цикла [cite: 10].

## 4. Ожидаемый результат

- **Чистый `main.rs`**: Только конфигурация и запуск задач.
- **Тестируемость**: Возможность написать модульные тесты для `SkinManager` и конечного автомата.
- **Надежность**: Отсутствие возможности взаимной блокировки (deadlock) из-за уменьшения количества и области действия `Mutex`.

---

Чтобы сделать проект поддерживаемым, нужно применить **Actor-like подход** или **Clean Architecture**, разделив состояние, логику переходов и ввод-вывод.

Вот основные направления рефакторинга:

---

## 1. Инкапсуляция логики переключения скинов (State Machine)

Сейчас логика `SkinChangeState` размазана между `tasks.rs` (где проверяются таймеры и нажатия) и `button_actions.rs` (где меняется индекс).

**Что сделать:** Создайте полноценный конечный автомат.

- Перенесите логику переходов (`Normal -> Pending -> Ready`) в методы `SkinChangeState`.
- Вместо того чтобы `tasks.rs` напрямую менял поля, он должен вызывать `state.handle_event(event)`.

```rust
// В skin_change_state.rs
impl SkinChangeState {
    pub fn handle_event(&mut self, event: &AppEvent) -> Option<SkinChangeAction> {
        // Вся логика таймеров и комбинаций кнопок здесь.
        // Возвращает Action (например, ChangeSkin(Direction)), который выполнит вызывающий.
    }
}

```

## 2. Уход от повсеместного `Arc<Mutex<AppState>>`

В `main.rs` и `tasks.rs` вы передаете кусочки состояния вручную. Это приводит к ошибкам владения.

**Что сделать:** Используйте **Message Passing** (передачу сообщений) вместо общего состояния там, где это возможно.

- Пусть один таск (владелец `GamepadState`) слушает события и обновляет данные.
- Другие части системы получают обновления через `broadcast` канал.

## 3. Рефакторинг `tasks.rs` (God Module)

Файл `tasks.rs` делает слишком много: и логику кнопок знает, и таймеры считает, и `gilrs` инициализирует.

**Предлагаемая структура:**

1. **Input Driver (`gilrs_loop`):** Только читает сырые события и шлет их в канал `events_tx`.
2. **Logic Processor:** Слушает `events_tx`, обновляет внутреннее состояние (скин, кнопки) и отправляет результат в `ws_tx`.
3. **Tick Loop:** Отдельный таймер только для стиков.

## 4. Очистка `Channels` и `AppState`

`Channels` содержит `Arc<broadcast::Sender>`, а потом `AppState` снова содержит их. Это дублирование.

**Что сделать:** \* Используйте паттерн **Registry** или просто передавайте `Sender` в конструкторы.

- Разделите "Статичное состояние" (список скинов) и "Динамическое состояние" (нажатые кнопки). Списки скинов не нужно оборачивать в `Mutex`, если они не меняются во время работы.

---

## Конкретный план действий:

### Шаг 1: Создание Command-шины

Вместо того чтобы `spawn_skin_change_tracker` сам отправлял события в WS, пусть он генерирует внутренние команды:

```rust
enum InternalCommand {
    NotifyWs(GamepadEvent),
    SwitchSkin(Direction),
}

```

### Шаг 2: Улучшение работы со скинами

В `button_actions.rs` логика смены индекса скина (`rem_euclid`) должна быть скрыта внутри структуры `SkinManager`.

```rust
pub struct SkinManager {
    skins: Vec<SkinEntry>,
    current_idx: usize,
}

impl SkinManager {
    pub fn next(&mut self) -> &SkinEntry { ... }
    pub fn prev(&mut self) -> &SkinEntry { ... }
}

```

### Шаг 3: Разделение Domain и Infrastructure

- **Domain:** `GamepadState`, `SkinChangeState`, `SkinManager`. (Без Tokio, без каналов, чистая логика).
- **Infrastructure:** `ws.rs`, `handlers.rs`, `gilrs_input.rs`. (Только клей между библиотеками и Domain-логикой).

### Шаг 4: Использование `select!` более эффективно

В `spawn_skin_change_tracker` используется `sleep(100)` внутри цикла. Это "активное ожидание".
Лучше использовать `tokio::time::sleep_until` или `Interval`, чтобы просыпаться ровно тогда, когда истечет таймаут `pending_since`.

---

## Пример того, как мог бы выглядеть `main.rs` после рефакторинга:

```rust
#[tokio::main]
async fn main() {
    let (event_tx, _) = broadcast::channel::<AppEvent>(1024);
    let (ws_tx, _) = broadcast::channel::<GamepadEvent>(1024);

    // Инициализация компонентов
    let skin_manager = SkinManager::new(discover_skins());
    let gamepad_manager = GamepadManager::new(ws_tx.clone());

    // Запуск акторов (тасков)
    tokio::spawn(input::gilrs_loop(event_tx.clone()));
    tokio::spawn(logic::main_loop(event_tx.subscribe(), skin_manager, gamepad_manager));
    tokio::spawn(network::tick_loop(ws_tx.clone(), gamepad_manager.state_ref()));

    // Axum...
}

```

**Результат:**

1.  **Тестируемость:** Вы сможете написать тесты для `SkinChangeState` без запуска веб-сервера и геймпада.
2.  **Читаемость:** Логика "если нажаты Start+Select" будет в одном месте, а не размазана по трем файлам.
3.  **Производительность:** Меньше блокировок `Mutex`, так как данные передаются через каналы.
