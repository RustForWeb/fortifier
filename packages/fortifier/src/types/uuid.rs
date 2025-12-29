//! Infallible validation implementations for [`uuid`] types.
//!
//! This prevents having to specify `#[validate(skip)]` for these types.

use uuid::Uuid;

use crate::validate_ok;

validate_ok!(Uuid);
