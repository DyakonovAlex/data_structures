## Data Structures

Реализация базовых структур данных на Rust.

### Модули

- **`arrays`**
  - **Трейт**: `Array<T>`
  - **Реализация**: `UnsortedArray<T>` — неотсортированный массив фиксированной ёмкости на основе `Box<[Option<T>]>`.

### Интерфейс `Array<T>`

Трейт задаёт абстрактный интерфейс для массивов:

- **`insert(&mut self, item: T) -> Result<(), ArrayError>`**: вставка элемента (ошибка при переполнении).
- **`delete(&mut self, index: usize) -> Result<T, ArrayError>`**: удаление по индексу с возвратом удалённого значения.
- **`find(&self, item: &T) -> Option<usize>`**: поиск индекса по значению.
- **`get(&self, index: usize) -> Option<&T>`**: доступ к элементу по индексу.
- **`size(&self) -> usize`**: текущее количество элементов.
- **`capacity(&self) -> usize`**: максимальная ёмкость.
- **`is_empty(&self) -> bool`**, **`is_full(&self) -> bool`**: проверки состояния.
- **`traverse(&self, f: impl FnMut(&T) -> Result<(), ArrayError>) -> Result<(), ArrayError>`**: обход всех элементов с пользовательской функцией.

Ошибки описываются через `ArrayError`:

- `Full`, `Empty`, `IndexOutOfBounds`.

### Асимптотика (Big O) для `UnsortedArray`

Для реализации `UnsortedArray<T>`:

- **`insert`**: \(O(1)\) — амортизированное, вставка в конец без сдвигов.
- **`delete`**: \(O(1)\) — удаление с заменой на последний элемент (порядок не сохраняется).
- **`find`**: \(O(n)\) — линейный поиск по значениям.
- **`get`**: \(O(1)\) — прямой доступ по индексу.
- **`size`** / **`capacity`** / **`is_empty`** / **`is_full`**: \(O(1)\).
- **`traverse`**: \(O(n)\) — один проход по всем текущим элементам.

### Примеры

В проекте есть пример использования `UnsortedArray`:

```bash
cargo run --example unsorted_array
```

Пример показывает:

- создание массива,
- операции `insert`, `delete`, `find`, `get`,
- проверку `is_full`/`is_empty`,
- использование `traverse` для обхода и агрегации.

### Тесты

Запуск тестов:

```bash
cargo test
```