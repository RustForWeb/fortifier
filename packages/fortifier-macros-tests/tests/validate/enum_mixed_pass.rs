use fortifier::{Validate, ValidationErrors};
use serde::{Deserialize, Serialize};

#[derive(Validate)]
#[validate(custom(function = validate_custom, error = CustomError))]
enum FieldType {
    Boolean,
    Integer,
    Decimal {
        #[validate(range(max = 10))]
        scale: u32,
    },
    String(#[validate(range(min = 1))] usize),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct CustomError;

fn validate_custom(_value: &FieldType) -> Result<(), CustomError> {
    Ok(())
}

fn main() -> Result<(), ValidationErrors<FieldTypeValidationError>> {
    let data = FieldType::Boolean;

    data.validate_sync()?;

    let data = FieldType::Integer;

    data.validate_sync()?;

    let data = FieldType::Decimal { scale: 3 };

    data.validate_sync()?;

    let data = FieldType::String(256);

    data.validate_sync()?;

    Ok(())
}
