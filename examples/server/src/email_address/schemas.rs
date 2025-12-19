use fortifier::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmailAddress {
    #[validate(email_address)]
    pub email_address: String,

    #[validate(length(min = 1, max = 256))]
    pub label: String,
}

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmailAddress {
    #[validate(email_address)]
    pub email_address: Option<String>,

    #[validate(length(min = 1, max = 256))]
    pub label: Option<String>,
}

#[derive(Deserialize, ToSchema, Validate)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ChangeEmailAddressRelation {
    Create(CreateEmailAddress),
    Update {
        #[validate(skip)]
        id: Uuid,

        #[serde(flatten)]
        data: UpdateEmailAddress,
    },
    Delete {
        #[validate(skip)]
        id: Uuid,
    },
}
