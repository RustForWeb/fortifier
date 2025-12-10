mod email_address;
mod user;

use std::error::Error;

use fortifier::Validate;

use crate::{email_address::ChangeEmailAddressRelation, user::CreateUser};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = CreateUser {
        email: "john@doe.com".to_owned(),
        name: "John Doe".to_owned(),
        url: "https://john.doe.com".to_owned(),
        country_code: "GB".to_owned(),
        locales: vec!["en_GB".to_owned()],
    };

    data.validate().await?;

    let data = ChangeEmailAddressRelation::Create {
        email_address: "john@doe.com".to_owned(),
    };

    data.validate().await?;

    let data = ChangeEmailAddressRelation::Update {
        id: "1".to_owned(),
        email_address: "john@doe.com".to_owned(),
    };

    data.validate().await?;

    let data = ChangeEmailAddressRelation::Delete { id: "1".to_owned() };

    data.validate().await?;

    Ok(())
}
