use fortifier::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateUser {
    #[validate(email)]
    pub email_address: String,

    #[validate(length(min = 1, max = 256))]
    pub name: String,
}
