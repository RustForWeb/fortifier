use std::fmt::Debug;

use fortifier::Validate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Validate)]
#[validate(custom(function = validate_min_max, error = BoundsMinMaxError<T>))]
pub struct Bounds<T>
where
    T: Clone + Debug + PartialEq + PartialOrd + Send + Sync,
{
    pub min: Option<T>,
    pub max: Option<T>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BoundsMinMaxError<T>
where
    T: Debug + PartialEq,
{
    pub min: T,
    pub max: T,
}

fn validate_min_max<T>(value: &Bounds<T>) -> Result<(), BoundsMinMaxError<T>>
where
    T: Clone + Debug + PartialEq + PartialOrd + Send + Sync,
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
