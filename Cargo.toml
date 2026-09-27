# Move vs Copy в Rust

## Краткий ответ

| | **Move** | **Copy** |
|---|---|---|
| **На уровне ассемблера** | `memcpy` стекового представления | `memcpy` стекового представления |
| **Исходный binding** | **Инвалидирован** | **Остаётся валидным** |
| **Ошибка при использовании** | `E0382: use of moved value` | — |
| **Типы** | `String`, `Vec`, `Box`, `File` | `i32`, `f64`, `bool`, `char`, `&T` |
| **`Drop`** | ✅ Может быть | ❌ **Несовместим** |

**Ключевое:** на уровне **железа** move и copy делают **одно и то же** — `memcpy`. Разница — в том, что **компилятор** делает с **исходным binding**.

## Move

### Что происходит

```rust
let s1 = String::from("hello");
let s2 = s1;   // ← move
```

1. **Компилятор** копирует **стековое** представление `String` (ptr + len + cap) в `s2`.
2. **Куча** **не копируется** — оба указывают на **те же** данные.
3. **`s1`** — **инвалидирован**.
4. **Любое** использование `s1` → **ошибка** `E0382`.

### Ошибка

```rust
println!("{}", s1);   // ❌
```

```
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:3:20
  |
2 |     let s2 = s1;
  |              -- value moved here
3 |     println!("{}", s1);
  |                    ^^ value borrowed here after move
```

### Схема

```
До move:
s1: [ptr]───┐
    [len]   │
    [cap]   │
            ↓
        [heap: "hello"]

После move:
s1: (инвалидирован)
s2: [ptr]───┐
    [len]   │
    [cap]   │
            ↓
        [heap: "hello"]   ← те же данные
```

## Copy

### Что происходит

```rust
let p = Point { x: 10.0, y: 20.0 };
let q = p;   // ← copy
let r = p;   // ← copy
```

1. **Компилятор** копирует **все** байты `Point` (8 байт) в `q` и `r`.
2. **`p`** — **остаётся валидным**.
3. **Все три** — **независимые** значения.

### Схема

```
p: [x=10.0][y=20.0]
q: [x=10.0][y=20.0]   ← копия
r: [x=10.0][y=20.0]   ← копия
```

### Проверка

```rust
println!("{}, {}, {}", p.x, q.x, r.x);   // 10, 10, 10
```

**`p`** доступен **после** `let q = p`.

## Разбор вашего примера

```rust
#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let p = Point { x: 10.0, y: 20.0 };
    let q = p;   // ← copy
    let r = p;   // ← copy

    println!("{}, {}, {}", p.x, q.x, r.x);   // 10, 10, 10
}
```

- **`Point`** реализует `Copy` → **копируется**.
- **`p`**, **`q`**, **`r`** — **три** независимых значения.
- **Все** доступны.

### Что было бы **без** `Copy`

```rust
#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let p = Point { x: 10.0, y: 20.0 };
    let q = p;   // ← move
    let r = p;   // ❌ ошибка: use of moved value

    println!("{}, {}, {}", p.x, q.x, r.x);
}
```

**Ошибка:**

```
error[E0382]: use of moved value: `p`
```

## Какие типы реализуют `Copy`

| Категория | Примеры |
|---|---|
| **Целые** | `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, ..., `usize` |
| **Плавающие** | `f32`, `f64` |
| **Логический** | `bool` |
| **Символьный** | `char` |
| **Разделяемые ссылки** | `&T` |
| **Неизменяемые сырые указатели** | `*const T` |
| **Кортежи** из `Copy` | `(i32, f64)` |
| **Массивы** из `Copy` | `[i32; 5]` |
| **`Option<T>`** | Если `T: Copy` |
| **`Result<T, E>`** | Если `T: Copy`, `E: Copy` |

### Пример

```rust
fn is_copy<T: Copy>() {}

is_copy::<i32>();              // ✅
is_copy::<f64>();              // ✅
is_copy::<bool>();             // ✅
is_copy::<char>();             // ✅
is_copy::<&str>();             // ✅
is_copy::<*const i32>();       // ✅
is_copy::<(i32, f64)>();       // ✅
is_copy::<[i32; 5]>();         // ✅
is_copy::<Option<i32>>();      // ✅
```

## Какие типы **не** реализуют `Copy`

| Тип | Почему |
|---|---|
| **`String`** | Владеет кучей |
| **`Vec<T>`** | Владеет кучей |
| **`Box<T>`** | Владеет кучей |
| **`File`** | Владеет ресурсом ОС |
| **`&mut T`** | Изменяемая ссылка |
| **`*mut T`** | Изменяемый сырой указатель |
| **`Rc<T>`, `Arc<T>`** | Счётчик ссылок |

### Почему `&mut T` — **не** `Copy`

```rust
let mut x = 42;
let r1: &mut i32 = &mut x;
let r2 = r1;   // ← move, не copy

// println!("{}", r1);   // ❌ r1 инвалидирован
```

**Причина:** **две** `&mut T` на **одну** память → **нарушение** borrow checker. `&mut T` **должна** быть **эксклюзивной**.

### Почему `*mut T` — **не** `Copy`

То же самое — `*mut T` **даёт** изменяемый доступ.

## `Copy` и `Drop` — **взаимоисключающие**

```rust
#[derive(Copy, Clone)]
struct MyType {
    data: String,   // ← String не Copy
}
```

**Ошибка:**

```
error[E0204]: the trait `Copy` cannot be implemented for this type
  |
1 | #[derive(Copy)]
  |          ^^^^
  |
  = note: the type `String` does not implement `Copy`
```

Или если у типа есть `Drop`:

```rust
struct MyType;

impl Drop for MyType {
    fn drop(&mut self) { }
}

#[derive(Copy)]
struct MyCopy(MyType);   // ❌
```

```
error[E0184]: the trait `Copy` cannot be implemented for this type;
              the type has a destructor
```

**Причина:** `Copy` **неявно** **дублирует** значение → **два** `drop` на **одну** память → **двойное** освобождение.

## `Clone` и `Copy`

**`Copy: Clone`** — `Copy` **требует** `Clone`:

```rust
#[derive(Copy, Clone)]   // ← Copy требует Clone
struct Point { x: f32, y: f32 }
```

**Но** не наоборот — `Clone` **без** `Copy` **возможен**:

```rust
#[derive(Clone)]   // ← только Clone
struct Data { v: Vec<i32> }
```

| | `Clone` | `Copy` |
|---|---|---|
| **Глубокое** копирование | ✅ Возможно | ❌ Только побитовое |
| **Требует `Clone`** | — | ✅ Да |
| **Совместим с `Drop`** | ✅ Да | ❌ Нет |
| **Пример** | `String`, `Vec` | `i32`, `&T` |

## Когда добавлять `Copy`

**Только** для **маленьких** **POD**-типов:

- **размер** — **несколько** байт;
- **нет** `Drop`;
- **побитовое** копирование **безопасно** и **дёшево**;
- **семантически** нет смысла различать **оригинал** и **копию**.

**Примеры:**

```rust
#[derive(Copy, Clone)]
struct Point { x: f32, y: f32 }

#[derive(Copy, Clone)]
struct Color { r: u8, g: u8, b: u8, a: u8 }

#[derive(Copy, Clone)]
enum Direction { North, South, East, West }
```

**Не** добавляйте `Copy`:

```rust
#[derive(Clone)]   // ← не Copy
struct BigData { v: Vec<u8> }   // владеет кучей

#[derive(Clone)]   // ← не Copy
struct FileHandle { fd: i32 }   // владеет ресурсом ОС
```

## Сводная таблица

| Тип | `Copy`? | `Drop`? | Move/Copy? |
|---|---|---|---|
| `i32`, `f64`, `bool`, `char` | ✅ | ❌ | **Copy** |
| `&T` | ✅ | ❌ | **Copy** |
| `*const T` | ✅ | ❌ | **Copy** |
| `(i32, f64)`, `[i32; 5]` | ✅ | ❌ | **Copy** |
| `Option<i32>` | ✅ | ❌ | **Copy** |
| **`String`, `Vec<T>`** | ❌ | ✅ | **Move** |
| **`Box<T>`** | ❌ | ✅ | **Move** |
| **`File`** | ❌ | ✅ | **Move** |
| **`&mut T`, `*mut T`** | ❌ | ❌ | **Move** |
| `Rc<T>`, `Arc<T>` | ❌ | ✅ | **Clone** |

## Сводная таблица

| Аспект | `Move` | `Copy` |
|---|---|---|
| **Ассемблер** | `memcpy` | `memcpy` |
| **Стек** | Копируется | Копируется |
| **Куча** | Не копируется | Не копируется (POD) |
| **Исходный binding** | Инвалидирован | Валиден |
| **Ошибка** | `E0382` | — |
| **`Drop`** | ✅ | ❌ |
| **Реализуется** | Автоматически | `#[derive(Copy, Clone)]` |

## Итог

- **Move** и **Copy** на **железе** — **одно и то же** (`memcpy`).
- **Разница** — в том, что **компилятор** делает с **исходным binding**:
  - **move** → **инвалидирует** (`E0382`);
  - **copy** → **оставляет валидным**.
- **`Copy`** реализуют: **примитивы**, **`&T`**, **`*const T`**, **кортежи/массивы** из `Copy`, **`Option<T: Copy>`**.
- **`Copy`** **несовместим** с **`Drop`** — иначе **двойное** освобождение.
- **`String`**, **`Vec`**, **`Box`**, **`File`** — **move-only**.
- **`&mut T`**, **`*mut T`** — **не** `Copy` (изменяемый доступ).
- **`Copy: Clone`**, но **не** наоборот.
- **Добавляйте `Copy`** только для **маленьких** **POD**-типов.
- **Правило:** `Copy` — для **дешёвых** **безопасных** побитовых копий; **всё остальное** — **move**.
