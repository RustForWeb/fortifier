# Length

Validate the length of a string or iterable.

```rust
# extern crate fortifier;
# use fortifier::Validate;
#
##[derive(Validate)]
struct User {
    #[validate(length(min = 1, max = 256))]
    name: String
}
```

## Types

### String

- [`str`](https://doc.rust-lang.org/std/primitive.str.html)
- [`String`](https://doc.rust-lang.org/std/string/struct.String.html)

Validate the amount of characters ([`chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars)) in a string.

### Iterable

- [`[T]` (slice)](https://doc.rust-lang.org/std/primitive.slice.html)
- [`[T; N]` (array)](https://doc.rust-lang.org/std/primitive.array.html)
- [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)
- [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
- [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
- [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [`IndexSet`](https://docs.rs/indexmap/latest/indexmap/set/struct.IndexSet.html) (requires feature `indexmap`)
- [`IndexMap`](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html) (requires feature `indexmap`)
- [`LinkedList`](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)
- [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)

Validate the amount of entries in an iterable.

## Options

### Equal

The length should be equal to the specified expression.

```rust
# extern crate fortifier;
# use fortifier::Validate;
#
##[derive(Validate)]
struct User {
    #[validate(length(equal = 2))]
    country_code: String
}
```

### Minimum

The length should be equal to or greater than the specified expression.

```rust
# extern crate fortifier;
# use fortifier::Validate;
#
##[derive(Validate)]
struct User {
    #[validate(length(min = 1))]
    name: String
}
```

### Maximum

The length should be equal to or less than the specified expression.

```rust
# extern crate fortifier;
# use fortifier::Validate;
#
##[derive(Validate)]
struct User {
    #[validate(length(max = 256))]
    name: String
}
```
