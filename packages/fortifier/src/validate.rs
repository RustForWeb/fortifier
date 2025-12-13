use std::{
    error::Error,
    fmt::{self, Debug, Display},
    pin::Pin,
};

/// Validation errors.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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

/// Validate a schema with context.
pub trait ValidateWithContext {
    /// Validation context.
    type Context: Send + Sync;

    /// Validation error.
    type Error: Error;

    /// Validate schema using all validators with context.
    fn validate_with_context(
        &self,
        context: &Self::Context,
    ) -> Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>> + Send>>
    where
        Self: Sync,
    {
        Box::pin(async {
            self.validate_sync_with_context(context)?;
            self.validate_async_with_context(context).await
        })
    }

    /// Validate schema using only synchronous validators with context.
    fn validate_sync_with_context(
        &self,
        context: &Self::Context,
    ) -> Result<(), ValidationErrors<Self::Error>>;

    /// Validate schema using only asynchronous validators  with context.
    fn validate_async_with_context(
        &self,
        context: &Self::Context,
    ) -> Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>> + Send>>;
}

/// Validate a schema.
pub trait Validate: ValidateWithContext<Context = ()> {
    /// Validate schema using all validators.
    fn validate(
        &self,
    ) -> Pin<Box<impl Future<Output = Result<(), ValidationErrors<<Self>::Error>>> + Send>>
    where
        Self: Sync,
    {
        self.validate_with_context(&())
    }

    /// Validate schema using only synchronous validators.
    fn validate_sync(&self) -> Result<(), ValidationErrors<Self::Error>> {
        self.validate_sync_with_context(&())
    }

    /// Validate schema using only asynchronous validators.
    fn validate_async(
        &self,
    ) -> Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>> + Send>> {
        self.validate_async_with_context(&())
    }
}
