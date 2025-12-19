use std::{
    error::Error,
    fmt::{self, Debug, Display},
    ops::Deref,
};

/// Validation errors.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ValidationErrors<E>(Vec<E>);

impl<E> Deref for ValidationErrors<E> {
    type Target = Vec<E>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

impl<E> IntoIterator for ValidationErrors<E> {
    type Item = E;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Validation error with index.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct IndexedValidationError<E: Error> {
    /// The index.
    pub index: usize,

    /// The error.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub error: E,
}

impl<E: Error> IndexedValidationError<E> {
    /// Constructs a new [`IndexedValidationError`].
    pub fn new(index: usize, error: E) -> Self {
        Self { index, error }
    }
}

impl<E: Error> Display for IndexedValidationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl<E: Error> Error for IndexedValidationError<E> {}

/// Validation error with key.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct KeyedValidationError<K, E: Error> {
    /// The key.
    pub key: K,

    /// The error.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub error: E,
}

impl<K, E: Error> KeyedValidationError<K, E> {
    /// Constructs a new [`KeyedValidationError`].
    pub fn new(key: K, error: E) -> Self {
        Self { key, error }
    }
}

impl<K: Debug, E: Error> Display for KeyedValidationError<K, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl<K: Debug, E: Error> Error for KeyedValidationError<K, E> {}
