use std::fmt::Debug;

use fortifier::{Validate, ValidationErrors};

#[derive(Validate)]
#[validate(custom(function = validate_min_max, error = BoundsMinMaxError<T>))]
struct Bounds<T>
where
    T: Clone + Debug + PartialEq + PartialOrd,
{
    min: Option<T>,
    max: Option<T>,
}

#[derive(Debug, PartialEq)]
struct BoundsMinMaxError<T>
where
    T: Debug + PartialEq,
{
    min: T,
    max: T,
}

fn validate_min_max<T>(value: &Bounds<T>) -> Result<(), BoundsMinMaxError<T>>
where
    T: Clone + Debug + PartialEq + PartialOrd,
{
    if let Some(min) = &value.min
        && let Some(max) = &value.max
        && min > max
    {
        Err(BoundsMinMaxError {
            min: min.clone(),
            max: max.clone(),
        })
    } else {
        Ok(())
    }
}

fn main() {
    let bounds = Bounds {
        min: Some(1),
        max: Some(10),
    };

    assert_eq!(bounds.validate_sync(), Ok(()));

    let bounds = Bounds {
        min: Some(11),
        max: Some(10),
    };

    assert_eq!(
        bounds.validate_sync(),
        Err(ValidationErrors::from_iter([BoundsValidationError::Root(
            BoundsMinMaxError { min: 11, max: 10 }
        )]))
    );
}
