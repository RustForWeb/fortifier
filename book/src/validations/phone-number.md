# Phone Number

> [!NOTE]
> Requires the `phone-number` feature.

Validate a string is a specification-compliant phone number using the [`phonenumber`](https://docs.rs/phonenumber/latest/phonenumber/) crate.

```rust
# extern crate fortifier;
#
use fortifier::Validate;

#[derive(Validate)]
struct User {
    #[validate(phone_number)]
    phone_number: String
}
```

## Types

### String

- [`str`](https://doc.rust-lang.org/std/primitive.str.html)
- [`String`](https://doc.rust-lang.org/std/string/struct.String.html)

Validate the string is a speficiation-compliant phone number.

### Phone number

- [`PhoneNumber`](https://docs.rs/phonenumber/latest/phonenumber/struct.PhoneNumber.html)

Validate the value is a specification-compliant phone number.

A `PhoneNumber` can be constructed with different options passed to [`phonenumber::parse`](https://docs.rs/phonenumber/latest/phonenumber/fn.parse.html), so re-validation is required.

## Options

### `allowed_countries`

A list of allowed country codes.

See [`phonenumber::country::Id`](https://docs.rs/phonenumber/latest/phonenumber/country/enum.Id.html) for available country codes. This enum is re-exported as [`fortifier::PhoneNumberCountry`].

```rust
# extern crate fortifier;
#
use fortifier::{PhoneNumberCountry, Validate};

#[derive(Validate)]
struct User<'a> {
    #[validate(phone_number(allowed_countries = vec![PhoneNumberCountry::GB]))]
    phone_number: &'a str
}

fn main() {
    let user = User {
        phone_number: "+44 20 7946 0000"
    };
    assert!(user.validate_sync().is_ok());

    let user = User {
        phone_number: "+31 6 12345678"
    };
    assert!(user.validate_sync().is_err());
}
```

### `default_country`

Default country code to use when no country code is provided.

See [`phonenumber::country::Id`](https://docs.rs/phonenumber/latest/phonenumber/country/enum.Id.html) for available country codes. This enum is re-exported as [`fortifier::PhoneNumberCountry`].

```rust
# extern crate fortifier;
#
use fortifier::{PhoneNumberCountry, Validate};

#[derive(Validate)]
struct User<'a> {
    #[validate(phone_number(default_country = PhoneNumberCountry::GB))]
    phone_number: &'a str
}

fn main() {
    let user = User {
        phone_number: "020 7946 0000"
    };
    assert!(user.validate_sync().is_ok());
}
```
