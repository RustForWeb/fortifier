use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    rc::Rc,
    sync::Arc,
};

use url::{ParseError, Url};

/// URL validation error.
#[derive(Debug, Eq, PartialEq)]
pub enum UrlError {
    /// Parse error.
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
        if let Some(url) = self {
            T::url(url)
        } else {
            None
        }
    }
}

macro_rules! validate_with_deref {
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

validate_with_deref!(&T);
validate_with_deref!(Arc<T>);
validate_with_deref!(Box<T>);
validate_with_deref!(Rc<T>);
validate_with_deref!(Ref<'_, T>);
validate_with_deref!(RefMut<'_, T>);

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, cell::RefCell, rc::Rc, sync::Arc};

    use url::ParseError;

    use super::{UrlError, ValidateUrl};

    #[test]
    fn ok() {
        assert_eq!((*"http://localhost").validate_url(), Ok(()));
        assert_eq!("http://localhost".validate_url(), Ok(()));
        assert_eq!("http://localhost".to_owned().validate_url(), Ok(()));
        assert_eq!(
            Cow::<str>::Borrowed("http://localhost").validate_url(),
            Ok(())
        );
        assert_eq!(
            Cow::<str>::Owned("http://localhost".to_owned()).validate_url(),
            Ok(())
        );

        assert_eq!(None::<&str>.validate_url(), Ok(()));
        assert_eq!(Some("http://localhost").validate_url(), Ok(()));

        assert_eq!((&"http://localhost").validate_url(), Ok(()));
        #[expect(unused_allocation)]
        {
            assert_eq!(Box::new("http://localhost").validate_url(), Ok(()));
        }
        assert_eq!(Arc::new("http://localhost").validate_url(), Ok(()));
        assert_eq!(Rc::new("http://localhost").validate_url(), Ok(()));

        let cell = RefCell::new("http://localhost");
        assert_eq!(cell.borrow().validate_url(), Ok(()));
        assert_eq!(cell.borrow_mut().validate_url(), Ok(()));
    }

    #[test]
    fn parse_error() {
        assert_eq!(
            (*"http://").validate_url(),
            Err(UrlError::Parse(ParseError::EmptyHost))
        );
        assert_eq!(
            "http://".validate_url(),
            Err(UrlError::Parse(ParseError::EmptyHost))
        );
        assert_eq!(
            "http://".to_owned().validate_url(),
            Err(UrlError::Parse(ParseError::EmptyHost))
        );
        assert_eq!(
            Cow::<str>::Borrowed("http://").validate_url(),
            Err(UrlError::Parse(ParseError::EmptyHost))
        );
        assert_eq!(
            Cow::<str>::Owned("http://".to_owned()).validate_url(),
            Err(UrlError::Parse(ParseError::EmptyHost))
        );

        assert_eq!(
            Some("http://").validate_url(),
            Err(UrlError::Parse(ParseError::EmptyHost))
        );

        assert_eq!(
            (&"http://").validate_url(),
            Err(UrlError::Parse(ParseError::EmptyHost))
        );
        #[expect(unused_allocation)]
        {
            assert_eq!(
                Box::new("http://").validate_url(),
                Err(UrlError::Parse(ParseError::EmptyHost))
            );
        }
        assert_eq!(
            Arc::new("http://").validate_url(),
            Err(UrlError::Parse(ParseError::EmptyHost))
        );
        assert_eq!(
            Rc::new("http://").validate_url(),
            Err(UrlError::Parse(ParseError::EmptyHost))
        );

        let cell = RefCell::new("http://");
        assert_eq!(
            cell.borrow().validate_url(),
            Err(UrlError::Parse(ParseError::EmptyHost))
        );
        assert_eq!(
            cell.borrow_mut().validate_url(),
            Err(UrlError::Parse(ParseError::EmptyHost))
        );
    }
}
