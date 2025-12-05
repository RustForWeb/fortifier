use std::error::Error;

use fortifier::Validate;

#[derive(Validate)]
struct CreateUser {
    #[validate(email)]
    email: String,

    #[validate(length(min = 1, max = 256))]
    name: String,

    #[validate(url)]
    url: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = CreateUser {
        email: "john@doe.com".to_owned(),
        name: "John Doe".to_owned(),
        url: "https://john.doe.com".to_owned(),
    };

    data.validate_sync()?;

    Ok(())
}
