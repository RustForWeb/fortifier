# Regular Expression

> [!NOTE]
> Requires the `regex` feature.

Validate a string matches a regular expression using the [`regex`](https://docs.rs/regex/latest/regex/) crate.

```rust
# extern crate fortifier;
# extern crate regex;
#
use std::sync::LazyLock;

use fortifier::Validate;
use regex::Regex;

static COUNTRY_CODE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[A-Z]{2}").expect("valid regex"));

#[derive(Validate)]
struct User {
    #[validate(regex = &COUNTRY_CODE_REGEX)]
    country_code: String,
}
```

## Types

### String

- [`str`](https://doc.rust-lang.org/std/primitive.str.html)
- [`String`](https://doc.rust-lang.org/std/string/struct.String.html)

Validate the string matches the specified regular expression.

## Options

### `expression`

The regular expression to match against.

The recommended approach for global regular expressions is to use a static [`LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html).

```rust
# extern crate fortifier;
# extern crate regex;
#
use std::sync::LazyLock;

use fortifier::Validate;
use regex::Regex;

static COUNTRY_CODE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[A-Z]{2}").expect("valid regex"));

#[derive(Validate)]
struct User {
    #[validate(regex(expression = &COUNTRY_CODE_REGEX))]
    country_code: String,
}
```
