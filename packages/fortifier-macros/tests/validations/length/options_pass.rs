use fortifier::{LengthError, LengthErrorCode, Validate, ValidationErrors};

#[derive(Validate)]
struct LengthData<'a> {
    #[validate(length(equal = 2))]
    equal: &'a str,
    #[validate(length(min = 1))]
    min: &'a str,
    #[validate(length(max = 4))]
    max: &'a str,
    #[validate(length(min = 1, max = 4))]
    min_max: &'a str,
}

fn main() {
    let data = LengthData {
        equal: "a",
        min: "",
        max: "abcde",
        min_max: "abcdef",
    };

    assert_eq!(
        data.validate_sync(),
        Err(ValidationErrors::from_iter([
            LengthDataValidationError::Equal(LengthError::Equal {
                code: LengthErrorCode,
                equal: 2,
                length: 1
            }),
            LengthDataValidationError::Min(LengthError::Min {
                code: LengthErrorCode,
                min: 1,
                length: 0
            }),
            LengthDataValidationError::Max(LengthError::Max {
                code: LengthErrorCode,
                max: 4,
                length: 5
            }),
            LengthDataValidationError::MinMax(LengthError::Max {
                code: LengthErrorCode,
                max: 4,
                length: 6
            })
        ]))
    );
}
