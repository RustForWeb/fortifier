//! Serde utilities

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
            errors: &'a T,
        }

        Wrapper { errors: value }.serialize(serializer)
    }
}
