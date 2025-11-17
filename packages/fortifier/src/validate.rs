use std::{error::Error, pin::Pin};

pub trait Validate {
    type Error: Error;

    fn validate(&self) -> Pin<Box<impl Future<Output = Result<(), Self::Error>> + Send>>
    where
        Self: Sync,
    {
        Box::pin(async {
            self.validate_sync()?;
            self.validate_async().await
        })
    }

    fn validate_sync(&self) -> Result<(), Self::Error>;

    fn validate_async(&self) -> Pin<Box<impl Future<Output = Result<(), Self::Error>> + Send>>;
}
