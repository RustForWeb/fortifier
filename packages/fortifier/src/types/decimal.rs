//! Infallible validation implementations for [`rust_decimal`] types.
//!
//! This prevents having to specify `#[validate(skip)]` for these types.

use rust_decimal::Decimal;

use crate::validate_ok;

validate_ok!(Decimal);
