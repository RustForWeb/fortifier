use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    rc::Rc,
    sync::Arc,
};

use url::{ParseError, Url};

/// URL validation error.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(
        tag = "subcode",
        rename_all = "camelCase",
        rename_all_fields = "camelCase"
    )
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UrlError {
    /// Empty host error.
    EmptyHost {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Invalid international domain name error.
    IdnaError {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Invalid port error.
    InvalidPort {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Invalid IPv4 address error.
    InvalidIpv4Address {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Invalid IPv6 address error.
    InvalidIpv6Address {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Invalid domain character error.
    InvalidDomainCharacter {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Relative URL without base error.
    RelativeUrlWithoutBase {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Relative URL with cannot-be-a-base base error.
    RelativeUrlWithCannotBeABaseBase {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Set host on cannot-be-a-base URL error.
    SetHostOnCannotBeABaseUrl {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Overflow error.
    Overflow {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Unknown error.
    Unknown {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
}

impl From<ParseError> for UrlError {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::EmptyHost => UrlError::EmptyHost {
                #[cfg(feature = "message")]
                message: value.to_string(),
            },
            ParseError::IdnaError => UrlError::IdnaError {
                #[cfg(feature = "message")]
                message: value.to_string(),
            },
            ParseError::InvalidPort => UrlError::InvalidPort {
                #[cfg(feature = "message")]
                message: value.to_string(),
            },
            ParseError::InvalidIpv4Address => UrlError::InvalidIpv4Address {
                #[cfg(feature = "message")]
                message: value.to_string(),
            },
            ParseError::InvalidIpv6Address => UrlError::InvalidIpv6Address {
                #[cfg(feature = "message")]
                message: value.to_string(),
            },
            ParseError::InvalidDomainCharacter => UrlError::InvalidDomainCharacter {
                #[cfg(feature = "message")]
                message: value.to_string(),
            },
            ParseError::RelativeUrlWithoutBase => UrlError::RelativeUrlWithoutBase {
                #[cfg(feature = "message")]
                message: value.to_string(),
            },
            ParseError::RelativeUrlWithCannotBeABaseBase => {
                UrlError::RelativeUrlWithCannotBeABaseBase {
                    #[cfg(feature = "message")]
                    message: value.to_string(),
                }
            }
            ParseError::SetHostOnCannotBeABaseUrl => UrlError::SetHostOnCannotBeABaseUrl {
                #[cfg(feature = "message")]
                message: value.to_string(),
            },
            ParseError::Overflow => UrlError::Overflow {
                #[cfg(feature = "message")]
                message: value.to_string(),
            },
            #[cfg_attr(not(feature = "message"), allow(unused_variables))]
            value => UrlError::Overflow {
                #[cfg(feature = "message")]
                message: value.to_string(),
            },
        }
    }
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

        Url::parse(&url).map_err(UrlError::from)?;

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

impl ValidateUrl for Url {
    fn url(&self) -> Option<Cow<'_, str>> {
        Some(self.as_str().into())
    }

    fn validate_url(&self) -> Result<(), UrlError> {
        // URL has already been parsed, so it must be valid.
        Ok(())
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

    use url::{ParseError, Url};

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
        assert_eq!(
            Url::parse("http://localhost")
                .expect("URL should be valid.")
                .validate_url(),
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
            Err(UrlError::from(ParseError::EmptyHost))
        );
        assert_eq!(
            "http://".validate_url(),
            Err(UrlError::from(ParseError::EmptyHost))
        );
        assert_eq!(
            "http://".to_owned().validate_url(),
            Err(UrlError::from(ParseError::EmptyHost))
        );
        assert_eq!(
            Cow::<str>::Borrowed("http://").validate_url(),
            Err(UrlError::from(ParseError::EmptyHost))
        );
        assert_eq!(
            Cow::<str>::Owned("http://".to_owned()).validate_url(),
            Err(UrlError::from(ParseError::EmptyHost))
        );

        assert_eq!(
            Some("http://").validate_url(),
            Err(UrlError::from(ParseError::EmptyHost))
        );

        assert_eq!(
            (&"http://").validate_url(),
            Err(UrlError::from(ParseError::EmptyHost))
        );
        #[expect(unused_allocation)]
        {
            assert_eq!(
                Box::new("http://").validate_url(),
                Err(UrlError::EmptyHost {
                    #[cfg(feature = "message")]
                    message: "empty host".to_owned(),
                })
            );
        }
        assert_eq!(
            Arc::new("http://").validate_url(),
            Err(UrlError::from(ParseError::EmptyHost))
        );
        assert_eq!(
            Rc::new("http://").validate_url(),
            Err(UrlError::from(ParseError::EmptyHost))
        );

        let cell = RefCell::new("http://");
        assert_eq!(
            cell.borrow().validate_url(),
            Err(UrlError::from(ParseError::EmptyHost))
        );
        assert_eq!(
            cell.borrow_mut().validate_url(),
            Err(UrlError::from(ParseError::EmptyHost))
        );
    }
}
