use std::fmt::Debug;

use fortifier::{Validate, ValidateWithContext};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(untagged, rename_all = "camelCase")]
enum CreateOrUpdate<C, U>
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

type CreateOrUpdateUser = CreateOrUpdate<CreateUser, UpdateUser>;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
struct CreateUser {
    #[validate(email_address)]
    email: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
struct UpdateUser {
    #[validate(email_address)]
    email: Option<String>,
}

fn main() {
    let data = CreateOrUpdateUser::Create {
        data: CreateUser {
            email: "amy.pond@example.com".to_owned(),
        },
    };

    assert!(data.validate_sync().is_ok());

    let data = CreateOrUpdateUser::Update {
        id: Uuid::nil(),
        data: UpdateUser {
            email: Some("amy.pond@example.com".to_owned()),
        },
    };

    assert!(data.validate_sync().is_ok());
}
