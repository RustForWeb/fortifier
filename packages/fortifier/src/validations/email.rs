use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    rc::Rc,
    sync::Arc,
};

/// Email validation error.
#[derive(Debug, Eq, PartialEq)]
pub enum EmailError {
    /// Invalid email address.
    Invalid,
}

/// Validate an email address.
pub trait ValidateEmail {
    /// The email address.
    fn email(&self) -> Option<Cow<'_, str>>;

    /// Validate email address.
    fn validate_email(&self) -> Result<(), EmailError> {
        let Some(email) = self.email() else {
            return Ok(());
        };

        if email.is_empty() || !email.contains("@") {
            return Err(EmailError::Invalid);
        }

        // TODO

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

    use super::{EmailError, ValidateEmail};

    #[test]
    fn ok() {
        assert_eq!((*"admin@localhost").validate_email(), Ok(()));
        assert_eq!("admin@localhost".validate_email(), Ok(()));
        assert_eq!("admin@localhost".to_owned().validate_email(), Ok(()));
        assert_eq!(
            Cow::<str>::Borrowed("admin@localhost").validate_email(),
            Ok(())
        );
        assert_eq!(
            Cow::<str>::Owned("admin@localhost".to_owned()).validate_email(),
            Ok(())
        );

        assert_eq!(None::<&str>.validate_email(), Ok(()));
        assert_eq!(Some("admin@localhost").validate_email(), Ok(()));

        assert_eq!((&"admin@localhost").validate_email(), Ok(()));
        #[expect(unused_allocation)]
        {
            assert_eq!(Box::new("admin@localhost").validate_email(), Ok(()));
        }
        assert_eq!(Arc::new("admin@localhost").validate_email(), Ok(()));
        assert_eq!(Rc::new("admin@localhost").validate_email(), Ok(()));

        let cell = RefCell::new("admin@localhost");
        assert_eq!(cell.borrow().validate_email(), Ok(()));
        assert_eq!(cell.borrow_mut().validate_email(), Ok(()));
    }

    #[test]
    fn invalid_error() {
        assert_eq!((*"admin").validate_email(), Err(EmailError::Invalid));
        assert_eq!("admin".validate_email(), Err(EmailError::Invalid));
        assert_eq!(
            "admin".to_owned().validate_email(),
            Err(EmailError::Invalid)
        );
        assert_eq!(
            Cow::<str>::Borrowed("admin").validate_email(),
            Err(EmailError::Invalid)
        );
        assert_eq!(
            Cow::<str>::Owned("admin".to_owned()).validate_email(),
            Err(EmailError::Invalid)
        );

        assert_eq!(Some("admin").validate_email(), Err(EmailError::Invalid));

        assert_eq!((&"admin").validate_email(), Err(EmailError::Invalid));
        #[expect(unused_allocation)]
        {
            assert_eq!(Box::new("admin").validate_email(), Err(EmailError::Invalid));
        }
        assert_eq!(Arc::new("admin").validate_email(), Err(EmailError::Invalid));
        assert_eq!(Rc::new("admin").validate_email(), Err(EmailError::Invalid));

        let cell = RefCell::new("admin");
        assert_eq!(cell.borrow().validate_email(), Err(EmailError::Invalid));
        assert_eq!(cell.borrow_mut().validate_email(), Err(EmailError::Invalid));
    }
}
