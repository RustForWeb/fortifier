#![warn(missing_docs)]

//! Fortifier.

mod error;
mod validate;
mod validations;

pub use error::*;
pub use validate::*;
pub use validations::*;

#[cfg(feature = "macros")]
pub use fortifier_macros::*;

#[doc(hidden)]
pub mod external {
    #[cfg(feature = "serde")]
    pub use serde;

    #[cfg(feature = "utoipa")]
    pub use utoipa;
}
