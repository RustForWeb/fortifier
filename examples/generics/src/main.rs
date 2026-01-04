mod action;
mod bounds;

use fortifier::{Validate, ValidationErrors};
use uuid::Uuid;

use crate::{
    action::{CreateOrUpdateUser, CreateUser, UpdateUser},
    bounds::{Bounds, BoundsMinMaxError, BoundsValidationError},
};

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

    let bounds = Bounds {
        min: Some(1),
        max: Some(10),
    };

    assert_eq!(bounds.validate_sync(), Ok(()));

    let bounds = Bounds {
        min: Some(11),
        max: Some(10),
    };

    assert_eq!(
        bounds.validate_sync(),
        Err(ValidationErrors::from_iter([BoundsValidationError::Root(
            BoundsMinMaxError { min: 11, max: 10 }
        )]))
    );
}
