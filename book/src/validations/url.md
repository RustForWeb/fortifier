# URL

> [!NOTE]
> Requires the `url` feature.

Validate a string is a specification-compliant URL using the [`url`](https://docs.rs/url/latest/url/) crate.

```rust
# extern crate fortifier;
# use fortifier::Validate;
#
##[derive(Validate)]
struct User {
    #[validate(url)]
    url: String
}
```

## Types

### String

- [`str`](https://doc.rust-lang.org/std/primitive.str.html)
- [`String`](https://doc.rust-lang.org/std/string/struct.String.html)

Validate the string is a specification-compliant URL.

### URL

- [`Url`](https://docs.rs/url/latest/url/struct.Url.html)

Validate the value is a specification-compliant URL.

A `Url` can only be constructed by parsing it, so no re-validation is performed.
