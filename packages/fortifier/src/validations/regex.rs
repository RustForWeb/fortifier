use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    rc::Rc,
    sync::{Arc, LazyLock},
};

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

/// Regular expression validation error.
#[derive(Debug, Eq, PartialEq)]
pub enum RegexError {
    /// Regular expression does not match.
    NoMatch,
}

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
            Err(RegexError::NoMatch)
        }
    }
}

impl ValidateRegex for &str {
    fn validate_regex(&self, regex: impl AsRegex) -> Result<(), RegexError> {
        if regex.as_regex().is_match(self) {
            Ok(())
        } else {
            Err(RegexError::NoMatch)
        }
    }
}

impl ValidateRegex for String {
    fn validate_regex(&self, regex: impl AsRegex) -> Result<(), RegexError> {
        if regex.as_regex().is_match(self) {
            Ok(())
        } else {
            Err(RegexError::NoMatch)
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

    use super::{RegexError, ValidateRegex};

    static REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"[0-9]{4}").expect("Regex should be valid."));

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
        assert_eq!((*"123").validate_regex(&REGEX), Err(RegexError::NoMatch));
        assert_eq!("123".validate_regex(&REGEX), Err(RegexError::NoMatch));
        assert_eq!(
            "123".to_owned().validate_regex(&REGEX),
            Err(RegexError::NoMatch)
        );
        assert_eq!(
            Cow::<str>::Borrowed("123").validate_regex(&REGEX),
            Err(RegexError::NoMatch)
        );
        assert_eq!(
            Cow::<str>::Owned("123".to_owned()).validate_regex(&REGEX),
            Err(RegexError::NoMatch)
        );

        assert_eq!(Some("123").validate_regex(&REGEX), Err(RegexError::NoMatch));

        assert_eq!((&"123").validate_regex(&REGEX), Err(RegexError::NoMatch));
        #[expect(unused_allocation)]
        {
            assert_eq!(
                Box::new("123").validate_regex(&REGEX),
                Err(RegexError::NoMatch)
            );
        }
        assert_eq!(
            Arc::new("123").validate_regex(&REGEX),
            Err(RegexError::NoMatch)
        );
        assert_eq!(
            Rc::new("123").validate_regex(&REGEX),
            Err(RegexError::NoMatch)
        );

        let cell = RefCell::new("123");
        assert_eq!(
            cell.borrow().validate_regex(&REGEX),
            Err(RegexError::NoMatch)
        );
        assert_eq!(
            cell.borrow_mut().validate_regex(&REGEX),
            Err(RegexError::NoMatch)
        );
    }
}
