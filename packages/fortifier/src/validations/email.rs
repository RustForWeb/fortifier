use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    rc::Rc,
    sync::Arc,
};

use email_address::EmailAddress;
pub use email_address::Options as EmailOptions;

/// Email validation error.
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
pub enum EmailError {
    /// Invalid character error.
    InvalidCharacter {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Missing separator error.
    MissingSeparator {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Locale part empty error.
    LocalPartEmpty {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Local part too long error.
    LocalPartTooLong {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Domain empty error.
    DomainEmpty {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Domain too long error.
    DomainTooLong {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Subdomain empty error.
    SubDomainEmpty {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Subdomain too long error.
    SubDomainTooLong {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Domain too few error.
    DomainTooFew {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Domain invalid separator error.
    DomainInvalidSeparator {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Unbalanced quotes error.
    UnbalancedQuotes {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Invalid comment error.
    InvalidComment {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Invalid IP Address error.
    InvalidIPAddress {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Unsupported domain literal error.
    UnsupportedDomainLiteral {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Unsupported display name error.
    UnsupportedDisplayName {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Missing display name error.
    MissingDisplayName {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Missing end bracket error.
    MissingEndBracket {
        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
}

impl From<email_address::Error> for EmailError {
    fn from(value: email_address::Error) -> Self {
        match value {
            email_address::Error::InvalidCharacter => Self::InvalidCharacter {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::MissingSeparator => Self::MissingSeparator {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::LocalPartEmpty => Self::LocalPartEmpty {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::LocalPartTooLong => Self::LocalPartTooLong {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::DomainEmpty => Self::DomainEmpty {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::DomainTooLong => Self::DomainTooLong {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::SubDomainEmpty => Self::SubDomainEmpty {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::SubDomainTooLong => Self::SubDomainTooLong {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::DomainTooFew => Self::DomainTooFew {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::DomainInvalidSeparator => Self::DomainInvalidSeparator {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::UnbalancedQuotes => Self::UnbalancedQuotes {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::InvalidComment => Self::InvalidComment {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::InvalidIPAddress => Self::InvalidIPAddress {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::UnsupportedDomainLiteral => Self::UnsupportedDomainLiteral {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::UnsupportedDisplayName => Self::UnsupportedDisplayName {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::MissingDisplayName => Self::MissingDisplayName {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::MissingEndBracket => Self::MissingEndBracket {
                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
        }
    }
}

/// Validate an email address.
pub trait ValidateEmail {
    /// The email address.
    fn email(&self) -> Option<Cow<'_, str>>;

    /// Validate email address.
    fn validate_email(&self, options: EmailOptions) -> Result<(), EmailError> {
        let Some(email) = self.email() else {
            return Ok(());
        };

        EmailAddress::parse_with_options(&email, options).map_err(EmailError::from)?;

        Ok(())
    }
}

impl ValidateEmail for str {
    fn email(&self) -> Option<Cow<'_, str>> {
        Some(self.into())
    }
}

impl ValidateEmail for &str {
    fn email(&self) -> Option<Cow<'_, str>> {
        Some((*self).into())
    }
}

impl ValidateEmail for String {
    fn email(&self) -> Option<Cow<'_, str>> {
        Some(self.into())
    }
}

impl ValidateEmail for Cow<'_, str> {
    fn email(&self) -> Option<Cow<'_, str>> {
        Some(self.clone())
    }
}

impl ValidateEmail for EmailAddress {
    fn email(&self) -> Option<Cow<'_, str>> {
        Some(self.as_str().into())
    }
}

impl<T> ValidateEmail for Option<T>
where
    T: ValidateEmail,
{
    fn email(&self) -> Option<Cow<'_, str>> {
        if let Some(s) = self {
            T::email(s)
        } else {
            None
        }
    }
}

macro_rules! validate_with_deref {
    ($type:ty) => {
        impl<T> ValidateEmail for $type
        where
            T: ValidateEmail,
        {
            fn email(&self) -> Option<Cow<'_, str>> {
                T::email(self)
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

    use email_address::EmailAddress;

    use super::{EmailError, EmailOptions, ValidateEmail};

    #[test]
    fn ok() {
        let options = EmailOptions::default().without_display_text();

        assert_eq!((*"admin@localhost").validate_email(options), Ok(()));
        assert_eq!("admin@localhost".validate_email(options), Ok(()));
        assert_eq!("admin@localhost".to_owned().validate_email(options), Ok(()));
        assert_eq!(
            Cow::<str>::Borrowed("admin@localhost").validate_email(options),
            Ok(())
        );
        assert_eq!(
            Cow::<str>::Owned("admin@localhost".to_owned()).validate_email(options),
            Ok(())
        );
        assert_eq!(
            EmailAddress::new_unchecked("admin@localhost").validate_email(options),
            Ok(())
        );

        assert_eq!(None::<&str>.validate_email(options), Ok(()));
        assert_eq!(Some("admin@localhost").validate_email(options), Ok(()));

        assert_eq!((&"admin@localhost").validate_email(options), Ok(()));
        #[expect(unused_allocation)]
        {
            assert_eq!(Box::new("admin@localhost").validate_email(options), Ok(()));
        }
        assert_eq!(Arc::new("admin@localhost").validate_email(options), Ok(()));
        assert_eq!(Rc::new("admin@localhost").validate_email(options), Ok(()));

        let cell = RefCell::new("admin@localhost");
        assert_eq!(cell.borrow().validate_email(options), Ok(()));
        assert_eq!(cell.borrow_mut().validate_email(options), Ok(()));
    }

    #[test]
    fn invalid_error() {
        let options = EmailOptions::default().without_display_text();

        assert_eq!(
            (*"admin").validate_email(options),
            Err(EmailError::from(email_address::Error::MissingSeparator))
        );
        assert_eq!(
            "admin".validate_email(options),
            Err(EmailError::from(email_address::Error::MissingSeparator))
        );
        assert_eq!(
            "admin".to_owned().validate_email(options),
            Err(EmailError::from(email_address::Error::MissingSeparator))
        );
        assert_eq!(
            Cow::<str>::Borrowed("admin").validate_email(options),
            Err(EmailError::from(email_address::Error::MissingSeparator))
        );
        assert_eq!(
            Cow::<str>::Owned("admin".to_owned()).validate_email(options),
            Err(EmailError::from(email_address::Error::MissingSeparator))
        );
        assert_eq!(
            EmailAddress::new_unchecked("admin").validate_email(options),
            Err(EmailError::from(email_address::Error::MissingSeparator))
        );

        assert_eq!(
            Some("admin").validate_email(options),
            Err(EmailError::from(email_address::Error::MissingSeparator))
        );

        assert_eq!(
            (&"admin").validate_email(options),
            Err(EmailError::from(email_address::Error::MissingSeparator))
        );
        #[expect(unused_allocation)]
        {
            assert_eq!(
                Box::new("admin").validate_email(options),
                Err(EmailError::from(email_address::Error::MissingSeparator))
            );
        }
        assert_eq!(
            Arc::new("admin").validate_email(options),
            Err(EmailError::from(email_address::Error::MissingSeparator))
        );
        assert_eq!(
            Rc::new("admin").validate_email(options),
            Err(EmailError::from(email_address::Error::MissingSeparator))
        );

        let cell = RefCell::new("admin");
        assert_eq!(
            cell.borrow().validate_email(options),
            Err(EmailError::from(email_address::Error::MissingSeparator))
        );
        assert_eq!(
            cell.borrow_mut().validate_email(options),
            Err(EmailError::from(email_address::Error::MissingSeparator))
        );
    }
}
