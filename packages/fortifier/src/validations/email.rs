use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    rc::Rc,
    sync::Arc,
};

/// Email validation error.
#[derive(Debug)]
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

macro_rules! validate_type_with_deref {
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

validate_type_with_deref!(&T);
validate_type_with_deref!(Arc<T>);
validate_type_with_deref!(Box<T>);
validate_type_with_deref!(Rc<T>);
validate_type_with_deref!(Ref<'_, T>);
validate_type_with_deref!(RefMut<'_, T>);

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
