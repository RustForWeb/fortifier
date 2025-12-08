use std::{
    error::Error,
    fmt::{self, Debug, Display},
    pin::Pin,
};

/// Validation errors.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ValidationErrors<E>(Vec<E>);

impl<E: Debug> Display for ValidationErrors<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<E: Error> Error for ValidationErrors<E> {}

impl<E> FromIterator<E> for ValidationErrors<E> {
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<E> From<Vec<E>> for ValidationErrors<E> {
    fn from(value: Vec<E>) -> Self {
        Self(value)
    }
}

/// Validate a schema.
pub trait Validate {
    /// Validation error.
    type Error: Error;

    /// Validate schema using all validators.
    fn validate(
        &self,
    ) -> Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>> + Send>>
    where
        Self: Sync,
    {
        Box::pin(async {
            self.validate_sync()?;
            self.validate_async().await
        })
    }

    /// Validate schema using only synchronous validators.
    fn validate_sync(&self) -> Result<(), ValidationErrors<Self::Error>>;

    /// Validate schema using only asynchronous validators.
    fn validate_async(
        &self,
    ) -> Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>> + Send>>;
}
