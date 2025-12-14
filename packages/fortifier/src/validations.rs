#[cfg(feature = "email-address")]
mod email_address;
mod length;
#[cfg(feature = "phone-number")]
mod phone_number;
#[cfg(feature = "regex")]
mod regex;
#[cfg(feature = "url")]
mod url;

#[cfg(feature = "email-address")]
pub use email_address::*;
pub use length::*;
#[cfg(feature = "phone-number")]
pub use phone_number::*;
#[cfg(feature = "regex")]
pub use regex::*;
#[cfg(feature = "url")]
pub use url::*;
