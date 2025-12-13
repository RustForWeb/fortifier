use fortifier::{Validate, ValidateWithContext, ValidationErrors};

struct Context {
    min: usize,
    max: usize,
}

#[derive(Validate)]
#[validate(context = Context)]
struct CreateUser {
    #[validate(length(min = context.min, max = context.max))]
    name: String,
}

fn main() -> Result<(), ValidationErrors<CreateUserValidationError>> {
    let data = CreateUser {
        name: "John Doe".to_owned(),
    };

    data.validate_sync_with_context(&Context { min: 1, max: 256 })?;

    Ok(())
}
