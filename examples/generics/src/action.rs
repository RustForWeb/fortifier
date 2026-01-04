use std::fmt::Debug;

use fortifier::{Validate, ValidateWithContext};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CreateOrUpdate<C, U>
where
    C: Debug + Send + Sync + ValidateWithContext<Context = ()>,
    U: Debug + Send + Sync + ValidateWithContext<Context = ()>,
{
    Update {
        #[validate(skip)]
        id: Uuid,
        #[serde(flatten)]
        data: U,
    },
    Create {
        #[serde(flatten)]
        data: C,
    },
}

pub type CreateOrUpdateUser = CreateOrUpdate<CreateUser, UpdateUser>;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateUser {
    #[validate(email_address)]
    pub email: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUser {
    #[validate(email_address)]
    pub email: Option<String>,
}
