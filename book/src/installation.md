# Installation

```shell
cargo add fortifier
```

- [View on crates.io](https://crates.io/crates/fortifier)
- [View on docs.rs](https://docs.rs/fortifier/latest/fortifier/)
- [View source](https://github.com/RustForWeb/fortifier/tree/main/packages/fortifier)

## Features

### General

- `macros` (default) - Derive macro for the `Validate` trait ([`fortifier-macros`](https://docs.rs/fortifier-macros/latest/fortifier_macros/)).
- `message` - Add a human-readable `message` field to validation errors.

### Types

- `all-types` - Enable all features below.
- `chrono` - Support for the `DateTime`, `NaiveDate`, `NaiveDateTime`, `NaiveTime` and `TimeDelta` types from the [`chrono`](https://docs.rs/chrono/latest/chrono/) crate.
- `decimal` - Support for the `Decimal` type from the [`rust_decimal`](https://docs.rs/rust_decimal/latest/rust_decimal/) crate.
- `indexmap` - Support for the `IndexMap` and `IndexSet` types from the [`indexmap`](https://docs.rs/indexmap/latest/indexmap/) crate.
- `uuid` - Support for the `Uuid` type from the [`uuid`](https://docs.rs/uuid/latest/uuid/) crate.

### Validations

- `all-validations` - Enable all features below.
- `email-address` - Email address validation using the [`email_address`](https://docs.rs/email_address/latest/email_address/) crate.
- `phone-number` - Phone number validation using the [`phonenumber`](https://docs.rs/phonenumber/latest/phonenumber/) crate.
- `regex` - Regular expression validation using the [`regex`](https://docs.rs/regex/latest/regex/) crate.
- `url` - URL validation using the [`url`](https://docs.rs/url/latest/url/) crate.

### Integrations

- `serde` - Support for the [`serde`](https://docs.rs/serde/latest/serde/) crate. Derives the `Deserialize` and `Serialize` traits for validation errors.
- `utoipa` - Support for the [`utoipa`](https://docs.rs/utoipa/latest/utoipa/) crate. Derives the `ToSchema` trait for validation errors.
