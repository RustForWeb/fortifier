use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    error::Error,
    fmt,
    rc::Rc,
    sync::{Arc, LazyLock},
};

use constant_string::constant_string;
use regex::Regex;

/// Convert to a regular expression.
pub trait AsRegex {
    /// Convert to a regular expression.
    fn as_regex(&self) -> &Regex;
}

impl AsRegex for Regex {
    fn as_regex(&self) -> &Regex {
        self
    }
}

impl AsRegex for LazyLock<Regex> {
    fn as_regex(&self) -> &Regex {
        self
    }
}

impl<T> AsRegex for &T
where
    T: AsRegex,
{
    fn as_regex(&self) -> &Regex {
        T::as_regex(self)
    }
}

constant_string!(RegexErrorCode, REGEX_ERROR_CODE, "regex");

/// Regular expression validation error.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RegexError {
    /// The error code.
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "utoipa", schema(inline))]
    code: RegexErrorCode,

    /// A human-readable error message.
    #[cfg(feature = "message")]
    message: String,
}

impl Default for RegexError {
    fn default() -> Self {
        Self {
            code: RegexErrorCode,

            #[cfg(feature = "message")]
            message: "value does not match regular expression".to_owned(),
        }
    }
}

impl fmt::Display for RegexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Error for RegexError {}

/// Validate a regular expression.
pub trait ValidateRegex {
    /// Validate regular expression.
    fn validate_regex(&self, regex: impl AsRegex) -> Result<(), RegexError>;
}

impl ValidateRegex for str {
    fn validate_regex(&self, regex: impl AsRegex) -> Result<(), RegexError> {
        if regex.as_regex().is_match(self) {
            Ok(())
        } else {
            Err(RegexError::default())
        }
    }
}

impl ValidateRegex for &str {
    fn validate_regex(&self, regex: impl AsRegex) -> Result<(), RegexError> {
        if regex.as_regex().is_match(self) {
            Ok(())
        } else {
            Err(RegexError::default())
        }
    }
}

impl ValidateRegex for String {
    fn validate_regex(&self, regex: impl AsRegex) -> Result<(), RegexError> {
        if regex.as_regex().is_match(self) {
            Ok(())
        } else {
            Err(RegexError::default())
        }
    }
}

impl<T> ValidateRegex for Option<T>
where
    T: ValidateRegex,
{
    fn validate_regex(&self, regex: impl AsRegex) -> Result<(), RegexError> {
        if let Some(value) = self {
            T::validate_regex(value, regex)
        } else {
            Ok(())
        }
    }
}

macro_rules! validate_with_deref {
    ($type:ty) => {
        impl<T> ValidateRegex for $type
        where
            T: ValidateRegex,
        {
            fn validate_regex(&self, regex: impl AsRegex) -> Result<(), RegexError> {
                T::validate_regex(self, regex)
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

impl<T> ValidateRegex for Cow<'_, T>
where
    T: ToOwned + ?Sized,
    for<'a> &'a T: ValidateRegex,
{
    fn validate_regex(&self, regex: impl AsRegex) -> Result<(), RegexError> {
        self.as_ref().validate_regex(regex)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        borrow::Cow,
        cell::RefCell,
        rc::Rc,
        sync::{Arc, LazyLock},
    };

    use regex::Regex;

    use crate::RegexErrorCode;

    use super::{RegexError, ValidateRegex};

    static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[0-9]{4}").expect("valid regex"));

    #[test]
    fn ok() {
        assert_eq!((*"1234").validate_regex(&REGEX), Ok(()));
        assert_eq!("1234".validate_regex(&REGEX), Ok(()));
        assert_eq!("1234".to_owned().validate_regex(&REGEX), Ok(()));
        assert_eq!(Cow::<str>::Borrowed("1234").validate_regex(&REGEX), Ok(()));
        assert_eq!(
            Cow::<str>::Owned("1234".to_owned()).validate_regex(&REGEX),
            Ok(())
        );

        assert_eq!(None::<&str>.validate_regex(&REGEX), Ok(()));
        assert_eq!(Some("1234").validate_regex(&REGEX), Ok(()));

        assert_eq!((&"1234").validate_regex(&REGEX), Ok(()));
        #[expect(unused_allocation)]
        {
            assert_eq!(Box::new("1234").validate_regex(&REGEX), Ok(()));
        }
        assert_eq!(Arc::new("1234").validate_regex(&REGEX), Ok(()));
        assert_eq!(Rc::new("1234").validate_regex(&REGEX), Ok(()));

        let cell = RefCell::new("1234");
        assert_eq!(cell.borrow().validate_regex(&REGEX), Ok(()));
        assert_eq!(cell.borrow_mut().validate_regex(&REGEX), Ok(()));
    }

    #[test]
    fn no_match_error() {
        assert_eq!((*"123").validate_regex(&REGEX), Err(RegexError::default()));
        assert_eq!("123".validate_regex(&REGEX), Err(RegexError::default()));
        assert_eq!(
            "123".to_owned().validate_regex(&REGEX),
            Err(RegexError::default())
        );
        assert_eq!(
            Cow::<str>::Borrowed("123").validate_regex(&REGEX),
            Err(RegexError::default())
        );
        assert_eq!(
            Cow::<str>::Owned("123".to_owned()).validate_regex(&REGEX),
            Err(RegexError::default())
        );

        assert_eq!(
            Some("123").validate_regex(&REGEX),
            Err(RegexError::default())
        );

        assert_eq!((&"123").validate_regex(&REGEX), Err(RegexError::default()));
        #[expect(unused_allocation)]
        {
            assert_eq!(
                Box::new("123").validate_regex(&REGEX),
                Err(RegexError {
                    code: RegexErrorCode,
                    #[cfg(feature = "message")]
                    message: "value does not match regular expression".to_owned(),
                })
            );
        }
        assert_eq!(
            Arc::new("123").validate_regex(&REGEX),
            Err(RegexError::default())
        );
        assert_eq!(
            Rc::new("123").validate_regex(&REGEX),
            Err(RegexError::default())
        );

        let cell = RefCell::new("123");
        assert_eq!(
            cell.borrow().validate_regex(&REGEX),
            Err(RegexError::default())
        );
        assert_eq!(
            cell.borrow_mut().validate_regex(&REGEX),
            Err(RegexError::default())
        );
    }
}
