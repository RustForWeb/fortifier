use std::{
    borrow::Cow,
    cell::{Ref, RefMut},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    fmt::Display,
    rc::Rc,
    sync::Arc,
};

#[cfg(feature = "indexmap")]
use indexmap::{IndexMap, IndexSet};

use crate::error_code;

error_code!(LengthErrorCode, "length");

/// Length validation error.
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
pub enum LengthError<T> {
    /// Length is not equal to the required length.
    Equal {
        /// The required length.
        equal: T,

        /// The actual length.
        length: T,

        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: LengthErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Length is less than the minimum length.
    Min {
        /// The minimum length.
        min: T,

        /// The actual length.
        length: T,

        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: LengthErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Length is more than the maximum length.
    Max {
        /// The maximum length.
        max: T,

        /// The length.
        length: T,

        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: LengthErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
}

/// Validate a length.
pub trait ValidateLength<T>
where
    T: Display + PartialEq + PartialOrd,
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
                #[cfg(feature = "message")]
                let message = format!("length {length} is not equal to required length {equal}");

                return Err(LengthError::Equal {
                    equal,
                    length,
                    code: LengthErrorCode,
                    #[cfg(feature = "message")]
                    message,
                });
            }
        } else {
            if let Some(min) = min
                && length < min
            {
                #[cfg(feature = "message")]
                let message = format!("length {length} is less than minimum length {min}");

                return Err(LengthError::Min {
                    min,
                    length,
                    code: LengthErrorCode,
                    #[cfg(feature = "message")]
                    message,
                });
            }

            if let Some(max) = max
                && length > max
            {
                #[cfg(feature = "message")]
                let message = format!("length {length} is greater than maximum length {max}");

                return Err(LengthError::Max {
                    max,
                    length,
                    code: LengthErrorCode,
                    #[cfg(feature = "message")]
                    message,
                });
            }
        }

        Ok(())
    }
}

macro_rules! validate_with_chars {
    ($type:ty) => {
        impl ValidateLength<usize> for $type {
            fn length(&self) -> Option<usize> {
                Some(self.chars().count())
            }
        }
    };
}

validate_with_chars!(str);
validate_with_chars!(&str);
validate_with_chars!(String);

impl<T, const N: usize> ValidateLength<usize> for [T; N] {
    fn length(&self) -> Option<usize> {
        Some(N)
    }
}

macro_rules! validate_with_len {
    ($type:ty) => {
        validate_with_len!($type,);
    };
    ($type:ty, $( $generic:ident ),*$( , )*) => {
        impl<$( $generic ),*> ValidateLength<usize> for $type {
            fn length(&self) -> Option<usize> {
                Some(self.len())
            }
        }
    };
}

validate_with_len!([T], T);
validate_with_len!(&[T], T);
validate_with_len!(BTreeSet<T>, T);
validate_with_len!(BTreeMap<K, V>, K, V);
validate_with_len!(HashSet<T, S>, T, S);
validate_with_len!(HashMap<K, V, S>, K, V, S);
validate_with_len!(LinkedList<T>, T);
validate_with_len!(Vec<T>, T);
validate_with_len!(VecDeque<T>, T);
#[cfg(feature = "indexmap")]
validate_with_len!(IndexSet<T>, T);
#[cfg(feature = "indexmap")]
validate_with_len!(IndexMap<K, V>, K, V);

impl<L, T> ValidateLength<L> for Option<T>
where
    L: Display + PartialEq + PartialOrd,
    T: ValidateLength<L>,
{
    fn length(&self) -> Option<L> {
        if let Some(s) = self {
            T::length(s)
        } else {
            None
        }
    }
}

macro_rules! validate_with_deref {
    ($type:ty) => {
        impl<L, T> ValidateLength<L> for $type
        where
            L: Display + PartialEq + PartialOrd,
            T: ValidateLength<L>,
        {
            fn length(&self) -> Option<L> {
                T::length(self)
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

impl<L, T> ValidateLength<L> for Cow<'_, T>
where
    L: Display + PartialEq + PartialOrd,
    T: ToOwned + ?Sized,
    for<'a> &'a T: ValidateLength<L>,
{
    fn length(&self) -> Option<L> {
        self.as_ref().length()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        borrow::Cow,
        cell::RefCell,
        collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
        rc::Rc,
        sync::Arc,
    };

    #[cfg(feature = "indexmap")]
    use indexmap::{IndexMap, IndexSet};

    use crate::LengthErrorCode;

    use super::{LengthError, ValidateLength};

    #[test]
    fn ok() {
        assert_eq!((*"a").validate_length(Some(1), None, None), Ok(()));
        assert_eq!("a".validate_length(Some(1), None, None), Ok(()));
        assert_eq!("a".to_owned().validate_length(Some(1), None, None), Ok(()));
        assert_eq!(
            Cow::<str>::Borrowed("a").validate_length(Some(1), None, None),
            Ok(())
        );
        assert_eq!(
            Cow::<str>::Owned("a".to_owned()).validate_length(Some(1), None, None),
            Ok(())
        );

        assert_eq!(None::<&str>.validate_length(Some(1), None, None), Ok(()));
        assert_eq!(Some("a").validate_length(Some(1), None, None), Ok(()));

        assert_eq!([""; 1].validate_length(Some(1), None, None), Ok(()));
        assert_eq!([""].validate_length(Some(1), None, None), Ok(()));
        assert_eq!(
            BTreeSet::from([""]).validate_length(Some(1), None, None),
            Ok(())
        );
        assert_eq!(
            BTreeMap::from([("", "")]).validate_length(Some(1), None, None),
            Ok(())
        );
        assert_eq!(
            HashSet::from([""]).validate_length(Some(1), None, None),
            Ok(())
        );
        assert_eq!(
            HashMap::from([("", "")]).validate_length(Some(1), None, None),
            Ok(())
        );
        assert_eq!(vec![""].validate_length(Some(1), None, None), Ok(()));
        assert_eq!(
            VecDeque::from([""]).validate_length(Some(1), None, None),
            Ok(())
        );

        #[cfg(feature = "indexmap")]
        {
            assert_eq!(
                IndexSet::from([""]).validate_length(Some(1), None, None),
                Ok(())
            );
            assert_eq!(
                IndexMap::from([("", "")]).validate_length(Some(1), None, None),
                Ok(())
            );
        }

        assert_eq!((&"a").validate_length(Some(1), None, None), Ok(()));
        #[expect(unused_allocation)]
        {
            assert_eq!(Box::new("a").validate_length(Some(1), None, None), Ok(()));
        }
        assert_eq!(Arc::new("a").validate_length(Some(1), None, None), Ok(()));
        assert_eq!(Rc::new("a").validate_length(Some(1), None, None), Ok(()));

        let cell = RefCell::new("a");
        assert_eq!(cell.borrow().validate_length(Some(1), None, None), Ok(()));
        assert_eq!(
            cell.borrow_mut().validate_length(Some(1), None, None),
            Ok(())
        );
    }

    #[test]
    fn equal_error() {
        assert_eq!(
            (*"a").validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            "a".validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            "a".to_owned().validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            Cow::<str>::Borrowed("a").validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            Cow::<str>::Owned("a".to_owned()).validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );

        assert_eq!(
            Some("a").validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );

        assert_eq!(
            [""; 1].validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            [""].validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            BTreeSet::from([""]).validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            BTreeMap::from([("", "")]).validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            HashSet::from([""]).validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            HashMap::from([("", "")]).validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            vec![""].validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            VecDeque::from([""]).validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );

        #[cfg(feature = "indexmap")]
        {
            assert_eq!(
                IndexSet::from([""]).validate_length(Some(2), None, None),
                Err(LengthError::Equal {
                    equal: 2,
                    length: 1,
                    code: LengthErrorCode,
                    #[cfg(feature = "message")]
                    message: "length 1 is not equal to required length 2".to_owned(),
                })
            );
            assert_eq!(
                IndexMap::from([("", "")]).validate_length(Some(2), None, None),
                Err(LengthError::Equal {
                    equal: 2,
                    length: 1,
                    code: LengthErrorCode,
                    #[cfg(feature = "message")]
                    message: "length 1 is not equal to required length 2".to_owned(),
                })
            );
        }

        assert_eq!(
            (&"a").validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        #[expect(unused_allocation)]
        {
            assert_eq!(
                Box::new("a").validate_length(Some(2), None, None),
                Err(LengthError::Equal {
                    equal: 2,
                    length: 1,
                    code: LengthErrorCode,
                    #[cfg(feature = "message")]
                    message: "length 1 is not equal to required length 2".to_owned(),
                })
            );
        }
        assert_eq!(
            Arc::new("a").validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            Rc::new("a").validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );

        let cell = RefCell::new("a");
        assert_eq!(
            cell.borrow().validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
        assert_eq!(
            cell.borrow_mut().validate_length(Some(2), None, None),
            Err(LengthError::Equal {
                equal: 2,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is not equal to required length 2".to_owned(),
            })
        );
    }

    #[test]
    fn min_error() {
        assert_eq!(
            (*"a").validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            "a".validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            "a".to_owned().validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            Cow::<str>::Borrowed("a").validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            Cow::<str>::Owned("a".to_owned()).validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );

        assert_eq!(
            Some("a").validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );

        assert_eq!(
            [""; 1].validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            [""].validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            BTreeSet::from([""]).validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            BTreeMap::from([("", "")]).validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            HashSet::from([""]).validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            HashMap::from([("", "")]).validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            vec![""].validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            VecDeque::from([""]).validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );

        #[cfg(feature = "indexmap")]
        {
            assert_eq!(
                IndexSet::from([""]).validate_length(None, Some(3), None),
                Err(LengthError::Min {
                    min: 3,
                    length: 1,
                    code: LengthErrorCode,
                    #[cfg(feature = "message")]
                    message: "length 1 is less than minimum length 3".to_owned(),
                })
            );
            assert_eq!(
                IndexMap::from([("", "")]).validate_length(None, Some(3), None),
                Err(LengthError::Min {
                    min: 3,
                    length: 1,
                    code: LengthErrorCode,
                    #[cfg(feature = "message")]
                    message: "length 1 is less than minimum length 3".to_owned(),
                })
            );
        }

        assert_eq!(
            (&"a").validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        #[expect(unused_allocation)]
        {
            assert_eq!(
                Box::new("a").validate_length(None, Some(3), None),
                Err(LengthError::Min {
                    min: 3,
                    length: 1,
                    code: LengthErrorCode,
                    #[cfg(feature = "message")]
                    message: "length 1 is less than minimum length 3".to_owned(),
                })
            );
        }
        assert_eq!(
            Arc::new("a").validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            Rc::new("a").validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );

        let cell = RefCell::new("a");
        assert_eq!(
            cell.borrow().validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
        assert_eq!(
            cell.borrow_mut().validate_length(None, Some(3), None),
            Err(LengthError::Min {
                min: 3,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is less than minimum length 3".to_owned(),
            })
        );
    }

    #[test]
    fn max_error() {
        assert_eq!(
            (*"a").validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            "a".validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            "a".to_owned().validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            Cow::<str>::Borrowed("a").validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            Cow::<str>::Owned("a".to_owned()).validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );

        assert_eq!(
            Some("a").validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );

        assert_eq!(
            [""; 1].validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            [""].validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            BTreeSet::from([""]).validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            BTreeMap::from([("", "")]).validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            HashSet::from([""]).validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            HashMap::from([("", "")]).validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            vec![""].validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            VecDeque::from([""]).validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );

        #[cfg(feature = "indexmap")]
        {
            assert_eq!(
                IndexSet::from([""]).validate_length(None, None, Some(0)),
                Err(LengthError::Max {
                    max: 0,
                    length: 1,
                    code: LengthErrorCode,
                    #[cfg(feature = "message")]
                    message: "length 1 is greater than maximum length 0".to_owned(),
                })
            );
            assert_eq!(
                IndexMap::from([("", "")]).validate_length(None, None, Some(0)),
                Err(LengthError::Max {
                    max: 0,
                    length: 1,
                    code: LengthErrorCode,
                    #[cfg(feature = "message")]
                    message: "length 1 is greater than maximum length 0".to_owned(),
                })
            );
        }

        assert_eq!(
            (&"a").validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        #[expect(unused_allocation)]
        {
            assert_eq!(
                Box::new("a").validate_length(None, None, Some(0)),
                Err(LengthError::Max {
                    max: 0,
                    length: 1,
                    code: LengthErrorCode,
                    #[cfg(feature = "message")]
                    message: "length 1 is greater than maximum length 0".to_owned(),
                })
            );
        }
        assert_eq!(
            Arc::new("a").validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            Rc::new("a").validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );

        let cell = RefCell::new("a");
        assert_eq!(
            cell.borrow().validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
        assert_eq!(
            cell.borrow_mut().validate_length(None, None, Some(0)),
            Err(LengthError::Max {
                max: 0,
                length: 1,
                code: LengthErrorCode,
                #[cfg(feature = "message")]
                message: "length 1 is greater than maximum length 0".to_owned(),
            })
        );
    }
}
