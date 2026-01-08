use fortifier::{RangeError, RangeErrorCode, Validate, ValidationErrors};

#[derive(Validate)]
struct RangeData {
    #[validate(range(min = 1))]
    min: usize,
    #[validate(range(max = 4))]
    max: usize,
    #[validate(range(min = 1, max = 4))]
    min_max: usize,
    #[validate(range(exclusive_min = 1))]
    exclusive_min: usize,
    #[validate(range(exclusive_max = 4))]
    exclusive_max: usize,
    #[validate(range(exclusive_min = 1, exclusive_max = 4))]
    exclusive_min_exclusive_max: usize,
    #[validate(range(exclusive_min = 1, max = 7))]
    exclusive_min_max: usize,
    #[validate(range(min = 1, exclusive_max = 7))]
    min_exclusive_max: usize,

    #[validate(range(min = 1))]
    one_option: Option<usize>,
    #[validate(range(min = 1))]
    two_options: Option<Option<usize>>,
}

fn main() {
    let data = RangeData {
        min: 0,
        max: 5,
        min_max: 6,
        exclusive_min: 1,
        exclusive_max: 4,
        exclusive_min_exclusive_max: 1,
        exclusive_min_max: 2,
        min_exclusive_max: 2,

        one_option: Some(1),
        two_options: Some(Some(1)),
    };

    assert_eq!(
        data.validate_sync(),
        Err(ValidationErrors::from_iter([
            RangeDataValidationError::Min(RangeError::Min {
                code: RangeErrorCode,
                min: 1,
                value: 0
            }),
            RangeDataValidationError::Max(RangeError::Max {
                code: RangeErrorCode,
                max: 4,
                value: 5
            }),
            RangeDataValidationError::MinMax(RangeError::Max {
                code: RangeErrorCode,
                max: 4,
                value: 6
            }),
            RangeDataValidationError::ExclusiveMin(RangeError::ExclusiveMin {
                code: RangeErrorCode,
                exclusive_min: 1,
                value: 1
            }),
            RangeDataValidationError::ExclusiveMax(RangeError::ExclusiveMax {
                code: RangeErrorCode,
                exclusive_max: 4,
                value: 4
            }),
            RangeDataValidationError::ExclusiveMinExclusiveMax(RangeError::ExclusiveMin {
                code: RangeErrorCode,
                exclusive_min: 1,
                value: 1
            })
        ]))
    );
}
