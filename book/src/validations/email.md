# Email

> [!NOTE]
> Requires the `email` feature.

Validate a string is an RFC-compliant email address using the [`email_address`](https://docs.rs/email_address/latest/email_address/) crate.

```rust
# extern crate fortifier;
# use fortifier::Validate;
#
##[derive(Validate)]
struct User {
    #[validate(email)]
    email_address: String
}
```

## Types

### String

- [`str`](https://doc.rust-lang.org/std/primitive.str.html)
- [`String`](https://doc.rust-lang.org/std/string/struct.String.html)

Validate the string is an RFC-compliant email address.

### Email address

- [`EmailAddress`](https://docs.rs/email_address/latest/email_address/struct.EmailAddress.html)

Validate the value is an RFC-compliant email address.

An `EmailAddress` can be constructed using [`EmailAddress::new_unchecked`](https://docs.rs/email_address/latest/email_address/struct.EmailAddress.html#method.new_unchecked) or with different options passed to [`EmailAddress::parse_with_options`](https://docs.rs/email_address/latest/email_address/struct.EmailAddress.html#method.parse_with_options), so re-validation is required.

## Options

### `allow_display_text`

Whether display text is allowed. Defaults to `false`.

See [`Options::allow_display_text`](https://docs.rs/email_address/latest/email_address/struct.Options.html#structfield.allow_display_text) for details.

```rust
# extern crate fortifier;
# use fortifier::Validate;
#
##[derive(Validate)]
struct User<'a> {
    #[validate(email(allow_display_text = false))]
    email_address: &'a str
}

fn main() {
    let user = User {
        email_address: "simon@example.com"
    };
    assert!(user.validate_sync().is_ok());

    let user = User {
        email_address: "Simon <simon@example.com>"
    };
    assert!(user.validate_sync().is_err());
}
```

### `allow_domain_literal`

Whether domain literals are allowed. Defaults to `true`.

See [`Options::allow_domain_literal`](https://docs.rs/email_address/latest/email_address/struct.Options.html#structfield.allow_domain_literal) for details.

```rust
# extern crate fortifier;
# use fortifier::Validate;
#
##[derive(Validate)]
struct User<'a> {
    #[validate(email(allow_domain_literal = false))]
    email_address: &'a str
}

fn main() {
    let user = User {
        email_address: "simon@localhost"
    };
    assert!(user.validate_sync().is_ok());

    let user = User {
        email_address: "simon@[127.0.0.1]"
    };
    assert!(user.validate_sync().is_err());
}
```

### `minimum_sub_domains`

The minimum number of domain segments. Defaults to `0`.

See [`Options::minimum_sub_domains`](https://docs.rs/email_address/latest/email_address/struct.Options.html#structfield.minimum_sub_domains) for details.

```rust
# extern crate fortifier;
# use fortifier::Validate;
#
##[derive(Validate)]
struct User<'a> {
    #[validate(email(minimum_sub_domains = 2))]
    email_address: &'a str
}

fn main() {
    let user = User {
        email_address: "simon@example.com"
    };
    assert!(user.validate_sync().is_ok());

    let user = User {
        email_address: "simon@localhost"
    };
    assert!(user.validate_sync().is_err());
}
```
