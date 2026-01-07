use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    error::Error,
    fmt,
    rc::Rc,
    sync::Arc,
};

pub use phonenumber::country::Id as PhoneNumberCountry;
use phonenumber::{ParseError, PhoneNumber};

use crate::error_code;

error_code!(PhoneNumberErrorCode, PHONE_NUMBER_ERROR_CODE, "phoneNumber");

/// Phone number validation error.
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
pub enum PhoneNumberError {
    /// No number error.
    NoNumber {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: PhoneNumberErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Invalid country error.
    InvalidCountryCode {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: PhoneNumberErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Too short after IDD error.
    TooShortAfterIdd {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: PhoneNumberErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Too short NSN error.
    TooShortNsn {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: PhoneNumberErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Too long error.
    TooLong {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: PhoneNumberErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Malformed integer error.
    MalformedInteger {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: PhoneNumberErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Disallowed country code error.
    DisallowedCountryCode {
        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: PhoneNumberErrorCode,

        /// Allowed country codes.
        #[cfg_attr(feature = "utoipa", schema(value_type = Vec<String>))]
        allowed: Vec<PhoneNumberCountry>,

        /// The actual country code.
        ///
        /// `None` if the country calling code did not match a country.
        #[cfg_attr(feature = "utoipa", schema(value_type = Option<String>))]
        value: Option<PhoneNumberCountry>,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
}

impl fmt::Display for PhoneNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Error for PhoneNumberError {}

impl From<ParseError> for PhoneNumberError {
    fn from(value: ParseError) -> Self {
        let code = PhoneNumberErrorCode;

        match value {
            ParseError::NoNumber => Self::NoNumber {
                code,

                #[cfg(feature = "message")]
                message: "no number".to_owned(),
            },
            ParseError::InvalidCountryCode => Self::InvalidCountryCode {
                code,

                #[cfg(feature = "message")]
                message: "invalid country code".to_owned(),
            },
            ParseError::TooShortAfterIdd => Self::TooShortAfterIdd {
                code,

                #[cfg(feature = "message")]
                message: "too short after IDD".to_owned(),
            },
            ParseError::TooShortNsn => Self::TooShortNsn {
                code,

                #[cfg(feature = "message")]
                message: "too short NSN".to_owned(),
            },
            ParseError::TooLong => Self::TooLong {
                code,

                #[cfg(feature = "message")]
                message: "too long".to_owned(),
            },
            ParseError::MalformedInteger(_) => Self::MalformedInteger {
                code,

                #[cfg(feature = "message")]
                message: "malformed integer".to_owned(),
            },
        }
    }
}

/// Validate a phone number.
pub trait ValidatePhoneNumber {
    /// The phone number.
    fn phone_number(&self) -> Option<Cow<'_, str>>;

    /// Validate phone number.
    fn validate_phone_number(
        &self,
        default_country: Option<PhoneNumberCountry>,
        allowed_countries: Option<Vec<PhoneNumberCountry>>,
    ) -> Result<(), PhoneNumberError> {
        let Some(phone_number) = self.phone_number() else {
            return Ok(());
        };

        let phone_number =
            phonenumber::parse(default_country, &phone_number).map_err(PhoneNumberError::from)?;

        if let Some(allowed_countries) = allowed_countries {
            match phone_number.country().id() {
                Some(country) => {
                    if !allowed_countries.contains(&country) {
                        #[cfg(feature = "message")]
                        let message = format!(
                            "country code `{}` is not allowed, must be one of `{}`",
                            country.as_ref(),
                            allowed_countries
                                .iter()
                                .map(AsRef::as_ref)
                                .collect::<Vec<_>>()
                                .join(", ")
                        );

                        return Err(PhoneNumberError::DisallowedCountryCode {
                            allowed: allowed_countries,
                            value: Some(country),
                            code: PhoneNumberErrorCode,
                            #[cfg(feature = "message")]
                            message,
                        });
                    }
                }
                None => {
                    #[cfg(feature = "message")]
                    let message = format!(
                        "unknown country code, must be one of `{}`",
                        allowed_countries
                            .iter()
                            .map(AsRef::as_ref)
                            .collect::<Vec<_>>()
                            .join(", ")
                    );

                    return Err(PhoneNumberError::DisallowedCountryCode {
                        allowed: allowed_countries,
                        value: None,
                        code: PhoneNumberErrorCode,
                        #[cfg(feature = "message")]
                        message,
                    });
                }
            }
        }

        Ok(())
    }
}

impl ValidatePhoneNumber for str {
    fn phone_number(&self) -> Option<Cow<'_, str>> {
        Some(self.into())
    }
}

impl ValidatePhoneNumber for &str {
    fn phone_number(&self) -> Option<Cow<'_, str>> {
        Some((*self).into())
    }
}

impl ValidatePhoneNumber for String {
    fn phone_number(&self) -> Option<Cow<'_, str>> {
        Some(self.into())
    }
}

impl ValidatePhoneNumber for Cow<'_, str> {
    fn phone_number(&self) -> Option<Cow<'_, str>> {
        Some(self.clone())
    }
}

impl ValidatePhoneNumber for PhoneNumber {
    fn phone_number(&self) -> Option<Cow<'_, str>> {
        Some(self.to_string().into())
    }
}

impl<T> ValidatePhoneNumber for Option<T>
where
    T: ValidatePhoneNumber,
{
    fn phone_number(&self) -> Option<Cow<'_, str>> {
        if let Some(s) = self {
            T::phone_number(s)
        } else {
            None
        }
    }
}

macro_rules! validate_with_deref {
    ($type:ty) => {
        impl<T> ValidatePhoneNumber for $type
        where
            T: ValidatePhoneNumber,
        {
            fn phone_number(&self) -> Option<Cow<'_, str>> {
                T::phone_number(self)
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
    use std::{borrow::Cow, cell::RefCell, rc::Rc, str::FromStr, sync::Arc};

    use phonenumber::{ParseError, PhoneNumber};

    use crate::PhoneNumberErrorCode;

    use super::{PhoneNumberCountry, PhoneNumberError, ValidatePhoneNumber};

    #[test]
    fn ok() {
        assert_eq!(
            (*"+44 20 7946 0000").validate_phone_number(None, None),
            Ok(())
        );
        assert_eq!("+44 20 7946 0000".validate_phone_number(None, None), Ok(()));
        assert_eq!(
            "+44 20 7946 0000"
                .to_owned()
                .validate_phone_number(None, None),
            Ok(())
        );
        assert_eq!(
            Cow::<str>::Borrowed("+44 20 7946 0000").validate_phone_number(None, None),
            Ok(())
        );
        assert_eq!(
            Cow::<str>::Owned("+44 20 7946 0000".to_owned()).validate_phone_number(None, None),
            Ok(())
        );
        assert_eq!(
            PhoneNumber::from_str("+44 20 7946 0000")
                .expect("valid phone number")
                .validate_phone_number(None, None),
            Ok(())
        );

        assert_eq!(None::<&str>.validate_phone_number(None, None), Ok(()));
        assert_eq!(
            Some("+44 20 7946 0000").validate_phone_number(None, None),
            Ok(())
        );

        assert_eq!(
            (&"+44 20 7946 0000").validate_phone_number(None, None),
            Ok(())
        );
        #[expect(unused_allocation)]
        {
            assert_eq!(
                Box::new("+44 20 7946 0000").validate_phone_number(None, None),
                Ok(())
            );
        }
        assert_eq!(
            Arc::new("+44 20 7946 0000").validate_phone_number(None, None),
            Ok(())
        );
        assert_eq!(
            Rc::new("+44 20 7946 0000").validate_phone_number(None, None),
            Ok(())
        );

        let cell = RefCell::new("+44 20 7946 0000");
        assert_eq!(cell.borrow().validate_phone_number(None, None), Ok(()));
        assert_eq!(cell.borrow_mut().validate_phone_number(None, None), Ok(()));
    }

    #[test]
    fn invalid_error() {
        assert_eq!(
            (*"+44").validate_phone_number(None, None),
            Err(PhoneNumberError::from(ParseError::TooShortNsn))
        );
        assert_eq!(
            "+44".validate_phone_number(None, None),
            Err(PhoneNumberError::from(ParseError::TooShortNsn))
        );
        assert_eq!(
            "+44".to_owned().validate_phone_number(None, None),
            Err(PhoneNumberError::from(ParseError::TooShortNsn))
        );
        assert_eq!(
            Cow::<str>::Borrowed("+44").validate_phone_number(None, None),
            Err(PhoneNumberError::from(ParseError::TooShortNsn))
        );
        assert_eq!(
            Cow::<str>::Owned("+44".to_owned()).validate_phone_number(None, None),
            Err(PhoneNumberError::from(ParseError::TooShortNsn))
        );

        assert_eq!(
            Some("+44").validate_phone_number(None, None),
            Err(PhoneNumberError::from(ParseError::TooShortNsn))
        );

        assert_eq!(
            (&"+44").validate_phone_number(None, None),
            Err(PhoneNumberError::from(ParseError::TooShortNsn))
        );
        #[expect(unused_allocation)]
        {
            assert_eq!(
                Box::new("+44").validate_phone_number(None, None),
                Err(PhoneNumberError::from(ParseError::TooShortNsn))
            );
        }
        assert_eq!(
            Arc::new("+44").validate_phone_number(None, None),
            Err(PhoneNumberError::from(ParseError::TooShortNsn))
        );
        assert_eq!(
            Rc::new("+44").validate_phone_number(None, None),
            Err(PhoneNumberError::from(ParseError::TooShortNsn))
        );

        let cell = RefCell::new("+44");
        assert_eq!(
            cell.borrow().validate_phone_number(None, None),
            Err(PhoneNumberError::from(ParseError::TooShortNsn))
        );
        assert_eq!(
            cell.borrow_mut().validate_phone_number(None, None),
            Err(PhoneNumberError::from(ParseError::TooShortNsn))
        );
    }

    #[test]
    fn disallowed_country_code_error() {
        assert_eq!(
            "+44 20 7946 0000".validate_phone_number(None, Some(vec![PhoneNumberCountry::NL])),
            Err(PhoneNumberError::DisallowedCountryCode {
                allowed: vec![PhoneNumberCountry::NL],
                value: Some(PhoneNumberCountry::GB),
                code: PhoneNumberErrorCode,
                #[cfg(feature = "message")]
                message: "country code `GB` is not allowed, must be one of `NL`".to_owned()
            })
        );
    }
}
