use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    error::Error,
    fmt,
    rc::Rc,
    sync::Arc,
};

use email_address::EmailAddress;
pub use email_address::Options as EmailAddressOptions;

use crate::error_code;

error_code!(
    EmailAddressErrorCode,
    EMAIL_ADDRESS_ERROR_CODE,
    "emailAddress"
);

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
pub enum EmailAddressError {
    /// Invalid character error.
    InvalidCharacter {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Missing separator error.
    MissingSeparator {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Locale part empty error.
    LocalPartEmpty {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Local part too long error.
    LocalPartTooLong {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Domain empty error.
    DomainEmpty {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Domain too long error.
    DomainTooLong {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Subdomain empty error.
    SubDomainEmpty {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Subdomain too long error.
    SubDomainTooLong {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Domain too few error.
    DomainTooFew {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Domain invalid separator error.
    DomainInvalidSeparator {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Unbalanced quotes error.
    UnbalancedQuotes {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Invalid comment error.
    InvalidComment {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Invalid IP Address error.
    InvalidIPAddress {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Unsupported domain literal error.
    UnsupportedDomainLiteral {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Unsupported display name error.
    UnsupportedDisplayName {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Missing display name error.
    MissingDisplayName {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Missing end bracket error.
    MissingEndBracket {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: EmailAddressErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
}

impl fmt::Display for EmailAddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Error for EmailAddressError {}

impl From<email_address::Error> for EmailAddressError {
    fn from(value: email_address::Error) -> Self {
        let code = EmailAddressErrorCode;

        match value {
            email_address::Error::InvalidCharacter => Self::InvalidCharacter {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::MissingSeparator => Self::MissingSeparator {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::LocalPartEmpty => Self::LocalPartEmpty {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::LocalPartTooLong => Self::LocalPartTooLong {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::DomainEmpty => Self::DomainEmpty {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::DomainTooLong => Self::DomainTooLong {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::SubDomainEmpty => Self::SubDomainEmpty {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::SubDomainTooLong => Self::SubDomainTooLong {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::DomainTooFew => Self::DomainTooFew {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::DomainInvalidSeparator => Self::DomainInvalidSeparator {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::UnbalancedQuotes => Self::UnbalancedQuotes {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::InvalidComment => Self::InvalidComment {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::InvalidIPAddress => Self::InvalidIPAddress {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::UnsupportedDomainLiteral => Self::UnsupportedDomainLiteral {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::UnsupportedDisplayName => Self::UnsupportedDisplayName {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::MissingDisplayName => Self::MissingDisplayName {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
            email_address::Error::MissingEndBracket => Self::MissingEndBracket {
                code,

                #[cfg(feature = "message")]
                message: "".to_owned(),
            },
        }
    }
}

/// Validate an email address.
pub trait ValidateEmailAddress {
    /// The email address.
    fn email_address(&self) -> Option<Cow<'_, str>>;

    /// Validate email address.
    fn validate_email_address(
        &self,
        options: EmailAddressOptions,
    ) -> Result<(), EmailAddressError> {
        let Some(email_address) = self.email_address() else {
            return Ok(());
        };

        EmailAddress::parse_with_options(&email_address, options)
            .map_err(EmailAddressError::from)?;

        Ok(())
    }
}

impl ValidateEmailAddress for str {
    fn email_address(&self) -> Option<Cow<'_, str>> {
        Some(self.into())
    }
}

impl ValidateEmailAddress for &str {
    fn email_address(&self) -> Option<Cow<'_, str>> {
        Some((*self).into())
    }
}

impl ValidateEmailAddress for String {
    fn email_address(&self) -> Option<Cow<'_, str>> {
        Some(self.into())
    }
}

impl ValidateEmailAddress for Cow<'_, str> {
    fn email_address(&self) -> Option<Cow<'_, str>> {
        Some(self.clone())
    }
}

impl ValidateEmailAddress for EmailAddress {
    fn email_address(&self) -> Option<Cow<'_, str>> {
        Some(self.as_str().into())
    }
}

impl<T> ValidateEmailAddress for Option<T>
where
    T: ValidateEmailAddress,
{
    fn email_address(&self) -> Option<Cow<'_, str>> {
        if let Some(s) = self {
            T::email_address(s)
        } else {
            None
        }
    }
}

macro_rules! validate_with_deref {
    ($type:ty) => {
        impl<T> ValidateEmailAddress for $type
        where
            T: ValidateEmailAddress,
        {
            fn email_address(&self) -> Option<Cow<'_, str>> {
                T::email_address(self)
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

    use super::{EmailAddressError, EmailAddressOptions, ValidateEmailAddress};

    #[test]
    fn ok() {
        let options = EmailAddressOptions::default().without_display_text();

        assert_eq!((*"admin@localhost").validate_email_address(options), Ok(()));
        assert_eq!("admin@localhost".validate_email_address(options), Ok(()));
        assert_eq!(
            "admin@localhost".to_owned().validate_email_address(options),
            Ok(())
        );
        assert_eq!(
            Cow::<str>::Borrowed("admin@localhost").validate_email_address(options),
            Ok(())
        );
        assert_eq!(
            Cow::<str>::Owned("admin@localhost".to_owned()).validate_email_address(options),
            Ok(())
        );
        assert_eq!(
            EmailAddress::new_unchecked("admin@localhost").validate_email_address(options),
            Ok(())
        );

        assert_eq!(None::<&str>.validate_email_address(options), Ok(()));
        assert_eq!(
            Some("admin@localhost").validate_email_address(options),
            Ok(())
        );

        assert_eq!((&"admin@localhost").validate_email_address(options), Ok(()));
        #[expect(unused_allocation)]
        {
            assert_eq!(
                Box::new("admin@localhost").validate_email_address(options),
                Ok(())
            );
        }
        assert_eq!(
            Arc::new("admin@localhost").validate_email_address(options),
            Ok(())
        );
        assert_eq!(
            Rc::new("admin@localhost").validate_email_address(options),
            Ok(())
        );

        let cell = RefCell::new("admin@localhost");
        assert_eq!(cell.borrow().validate_email_address(options), Ok(()));
        assert_eq!(cell.borrow_mut().validate_email_address(options), Ok(()));
    }

    #[test]
    fn invalid_error() {
        let options = EmailAddressOptions::default().without_display_text();

        assert_eq!(
            (*"admin").validate_email_address(options),
            Err(EmailAddressError::from(
                email_address::Error::MissingSeparator
            ))
        );
        assert_eq!(
            "admin".validate_email_address(options),
            Err(EmailAddressError::from(
                email_address::Error::MissingSeparator
            ))
        );
        assert_eq!(
            "admin".to_owned().validate_email_address(options),
            Err(EmailAddressError::from(
                email_address::Error::MissingSeparator
            ))
        );
        assert_eq!(
            Cow::<str>::Borrowed("admin").validate_email_address(options),
            Err(EmailAddressError::from(
                email_address::Error::MissingSeparator
            ))
        );
        assert_eq!(
            Cow::<str>::Owned("admin".to_owned()).validate_email_address(options),
            Err(EmailAddressError::from(
                email_address::Error::MissingSeparator
            ))
        );
        assert_eq!(
            EmailAddress::new_unchecked("admin").validate_email_address(options),
            Err(EmailAddressError::from(
                email_address::Error::MissingSeparator
            ))
        );

        assert_eq!(
            Some("admin").validate_email_address(options),
            Err(EmailAddressError::from(
                email_address::Error::MissingSeparator
            ))
        );

        assert_eq!(
            (&"admin").validate_email_address(options),
            Err(EmailAddressError::from(
                email_address::Error::MissingSeparator
            ))
        );
        #[expect(unused_allocation)]
        {
            assert_eq!(
                Box::new("admin").validate_email_address(options),
                Err(EmailAddressError::from(
                    email_address::Error::MissingSeparator
                ))
            );
        }
        assert_eq!(
            Arc::new("admin").validate_email_address(options),
            Err(EmailAddressError::from(
                email_address::Error::MissingSeparator
            ))
        );
        assert_eq!(
            Rc::new("admin").validate_email_address(options),
            Err(EmailAddressError::from(
                email_address::Error::MissingSeparator
            ))
        );

        let cell = RefCell::new("admin");
        assert_eq!(
            cell.borrow().validate_email_address(options),
            Err(EmailAddressError::from(
                email_address::Error::MissingSeparator
            ))
        );
        assert_eq!(
            cell.borrow_mut().validate_email_address(options),
            Err(EmailAddressError::from(
                email_address::Error::MissingSeparator
            ))
        );
    }
}
