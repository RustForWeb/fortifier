use std::{collections::HashMap, error::Error, fmt::Debug, pin::Pin};

use crate::error::{IndexedValidationError, KeyedValidationError, ValidationErrors};

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

impl<T> ValidateWithContext for Vec<T>
where
    T: ValidateWithContext + Send + Sync,
    T::Error: Error + Send + Sync,
{
    type Context = T::Context;
    type Error = IndexedValidationError<T::Error>;

    fn validate_sync_with_context(
        &self,
        context: &Self::Context,
    ) -> Result<(), ValidationErrors<Self::Error>> {
        let mut errors = vec![];

        for (index, value) in self.iter().enumerate() {
            if let Err(error) = value.validate_sync_with_context(context) {
                for error in error {
                    errors.push(IndexedValidationError { index, error });
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.into())
        }
    }

    fn validate_async_with_context(
        &self,
        context: &Self::Context,
    ) -> Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>> + Send>> {
        Box::pin(async move {
            let mut errors = vec![];

            for (index, value) in self.iter().enumerate() {
                if let Err(error) = value.validate_async_with_context(context).await {
                    for error in error {
                        errors.push(IndexedValidationError { index, error });
                    }
                }
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors.into())
            }
        })
    }
}

// TODO: Should this validate both keys and values?
impl<K, V> ValidateWithContext for HashMap<K, V>
where
    K: Clone + Debug + Send + Sync,
    V: ValidateWithContext + Send + Sync,
    V::Error: Error + Send + Sync,
{
    type Context = V::Context;
    type Error = KeyedValidationError<K, V::Error>;

    fn validate_sync_with_context(
        &self,
        context: &Self::Context,
    ) -> Result<(), ValidationErrors<Self::Error>> {
        let mut errors = vec![];

        for (key, value) in self.iter() {
            if let Err(error) = value.validate_sync_with_context(context) {
                for error in error {
                    errors.push(KeyedValidationError {
                        key: key.clone(),
                        error,
                    });
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.into())
        }
    }

    fn validate_async_with_context(
        &self,
        context: &Self::Context,
    ) -> Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>> + Send>> {
        Box::pin(async move {
            let mut errors = vec![];

            for (key, value) in self.iter() {
                if let Err(error) = value.validate_async_with_context(context).await {
                    for error in error {
                        errors.push(KeyedValidationError {
                            key: key.clone(),
                            error,
                        });
                    }
                }
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors.into())
            }
        })
    }
}
