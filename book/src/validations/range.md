# Range

Validate a value is within a range.

```rust
# extern crate fortifier;
#
use fortifier::Validate;

#[derive(Validate)]
struct Object {
    #[validate(range(exclusive_min = 0.0, max = 100.0))]
    height: f64,
}
```

## Types

### Boolean

- [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)

### Number

- [`u8`](https://doc.rust-lang.org/std/primitive.u8.html)
- [`u16`](https://doc.rust-lang.org/std/primitive.u16.html)
- [`u32`](https://doc.rust-lang.org/std/primitive.u32.html)
- [`u64`](https://doc.rust-lang.org/std/primitive.u64.html)
- [`u128`](https://doc.rust-lang.org/std/primitive.u128.html)
- [`usize`](https://doc.rust-lang.org/std/primitive.usize.html)
- [`i8`](https://doc.rust-lang.org/std/primitive.i8.html)
- [`i16`](https://doc.rust-lang.org/std/primitive.i16.html)
- [`i32`](https://doc.rust-lang.org/std/primitive.i32.html)
- [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)
- [`i128`](https://doc.rust-lang.org/std/primitive.i128.html)
- [`isize`](https://doc.rust-lang.org/std/primitive.isize.html)
- [`f32`](https://doc.rust-lang.org/std/primitive.f32.html)
- [`f64`](https://doc.rust-lang.org/std/primitive.f64.html)

### Character

- [`char`](https://doc.rust-lang.org/std/primitive.char.html)

### String

- [`str`](https://doc.rust-lang.org/std/primitive.str.html)
- [`String`](https://doc.rust-lang.org/std/string/struct.String.html)

### Other

- [`DateTime`](https://docs.rs/chrono/latest/chrono/struct.DateTime.html) (requires feature `chrono`)
- [`NaiveDate`](https://docs.rs/chrono/latest/chrono/struct.NaiveDate.html) (requires feature `chrono`)
- [`NaiveDateTime`](https://docs.rs/chrono/latest/chrono/struct.NaiveDateTime.html) (requires feature `chrono`)
- [`NaiveTime`](https://docs.rs/chrono/latest/chrono/struct.NaiveTime.html) (requires feature `chrono`)
- [`TimeDelta`](https://docs.rs/chrono/latest/chrono/struct.TimeDelta.html) (requires feature `chrono`)
- [`Decimal`](https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html) (requires feature `decimal`)
- [`Uuid`](https://docs.rs/uuid/latest/uuid/struct.Uuid.html) (requires feature `uuid`)

## Options

### `min`

The value should be equal to or greater than the specified expression.

```rust
# extern crate fortifier;
#
use fortifier::Validate;

#[derive(Validate)]
struct Object {
    #[validate(range(min = 0.0))]
    height: f64
}
```

### `max`

The value should be equal to or less than the specified expression.

```rust
# extern crate fortifier;
#
use fortifier::Validate;

#[derive(Validate)]
struct Object {
    #[validate(range(max = 100.0))]
    height: f64
}
```

### `exclusive_min`

The value should be greater than the specified expression.

```rust
# extern crate fortifier;
#
use fortifier::Validate;

#[derive(Validate)]
struct Object {
    #[validate(range(exclusive_min = 0.0))]
    height: f64
}
```

### `exclusive_max`

The value should be less than the specified expression.

```rust
# extern crate fortifier;
#
use fortifier::Validate;

#[derive(Validate)]
struct Object {
    #[validate(range(exclusive_max = 100.0))]
    height: f64
}
```
