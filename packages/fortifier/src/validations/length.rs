use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    rc::Rc,
    sync::Arc,
};

#[cfg(feature = "indexmap")]
use indexmap::{IndexMap, IndexSet};

/// Length validation error.
#[derive(Debug)]
pub enum LengthError<T> {
    /// Length is not equal to the required length.
    Equal {
        /// The required length.
        equal: T,

        /// The actual length.
        length: T,
    },
    /// Length is less than the minimum length.
    Min {
        /// The minimum length.
        min: T,

        /// The actual length.
        length: T,
    },
    /// Length is more than the maximum length.
    Max {
        /// The maximum length.
        max: T,

        /// The length.
        length: T,
    },
}

/// Validate a length.
pub trait ValidateLength<T>
where
    T: PartialEq + PartialOrd,
{
    /// The length.
    fn length(&self) -> Option<T>;

    /// Validate length.
    fn validate_length(
        &self,
        equal: Option<T>,
        min: Option<T>,
        max: Option<T>,
    ) -> Result<(), LengthError<T>> {
        let Some(length) = self.length() else {
            return Ok(());
        };

        if let Some(equal) = equal {
            if length != equal {
                return Err(LengthError::Equal { equal, length });
            }
        } else {
            if let Some(min) = min
                && length < min
            {
                return Err(LengthError::Min { min, length });
            }

            if let Some(max) = max
                && length > max
            {
                return Err(LengthError::Max { max, length });
            }
        }

        Ok(())
    }
}

macro_rules! validate_type_with_deref {
    ($type:ty) => {
        impl<T> ValidateLength<usize> for $type
        where
            T: ValidateLength<usize>,
        {
            fn length(&self) -> Option<usize> {
                T::length(self)
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

macro_rules! validate_type_with_chars {
    ($type:ty) => {
        impl ValidateLength<usize> for $type {
            fn length(&self) -> Option<usize> {
                Some(self.chars().count())
            }
        }
    };
}

validate_type_with_chars!(str);
validate_type_with_chars!(&str);
validate_type_with_chars!(String);

macro_rules! validate_type_with_len {
    ($type:ty) => {
        validate_type_with_len!($type,);
    };
    ($type:ty, $( $generic:ident ),*$( , )*) => {
        impl<$( $generic ),*> ValidateLength<usize> for $type {
            fn length(&self) -> Option<usize> {
                Some(self.len())
            }
        }
    };
}

validate_type_with_len!([T], T);
validate_type_with_len!(BTreeSet<T>, T);
validate_type_with_len!(BTreeMap<K, V>, K, V);
validate_type_with_len!(HashSet<T, S>, T, S);
validate_type_with_len!(HashMap<K, V, S>, K, V, S);
validate_type_with_len!(Vec<T>, T);
validate_type_with_len!(VecDeque<T>, T);
#[cfg(feature = "indexmap")]
validate_type_with_len!(IndexSet<T>, T);
#[cfg(feature = "indexmap")]
validate_type_with_len!(IndexMap<K, V>, K, V);

impl<T> ValidateLength<usize> for Cow<'_, T>
where
    T: ToOwned + ?Sized,
    for<'a> &'a T: ValidateLength<usize>,
{
    fn length(&self) -> Option<usize> {
        self.as_ref().length()
    }
}

impl<T> ValidateLength<usize> for Option<T>
where
    T: ValidateLength<usize>,
{
    fn length(&self) -> Option<usize> {
        if let Some(s) = self {
            T::length(s)
        } else {
            None
        }
    }
}

impl<T, const N: usize> ValidateLength<usize> for [T; N] {
    fn length(&self) -> Option<usize> {
        Some(N)
    }
}
