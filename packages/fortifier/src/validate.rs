use std::{
    error::Error,
    fmt::{self, Display},
    pin::Pin,
};

#[derive(Debug)]
pub struct ValidationErrors<E: Error>(Vec<E>);

impl<E: Error> Display for ValidationErrors<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<E: Error> Error for ValidationErrors<E> {}

impl<E: Error> From<Vec<E>> for ValidationErrors<E> {
    fn from(value: Vec<E>) -> Self {
        Self(value)
    }
}

pub trait Validate {
    type Error: Error;

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

    fn validate_sync(&self) -> Result<(), ValidationErrors<Self::Error>>;

    fn validate_async(
        &self,
    ) -> Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>> + Send>>;
}
