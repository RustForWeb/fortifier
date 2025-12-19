use fortifier::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    email_address::{
        entities::email_address,
        schemas::{
            ChangeEmailAddressRelation, ChangeEmailAddressRelationValidationError,
            CreateEmailAddress, CreateEmailAddressValidationError,
        },
    },
    user::entities::user,
};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserWithEmailAddresses {
    #[serde(flatten)]
    pub model: user::Model,

    pub email_addresses: Vec<email_address::Model>,
}

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    #[validate(length(min = 1, max = 256))]
    pub name: String,

    pub email_addresses: Vec<CreateEmailAddress>,
}

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    #[validate(length(min = 1, max = 256))]
    pub name: String,

    pub email_addresses: Vec<ChangeEmailAddressRelation>,
}
