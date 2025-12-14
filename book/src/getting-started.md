# Getting Started

## Install

Add Fortifier to your project:

```shell
cargo add fortifier --features email-address
```

See [Installation](installation.md) for more details.

## Data structure

Define a data structure:

```rust
struct CreateUser {
    email_address: String,
    name: String,
}
```

## Derive

Derive the `Validate` trait:

```rust
# extern crate fortifier;
use fortifier::Validate;

#[derive(Validate)]
struct CreateUser {
    email_address: String,
    name: String,
}
```

## Validations

Define validations:

```rust
# extern crate fortifier;
use fortifier::Validate;

#[derive(Validate)]
struct CreateUser {
    #[validate(email_address)]
    email_address: String,

    #[validate(length(min = 1, max = 256))]
    name: String,
}
```

## Validate

Fortifier supports both synchronous and asynchronous validation. This example will only use synchronous validation.

Call the `validate_sync` method on the data structure:

```rust
# extern crate fortifier;
use fortifier::{EmailAddressError, LengthError, Validate, ValidationErrors};

#[derive(Validate)]
struct CreateUser {
    #[validate(email_address)]
    email_address: String,

    #[validate(length(min = 1, max = 256))]
    name: String,
}

fn main() {
    let data = CreateUser {
        email_address: "amy.pond@example.com".to_string(),
        name: "Amy Pond".to_string(),
    };

    assert_eq!(data.validate_sync(), Ok(()));

    let data = CreateUser {
        email_address: "invalid".to_string(),
        name: "".to_string(),
    };

    assert_eq!(
        data.validate_sync(),
        Err(ValidationErrors::from_iter([
            CreateUserValidationError::EmailAddress(
                EmailAddressError::MissingSeparator {},
            ),
            CreateUserValidationError::Name(
                LengthError::Min {
                    min: 1,
                    length: 0,
                }
            ),
        ])),
    );
}
```

## Next Steps

- [Installation](./installation.md) - Lists all available features.
- [Validate](./validate/README.md) - Describes how to use the `Validate` derive macro.
- [Validations](./validations/README.md) - Explains all available validations and their options.
