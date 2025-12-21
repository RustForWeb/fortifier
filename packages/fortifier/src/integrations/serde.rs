//! Serde utilities

use std::fmt;

use serde::de::{Error, Unexpected, Visitor};

/// Deserialize and serialize with `errors` field.
pub mod errors {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Deserialize with `errors` field.
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper<T> {
            errors: T,
        }

        let Wrapper { errors } = Wrapper::deserialize(deserializer)?;
        Ok(errors)
    }

    /// Serialize with `errors` field.
    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        #[derive(Serialize)]
        struct Wrapper<'a, T> {
            code: &'static str,
            errors: &'a T,
        }

        Wrapper {
            code: "nested",
            errors: value,
        }
        .serialize(serializer)
    }
}

/// Serde visitor for a static string.
///
/// Based on `MustBeStrVisitor` from [`monostate`](https://crates.io/crates/monostate).
pub struct MustBeStrVisitor(pub &'static str);

impl<'de> Visitor<'de> for MustBeStrVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "string {:?}", self.0)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v == self.0 {
            Ok(())
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }
}
