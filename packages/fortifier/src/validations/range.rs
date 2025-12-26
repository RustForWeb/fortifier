use std::{
    cell::{Ref, RefMut},
    error::Error,
    fmt::{self, Debug, Display},
    rc::Rc,
    sync::Arc,
};

use crate::error_code;

error_code!(RangeErrorCode, "range");

/// Range validation error.
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
pub enum RangeError<T> {
    /// Value is less than the minimum value.
    Min {
        /// The minimum value.
        min: T,

        /// The actual value.
        value: T,

        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: RangeErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Value is more than the maximum value.
    Max {
        /// The maximum value.
        max: T,

        /// The value.
        value: T,

        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: RangeErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Value is less than or equal to the exclusive minimum value.
    ExclusiveMin {
        /// The minimum value.
        exclusive_min: T,

        /// The actual value.
        value: T,

        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: RangeErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
    /// Value is greater than or equal to the exclusive maximum value.
    ExclusiveMax {
        /// The maximum value.
        exclusive_max: T,

        /// The value.
        value: T,

        /// The error code.
        #[cfg_attr(feature = "serde", serde(default))]
        code: RangeErrorCode,

        /// A human-readable error message.
        #[cfg(feature = "message")]
        message: String,
    },
}

impl<T: Debug> fmt::Display for RangeError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl<T: Debug> Error for RangeError<T> {}

/// Validate a range.
pub trait ValidateRange<T>
where
    T: Display + PartialEq + PartialOrd,
{
    /// The value.
    fn range_value(&self) -> Option<T>;

    /// Validate range.
    fn validate_range(
        &self,
        min: Option<T>,
        max: Option<T>,
        exclusive_min: Option<T>,
        exclusive_max: Option<T>,
    ) -> Result<(), RangeError<T>> {
        let Some(value) = self.range_value() else {
            return Ok(());
        };

        if let Some(min) = min
            && value < min
        {
            #[cfg(feature = "message")]
            let message = format!("value {value} is less than minimum value {min}");

            return Err(RangeError::Min {
                min,
                value,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message,
            });
        }

        if let Some(max) = max
            && value > max
        {
            #[cfg(feature = "message")]
            let message = format!("value {value} is greater than maximum value {max}");

            return Err(RangeError::Max {
                max,
                value,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message,
            });
        }

        if let Some(exclusive_min) = exclusive_min
            && value <= exclusive_min
        {
            #[cfg(feature = "message")]
            let message = format!(
                "value {value} is less than or equal to exclusive minimum value {exclusive_min}"
            );

            return Err(RangeError::ExclusiveMin {
                exclusive_min,
                value,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message,
            });
        }

        if let Some(exclusive_max) = exclusive_max
            && value >= exclusive_max
        {
            #[cfg(feature = "message")]
            let message = format!(
                "value {value} is greater than or equal to exclusive maximum value {exclusive_max}"
            );

            return Err(RangeError::ExclusiveMax {
                exclusive_max,
                value,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message,
            });
        }

        Ok(())
    }
}

macro_rules! validate_with_copy {
    ($type:ty) => {
        impl ValidateRange<$type> for $type {
            fn range_value(&self) -> Option<Self> {
                Some(*self)
            }
        }
    };
}

validate_with_copy!(bool);
validate_with_copy!(u8);
validate_with_copy!(u16);
validate_with_copy!(u32);
validate_with_copy!(u64);
validate_with_copy!(u128);
validate_with_copy!(usize);
validate_with_copy!(i8);
validate_with_copy!(i16);
validate_with_copy!(i32);
validate_with_copy!(i64);
validate_with_copy!(i128);
validate_with_copy!(isize);
validate_with_copy!(f32);
validate_with_copy!(f64);
validate_with_copy!(char);
#[cfg(feature = "chrono")]
validate_with_copy!(chrono::NaiveDate);
#[cfg(feature = "chrono")]
validate_with_copy!(chrono::NaiveDateTime);
#[cfg(feature = "chrono")]
validate_with_copy!(chrono::NaiveTime);
#[cfg(feature = "chrono")]
validate_with_copy!(chrono::TimeDelta);
#[cfg(feature = "decimal")]
validate_with_copy!(rust_decimal::Decimal);
#[cfg(feature = "uuid")]
validate_with_copy!(uuid::Uuid);

impl<'a> ValidateRange<&'a str> for &'a str {
    fn range_value(&self) -> Option<Self> {
        Some(self)
    }
}

macro_rules! validate_with_clone {
    ($type:ty) => {
        impl ValidateRange<$type> for $type {
            fn range_value(&self) -> Option<Self> {
                Some(self.clone())
            }
        }
    };
}

validate_with_clone!(String);

#[cfg(feature = "chrono")]
impl<Tz> ValidateRange<chrono::DateTime<Tz>> for chrono::DateTime<Tz>
where
    Tz: chrono::TimeZone,
    Tz::Offset: Display,
{
    fn range_value(&self) -> Option<Self> {
        Some(self.clone())
    }
}

impl<L, T> ValidateRange<L> for Option<T>
where
    L: Display + PartialEq + PartialOrd,
    T: ValidateRange<L>,
{
    fn range_value(&self) -> Option<L> {
        if let Some(s) = self {
            T::range_value(s)
        } else {
            None
        }
    }
}

macro_rules! validate_with_deref {
    ($type:ty) => {
        impl<V, T> ValidateRange<V> for $type
        where
            V: Display + PartialEq + PartialOrd,
            T: ValidateRange<V>,
        {
            fn range_value(&self) -> Option<V> {
                T::range_value(self)
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
    use std::{cell::RefCell, rc::Rc, sync::Arc};

    #[cfg(feature = "chrono")]
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
    #[cfg(feature = "decimal")]
    use rust_decimal::dec;
    use uuid::Uuid;

    use super::{RangeError, RangeErrorCode, ValidateRange};

    #[test]
    fn ok() {
        assert_eq!(false.validate_range(Some(false), None, None, None), Ok(()));
        assert_eq!(3u8.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3u16.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3u32.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3u32.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3u64.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3u128.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3usize.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3i8.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3i16.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3i32.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3i32.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3i64.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3i128.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3isize.validate_range(Some(1), None, None, None), Ok(()));
        assert_eq!(3.0f32.validate_range(Some(1.0), None, None, None), Ok(()));
        assert_eq!(3.0f64.validate_range(Some(1.0), None, None, None), Ok(()));
        assert_eq!('c'.validate_range(Some('a'), None, None, None), Ok(()));
        assert_eq!("c".validate_range(Some("a"), None, None, None), Ok(()));

        #[cfg(feature = "chrono")]
        {
            assert_eq!(
                NaiveDate::from_ymd_opt(2025, 4, 3)
                    .expect("valid date")
                    .validate_range(
                        Some(NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date")),
                        None,
                        None,
                        None
                    ),
                Ok(())
            );
            assert_eq!(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2025, 4, 3).expect("valid date"),
                    NaiveTime::from_hms_opt(12, 34, 56).expect("valid time")
                )
                .validate_range(
                    Some(NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    )),
                    None,
                    None,
                    None
                ),
                Ok(())
            );
            assert_eq!(
                NaiveTime::from_hms_opt(12, 34, 56)
                    .expect("valid time")
                    .validate_range(
                        Some(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")),
                        None,
                        None,
                        None
                    ),
                Ok(())
            );
            assert_eq!(
                TimeDelta::minutes(3).validate_range(Some(TimeDelta::minutes(1)), None, None, None),
                Ok(())
            );
            assert_eq!(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2025, 4, 3).expect("valid date"),
                    NaiveTime::from_hms_opt(12, 34, 56).expect("valid time")
                )
                .and_utc()
                .validate_range(
                    Some(
                        NaiveDateTime::new(
                            NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                            NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                        )
                        .and_utc()
                    ),
                    None,
                    None,
                    None
                ),
                Ok(())
            );
        }

        #[cfg(feature = "decimal")]
        assert_eq!(
            dec!(3.0).validate_range(Some(dec!(1.0)), None, None, None),
            Ok(())
        );

        #[cfg(feature = "uuid")]
        assert_eq!(
            Uuid::max().validate_range(Some(Uuid::nil()), None, None, None),
            Ok(())
        );

        assert_eq!(
            None::<i32>.validate_range(Some(1), None, None, None),
            Ok(())
        );
        assert_eq!(Some(3).validate_range(Some(1), None, None, None), Ok(()));

        #[expect(clippy::needless_borrow)]
        {
            assert_eq!((&3).validate_range(Some(1), None, None, None), Ok(()));
        }
        #[expect(unused_allocation)]
        {
            assert_eq!(
                (Box::new(3)).validate_range(Some(1), None, None, None),
                Ok(())
            );
        }
        assert_eq!(
            (Arc::new(3)).validate_range(Some(1), None, None, None),
            Ok(())
        );
        assert_eq!(
            (Rc::new(3)).validate_range(Some(1), None, None, None),
            Ok(())
        );

        let cell = RefCell::new(3);
        assert_eq!(
            cell.borrow().validate_range(Some(1), None, None, None),
            Ok(())
        );
        assert_eq!(
            cell.borrow_mut().validate_range(Some(1), None, None, None),
            Ok(())
        );
    }

    #[test]
    fn min_error() {
        assert_eq!(
            false.validate_range(Some(true), None, None, None),
            Err(RangeError::Min {
                min: true,
                value: false,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value false is less than minimum value true".to_owned(),
            })
        );
        assert_eq!(
            3u8.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3u16.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3u32.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3u32.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3u64.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3u128.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3usize.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3i8.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3i16.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3i32.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3i32.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3i64.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3i128.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3isize.validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3.0f32.validate_range(Some(4.0), None, None, None),
            Err(RangeError::Min {
                min: 4.0,
                value: 3.0,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            3.0f64.validate_range(Some(4.0), None, None, None),
            Err(RangeError::Min {
                min: 4.0,
                value: 3.0,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            'c'.validate_range(Some('d'), None, None, None),
            Err(RangeError::Min {
                min: 'd',
                value: 'c',
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value c is less than minimum value d".to_owned(),
            })
        );
        assert_eq!(
            "c".validate_range(Some("d"), None, None, None),
            Err(RangeError::Min {
                min: "d",
                value: "c",
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value c is less than minimum value d".to_owned(),
            })
        );

        #[cfg(feature = "chrono")]
        {
            assert_eq!(
                NaiveDate::from_ymd_opt(2024, 3, 2)
                    .expect("valid date")
                    .validate_range(
                        Some(NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date")),
                        None,
                        None,
                        None
                    ),
                Err(RangeError::Min {
                    min: NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                    value: NaiveDate::from_ymd_opt(2024, 3, 2).expect("valid date"),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 2024-03-02 is less than minimum value 2025-01-01".to_owned(),
                })
            );
            assert_eq!(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2024, 3, 2).expect("valid date"),
                    NaiveTime::from_hms_opt(12, 34, 56).expect("valid time")
                )
                .validate_range(
                    Some(NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    )),
                    None,
                    None,
                    None
                ),
                Err(RangeError::Min {
                    min: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    ),
                    value: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 3, 2).expect("valid date"),
                        NaiveTime::from_hms_opt(12, 34, 56).expect("valid time")
                    ),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message:
                        "value 2024-03-02 12:34:56 is less than minimum value 2025-01-01 00:00:00"
                            .to_owned(),
                })
            );
            assert_eq!(
                NaiveTime::from_hms_opt(12, 34, 56)
                    .expect("valid time")
                    .validate_range(
                        Some(NaiveTime::from_hms_opt(13, 0, 0).expect("valid time")),
                        None,
                        None,
                        None
                    ),
                Err(RangeError::Min {
                    min: NaiveTime::from_hms_opt(13, 0, 0).expect("valid time"),
                    value: NaiveTime::from_hms_opt(12, 34, 56).expect("valid time"),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 12:34:56 is less than minimum value 13:00:00".to_owned(),
                })
            );
            assert_eq!(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2024, 3, 2).expect("valid date"),
                    NaiveTime::from_hms_opt(12, 34, 56).expect("valid time")
                )
                .and_utc()
                .validate_range(
                    Some(
                        NaiveDateTime::new(
                            NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                            NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                        )
                        .and_utc()
                    ),
                    None,
                    None,
                    None
                ),
                Err(RangeError::Min {
                    min: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    )
                    .and_utc(),
                    value: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 3, 2).expect("valid date"),
                        NaiveTime::from_hms_opt(12, 34, 56).expect("valid time")
                    )
                    .and_utc(),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message:
                        "value 2024-03-02 12:34:56 UTC is less than minimum value 2025-01-01 00:00:00 UTC"
                            .to_owned(),
                })
            );
            assert_eq!(
                TimeDelta::minutes(3).validate_range(Some(TimeDelta::minutes(4)), None, None, None),
                Err(RangeError::Min {
                    min: TimeDelta::minutes(4),
                    value: TimeDelta::minutes(3),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value PT180S is less than minimum value PT240S".to_owned(),
                })
            );
        }

        #[cfg(feature = "decimal")]
        assert_eq!(
            dec!(3.0).validate_range(Some(dec!(4.0)), None, None, None),
            Err(RangeError::Min {
                min: dec!(4.0),
                value: dec!(3.0),
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3.0 is less than minimum value 4.0".to_owned(),
            })
        );

        #[cfg(feature = "uuid")]
        assert_eq!(
            Uuid::nil().validate_range(Some(Uuid::max()), None, None, None),
            Err(RangeError::Min {
                min: Uuid::max(),
                value: Uuid::nil(),
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 00000000-0000-0000-0000-000000000000 is less than minimum value ffffffff-ffff-ffff-ffff-ffffffffffff".to_owned(),
            })
        );

        assert_eq!(
            Some(3).validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );

        #[expect(clippy::needless_borrow)]
        {
            assert_eq!(
                (&3).validate_range(Some(4), None, None, None),
                Err(RangeError::Min {
                    min: 4,
                    value: 3,
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 3 is less than minimum value 4".to_owned(),
                })
            );
        }
        #[expect(unused_allocation)]
        {
            assert_eq!(
                (Box::new(3)).validate_range(Some(4), None, None, None),
                Err(RangeError::Min {
                    min: 4,
                    value: 3,
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 3 is less than minimum value 4".to_owned(),
                })
            );
        }
        assert_eq!(
            (Arc::new(3)).validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            (Rc::new(3)).validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );

        let cell = RefCell::new(3);
        assert_eq!(
            cell.borrow().validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
        assert_eq!(
            cell.borrow_mut().validate_range(Some(4), None, None, None),
            Err(RangeError::Min {
                min: 4,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than minimum value 4".to_owned(),
            })
        );
    }

    #[test]
    fn max_error() {
        assert_eq!(
            true.validate_range(None, Some(false), None, None),
            Err(RangeError::Max {
                max: false,
                value: true,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value true is greater than maximum value false".to_owned(),
            })
        );
        assert_eq!(
            3u8.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3u16.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3u32.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3u32.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3u64.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3u128.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3usize.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3i8.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3i16.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3i32.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3i32.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3i64.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3i128.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3isize.validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3.0f32.validate_range(None, Some(2.0), None, None),
            Err(RangeError::Max {
                max: 2.0,
                value: 3.0,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            3.0f64.validate_range(None, Some(2.0), None, None),
            Err(RangeError::Max {
                max: 2.0,
                value: 3.0,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            'c'.validate_range(None, Some('b'), None, None),
            Err(RangeError::Max {
                max: 'b',
                value: 'c',
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value c is greater than maximum value b".to_owned(),
            })
        );
        assert_eq!(
            "c".validate_range(None, Some("b"), None, None),
            Err(RangeError::Max {
                max: "b",
                value: "c",
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value c is greater than maximum value b".to_owned(),
            })
        );

        #[cfg(feature = "chrono")]
        {
            assert_eq!(
                NaiveDate::from_ymd_opt(2025, 4, 3)
                    .expect("valid date")
                    .validate_range(
                        None,
                        Some(NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date")),
                        None,
                        None
                    ),
                Err(RangeError::Max {
                    max: NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                    value: NaiveDate::from_ymd_opt(2025, 4, 3).expect("valid date"),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 2025-04-03 is greater than maximum value 2025-01-01".to_owned(),
                })
            );
            assert_eq!(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2025, 4, 3).expect("valid date"),
                    NaiveTime::from_hms_opt(12, 34, 56).expect("valid time")
                )
                .validate_range(
                    None,
                    Some(NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    )),
                    None,
                    None
                ),
                Err(RangeError::Max {
                    max: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    ),
                    value: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 4, 3).expect("valid date"),
                        NaiveTime::from_hms_opt(12, 34, 56).expect("valid time")
                    ),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message:
                        "value 2025-04-03 12:34:56 is greater than maximum value 2025-01-01 00:00:00"
                            .to_owned(),
                })
            );
            assert_eq!(
                NaiveTime::from_hms_opt(12, 34, 56)
                    .expect("valid time")
                    .validate_range(
                        None,
                        Some(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")),
                        None,
                        None
                    ),
                Err(RangeError::Max {
                    max: NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"),
                    value: NaiveTime::from_hms_opt(12, 34, 56).expect("valid time"),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 12:34:56 is greater than maximum value 00:00:00".to_owned(),
                })
            );
            assert_eq!(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2025, 4, 3).expect("valid date"),
                    NaiveTime::from_hms_opt(12, 34, 56).expect("valid time")
                ).and_utc()
                .validate_range(
                    None,
                    Some(NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    ).and_utc()),
                    None,
                    None
                ),
                Err(RangeError::Max {
                    max: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    ).and_utc(),
                    value: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 4, 3).expect("valid date"),
                        NaiveTime::from_hms_opt(12, 34, 56).expect("valid time")
                    ).and_utc(),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message:
                        "value 2025-04-03 12:34:56 UTC is greater than maximum value 2025-01-01 00:00:00 UTC"
                            .to_owned(),
                })
            );
            assert_eq!(
                TimeDelta::minutes(3).validate_range(None, Some(TimeDelta::minutes(2)), None, None),
                Err(RangeError::Max {
                    max: TimeDelta::minutes(2),
                    value: TimeDelta::minutes(3),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value PT180S is greater than maximum value PT120S".to_owned(),
                })
            );
        }

        #[cfg(feature = "decimal")]
        assert_eq!(
            dec!(3.0).validate_range(None, Some(dec!(2.0)), None, None),
            Err(RangeError::Max {
                max: dec!(2.0),
                value: dec!(3.0),
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3.0 is greater than maximum value 2.0".to_owned(),
            })
        );

        #[cfg(feature = "uuid")]
        assert_eq!(
            Uuid::max().validate_range(None, Some(Uuid::nil()), None, None),
            Err(RangeError::Max {
                max: Uuid::nil(),
                value: Uuid::max(),
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value ffffffff-ffff-ffff-ffff-ffffffffffff is greater than maximum value 00000000-0000-0000-0000-000000000000".to_owned(),
            })
        );

        assert_eq!(
            Some(3).validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );

        #[expect(clippy::needless_borrow)]
        {
            assert_eq!(
                (&3).validate_range(None, Some(2), None, None),
                Err(RangeError::Max {
                    max: 2,
                    value: 3,
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 3 is greater than maximum value 2".to_owned(),
                })
            );
        }
        #[expect(unused_allocation)]
        {
            assert_eq!(
                (Box::new(3)).validate_range(None, Some(2), None, None),
                Err(RangeError::Max {
                    max: 2,
                    value: 3,
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 3 is greater than maximum value 2".to_owned(),
                })
            );
        }
        assert_eq!(
            (Arc::new(3)).validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            (Rc::new(3)).validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );

        let cell = RefCell::new(3);
        assert_eq!(
            cell.borrow().validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
        assert_eq!(
            cell.borrow_mut().validate_range(None, Some(2), None, None),
            Err(RangeError::Max {
                max: 2,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than maximum value 2".to_owned(),
            })
        );
    }

    #[test]
    fn exclusive_min_error() {
        assert_eq!(
            true.validate_range(None, None, Some(true), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: true,
                value: true,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value true is less than or equal to exclusive minimum value true"
                    .to_owned(),
            })
        );
        assert_eq!(
            3u8.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3u16.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3u32.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3u32.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3u64.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3u128.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3usize.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3i8.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3i16.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3i32.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3i32.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3i64.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3i128.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3isize.validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3.0f32.validate_range(None, None, Some(3.0), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3.0,
                value: 3.0,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            3.0f64.validate_range(None, None, Some(3.0), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3.0,
                value: 3.0,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            'c'.validate_range(None, None, Some('c'), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 'c',
                value: 'c',
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value c is less than or equal to exclusive minimum value c".to_owned(),
            })
        );
        assert_eq!(
            "c".validate_range(None, None, Some("c"), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: "c",
                value: "c",
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value c is less than or equal to exclusive minimum value c".to_owned(),
            })
        );

        #[cfg(feature = "chrono")]
        {
            assert_eq!(
                NaiveDate::from_ymd_opt(2025, 1, 1)
                    .expect("valid date")
                    .validate_range(
                        None,
                        None,
                        Some(NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date")),
                        None
                    ),
                Err(RangeError::ExclusiveMin {
                    exclusive_min: NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                    value: NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 2025-01-01 is less than or equal to exclusive minimum value 2025-01-01"
                        .to_owned(),
                })
            );
            assert_eq!(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                    NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                )
                .validate_range(
                    None,
                    None,
                    Some(NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    )),
                    None
                ),
                Err(RangeError::ExclusiveMin {
                    exclusive_min: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    ),
                    value: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0,0,0).expect("valid time")
                    ),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message:
                        "value 2025-01-01 00:00:00 is less than or equal to exclusive minimum value 2025-01-01 00:00:00"
                            .to_owned(),
                })
            );
            assert_eq!(
                NaiveTime::from_hms_opt(0, 0, 0)
                    .expect("valid time")
                    .validate_range(
                        None,
                        None,
                        Some(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")),
                        None
                    ),
                Err(RangeError::ExclusiveMin {
                    exclusive_min: NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"),
                    value: NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message:
                        "value 00:00:00 is less than or equal to exclusive minimum value 00:00:00"
                            .to_owned(),
                })
            );
            assert_eq!(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                    NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                ).and_utc()
                .validate_range(
                    None,
                    None,
                    Some(NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    ).and_utc()),
                    None
                ),
                Err(RangeError::ExclusiveMin {
                    exclusive_min: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    ).and_utc(),
                    value: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0,0,0).expect("valid time")
                    ).and_utc(),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message:
                        "value 2025-01-01 00:00:00 UTC is less than or equal to exclusive minimum value 2025-01-01 00:00:00 UTC"
                            .to_owned(),
                })
            );
            assert_eq!(
                TimeDelta::minutes(3).validate_range(None, None, Some(TimeDelta::minutes(3)), None),
                Err(RangeError::ExclusiveMin {
                    exclusive_min: TimeDelta::minutes(3),
                    value: TimeDelta::minutes(3),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value PT180S is less than or equal to exclusive minimum value PT180S"
                        .to_owned(),
                })
            );
        }

        #[cfg(feature = "decimal")]
        assert_eq!(
            dec!(3.0).validate_range(None, None, Some(dec!(3.0)), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: dec!(3.0),
                value: dec!(3.0),
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3.0 is less than or equal to exclusive minimum value 3.0"
                    .to_owned(),
            })
        );

        #[cfg(feature = "uuid")]
        assert_eq!(
            Uuid::nil().validate_range(None, None, Some(Uuid::nil()), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: Uuid::nil(),
                value: Uuid::nil(),
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 00000000-0000-0000-0000-000000000000 is less than or equal to exclusive minimum value 00000000-0000-0000-0000-000000000000".to_owned(),
            })
        );

        assert_eq!(
            Some(3).validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );

        #[expect(clippy::needless_borrow)]
        {
            assert_eq!(
                (&3).validate_range(None, None, Some(3), None),
                Err(RangeError::ExclusiveMin {
                    exclusive_min: 3,
                    value: 3,
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 3 is less than or equal to exclusive minimum value 3"
                        .to_owned(),
                })
            );
        }
        #[expect(unused_allocation)]
        {
            assert_eq!(
                (Box::new(3)).validate_range(None, None, Some(3), None),
                Err(RangeError::ExclusiveMin {
                    exclusive_min: 3,
                    value: 3,
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 3 is less than or equal to exclusive minimum value 3"
                        .to_owned(),
                })
            );
        }
        assert_eq!(
            (Arc::new(3)).validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            (Rc::new(3)).validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );

        let cell = RefCell::new(3);
        assert_eq!(
            cell.borrow().validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
        assert_eq!(
            cell.borrow_mut().validate_range(None, None, Some(3), None),
            Err(RangeError::ExclusiveMin {
                exclusive_min: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is less than or equal to exclusive minimum value 3".to_owned(),
            })
        );
    }

    #[test]
    fn exclusive_max_error() {
        assert_eq!(
            true.validate_range(None, None, None, Some(true)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: true,
                value: true,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value true is greater than or equal to exclusive maximum value true"
                    .to_owned(),
            })
        );
        assert_eq!(
            3u8.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3u16.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3u32.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3u32.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3u64.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3u128.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3usize.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3i8.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3i16.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3i32.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3i32.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3i64.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3i128.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3isize.validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3.0f32.validate_range(None, None, None, Some(3.0)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3.0,
                value: 3.0,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            3.0f64.validate_range(None, None, None, Some(3.0)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3.0,
                value: 3.0,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            'c'.validate_range(None, None, None, Some('c')),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 'c',
                value: 'c',
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value c is greater than or equal to exclusive maximum value c".to_owned(),
            })
        );
        assert_eq!(
            "c".validate_range(None, None, None, Some("c")),
            Err(RangeError::ExclusiveMax {
                exclusive_max: "c",
                value: "c",
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value c is greater than or equal to exclusive maximum value c".to_owned(),
            })
        );

        #[cfg(feature = "chrono")]
        {
            assert_eq!(
                NaiveDate::from_ymd_opt(2025, 1, 1)
                    .expect("valid date")
                    .validate_range(
                        None,
                        None,
                        None,
                        Some(NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date")),
                    ),
                Err(RangeError::ExclusiveMax {
                    exclusive_max: NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                    value: NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 2025-01-01 is greater than or equal to exclusive maximum value 2025-01-01"
                        .to_owned(),
                })
            );
            assert_eq!(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                    NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                )
                .validate_range(
                    None,
                    None,
                    None,
                    Some(NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    )),
                ),
                Err(RangeError::ExclusiveMax {
                    exclusive_max: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    ),
                    value: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0,0,0).expect("valid time")
                    ),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message:
                        "value 2025-01-01 00:00:00 is greater than or equal to exclusive maximum value 2025-01-01 00:00:00"
                            .to_owned(),
                })
            );
            assert_eq!(
                NaiveTime::from_hms_opt(0, 0, 0)
                    .expect("valid time")
                    .validate_range(
                        None,
                        None,
                        None,
                        Some(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")),
                    ),
                Err(RangeError::ExclusiveMax {
                    exclusive_max: NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"),
                    value: NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message:
                        "value 00:00:00 is greater than or equal to exclusive maximum value 00:00:00"
                            .to_owned(),
                })
            );
            assert_eq!(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                    NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                ).and_utc()
                .validate_range(
                    None,
                    None,
                    None,
                    Some(NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    ).and_utc()),
                ),
                Err(RangeError::ExclusiveMax {
                    exclusive_max: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0, 0, 0).expect("valid time")
                    ).and_utc(),
                    value: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        NaiveTime::from_hms_opt(0,0,0).expect("valid time")
                    ).and_utc(),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message:
                        "value 2025-01-01 00:00:00 UTC is greater than or equal to exclusive maximum value 2025-01-01 00:00:00 UTC"
                            .to_owned(),
                })
            );
            assert_eq!(
                TimeDelta::minutes(3)
                    .validate_range(None, None, None, Some(TimeDelta::minutes(3)),),
                Err(RangeError::ExclusiveMax {
                    exclusive_max: TimeDelta::minutes(3),
                    value: TimeDelta::minutes(3),
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message:
                        "value PT180S is greater than or equal to exclusive maximum value PT180S"
                            .to_owned(),
                })
            );
        }

        #[cfg(feature = "decimal")]
        assert_eq!(
            dec!(3.0).validate_range(None, None, None, Some(dec!(3.0))),
            Err(RangeError::ExclusiveMax {
                exclusive_max: dec!(3.0),
                value: dec!(3.0),
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3.0 is greater than or equal to exclusive maximum value 3.0"
                    .to_owned(),
            })
        );

        #[cfg(feature = "uuid")]
        assert_eq!(
            Uuid::max().validate_range(None, None,None, Some(Uuid::max())),
            Err(RangeError::ExclusiveMax {
                exclusive_max: Uuid::max(),
                value: Uuid::max(),
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value ffffffff-ffff-ffff-ffff-ffffffffffff is greater than or equal to exclusive maximum value ffffffff-ffff-ffff-ffff-ffffffffffff".to_owned(),
            })
        );

        assert_eq!(
            Some(3).validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );

        #[expect(clippy::needless_borrow)]
        {
            assert_eq!(
                (&3).validate_range(None, None, None, Some(3)),
                Err(RangeError::ExclusiveMax {
                    exclusive_max: 3,
                    value: 3,
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 3 is greater than or equal to exclusive maximum value 3"
                        .to_owned(),
                })
            );
        }
        #[expect(unused_allocation)]
        {
            assert_eq!(
                (Box::new(3)).validate_range(None, None, None, Some(3)),
                Err(RangeError::ExclusiveMax {
                    exclusive_max: 3,
                    value: 3,
                    code: RangeErrorCode,
                    #[cfg(feature = "message")]
                    message: "value 3 is greater than or equal to exclusive maximum value 3"
                        .to_owned(),
                })
            );
        }
        assert_eq!(
            (Arc::new(3)).validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            (Rc::new(3)).validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );

        let cell = RefCell::new(3);
        assert_eq!(
            cell.borrow().validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
        assert_eq!(
            cell.borrow_mut().validate_range(None, None, None, Some(3)),
            Err(RangeError::ExclusiveMax {
                exclusive_max: 3,
                value: 3,
                code: RangeErrorCode,
                #[cfg(feature = "message")]
                message: "value 3 is greater than or equal to exclusive maximum value 3".to_owned(),
            })
        );
    }
}
