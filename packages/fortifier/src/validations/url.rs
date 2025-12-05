use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    rc::Rc,
    sync::Arc,
};

use url::{ParseError, Url};

/// URL validation error.
#[derive(Debug)]
pub enum UrlError {
    /// Invalid URL.
    Parse(ParseError),
}

/// Validate a URL.
pub trait ValidateUrl {
    /// The URL.
    fn url(&self) -> Option<Cow<'_, str>>;

    /// Validate URL.
    fn validate_url(&self) -> Result<(), UrlError> {
        let Some(url) = self.url() else {
            return Ok(());
        };

        Url::parse(&url).map_err(UrlError::Parse)?;

        Ok(())
    }
}

macro_rules! validate_type_with_deref {
    ($type:ty) => {
        impl<T> ValidateUrl for $type
        where
            T: ValidateUrl,
        {
            fn url(&self) -> Option<Cow<'_, str>> {
                T::url(self)
            }
        }
    };
}

validate_type_with_deref!(&T);
validate_type_with_deref!(Arc<T>);
validate_type_with_deref!(Box<T>);
validate_type_with_deref!(Rc<T>);
validate_type_with_deref!(Ref<'_, T>);
validate_type_with_deref!(RefMut<'_, T>);

impl ValidateUrl for str {
    fn url(&self) -> Option<Cow<'_, str>> {
        Some(self.into())
    }
}

impl ValidateUrl for &str {
    fn url(&self) -> Option<Cow<'_, str>> {
        Some((*self).into())
    }
}

impl ValidateUrl for String {
    fn url(&self) -> Option<Cow<'_, str>> {
        Some(self.into())
    }
}

impl ValidateUrl for Cow<'_, str> {
    fn url(&self) -> Option<Cow<'_, str>> {
        Some(self.clone())
    }
}

impl<T> ValidateUrl for Option<T>
where
    T: ValidateUrl,
{
    fn url(&self) -> Option<Cow<'_, str>> {
        if let Some(s) = self { T::url(s) } else { None }
    }
}
