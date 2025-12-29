//! Infallible validation implementations for [`chrono`] types.
//!
//! This prevents having to specify `#[validate(skip)]` for these types.

use std::convert::Infallible;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone};

use crate::{
    error::ValidationErrors,
    validate::{Validate, ValidateWithContext},
    validate_ok,
};

validate_ok!(NaiveDate);
validate_ok!(NaiveDateTime);
validate_ok!(NaiveTime);
validate_ok!(TimeDelta);

impl<Tz: TimeZone> ValidateWithContext for DateTime<Tz> {
    type Context = ();
    type Error = Infallible;

    fn validate_sync_with_context(
        &self,
        _context: &Self::Context,
    ) -> Result<(), ValidationErrors<Self::Error>> {
        Ok(())
    }

    fn validate_async_with_context(
        &self,
        _context: &Self::Context,
    ) -> ::std::pin::Pin<Box<impl Future<Output = Result<(), ValidationErrors<Self::Error>>> + Send>>
    {
        Box::pin(async { Ok(()) })
    }
}

impl<Tz: TimeZone> Validate for DateTime<Tz> {}
