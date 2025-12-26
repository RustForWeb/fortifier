use fortifier::{Validate, ValidationErrors};

#[derive(Validate)]
enum FieldType {
    Boolean,
    Integer,
    Decimal {
        #[validate(range(max = 10))]
        scale: u32,
    },
    String(#[validate(range(min = 1))] usize),
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
