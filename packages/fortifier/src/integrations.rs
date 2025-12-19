#[cfg(feature = "serde")]
pub mod serde;

#[doc(hidden)]
pub mod external {
    #[cfg(feature = "serde")]
    pub use serde;

    #[cfg(feature = "utoipa")]
    pub use utoipa;
}
