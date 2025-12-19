use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use fortifier::{Validate, ValidationErrors};
use serde::Deserialize;
use thiserror::Error;
use utoipa::IntoParams;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    email_address::entities::email_address,
    user::{
        entities::user,
        schemas::{
            CreateUser, CreateUserValidationError, UpdateUser, UpdateUserValidationError,
            UserWithEmailAddresses,
        },
    },
};

pub struct UserRoutes;

impl UserRoutes {
    pub fn router<S>() -> OpenApiRouter<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        OpenApiRouter::new()
            .routes(routes!(create_user))
            .routes(routes!(user, update_user, delete_user))
    }
}

#[derive(Debug, Error)]
enum CreateUserError {
    #[error(transparent)]
    UnprocessableContent(#[from] ValidationErrors<CreateUserValidationError>),
}

impl IntoResponse for CreateUserError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CreateUserError::UnprocessableContent(errors) => {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(errors)).into_response()
            }
        }
    }
}

#[utoipa::path(
    post,
    path = "/users",
    operation_id = "createUser",
    summary = "Create user",
    description = "Create a user.",
    tags = ["User"],
    request_body = CreateUser,
    responses(
        (status = CREATED, description = "The created user.", body = UserWithEmailAddresses),
        (status = UNPROCESSABLE_ENTITY, description = "Validation error.", body = ValidationErrors<CreateUserValidationError>),
    )
)]
async fn create_user(
    Json(data): Json<CreateUser>,
) -> Result<(StatusCode, Json<UserWithEmailAddresses>), CreateUserError> {
    data.validate().await?;

    let user = user::Model {
        id: Uuid::now_v7(),
        name: data.name,
    };

    let mut email_addresses = Vec::with_capacity(data.email_addresses.len());
    for email_address_data in data.email_addresses {
        email_addresses.push(email_address::Model {
            id: Uuid::now_v7(),
            email_addres: email_address_data.email_address,
            label: email_address_data.label,
        });
    }

    Ok((
        StatusCode::CREATED,
        Json(UserWithEmailAddresses {
            model: user,
            email_addresses,
        }),
    ))
}

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct UserPathParams {
    user_id: Uuid,
}

#[utoipa::path(
    get,
    path = "/users/{userId}",
    operation_id = "getUser",
    summary = "Get user",
    description = "Get a user.",
    tags = ["User"],
      params(
        UserPathParams,
    ),
    responses(
        (status = OK, description = "The user.", body = UserWithEmailAddresses),
        (status = NOT_FOUND, description = "Not found error."),
    )
)]
async fn user(
    Path(UserPathParams { user_id }): Path<UserPathParams>,
) -> Result<Json<UserWithEmailAddresses>, StatusCode> {
    // TODO

    let user = user::Model {
        id: user_id,
        name: "".to_owned(),
    };

    let email_addresses = vec![];

    Ok(Json(UserWithEmailAddresses {
        model: user,
        email_addresses,
    }))
}

#[derive(Debug, Error)]
enum UpdateUserError {
    #[error(transparent)]
    UnprocessableContent(#[from] ValidationErrors<UpdateUserValidationError>),
}

impl IntoResponse for UpdateUserError {
    fn into_response(self) -> axum::response::Response {
        match self {
            UpdateUserError::UnprocessableContent(errors) => {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(errors)).into_response()
            }
        }
    }
}

#[utoipa::path(
    patch,
    path = "/users/{userId}",
    operation_id = "updateUser",
    summary = "Update user",
    description = "Update a user.",
    tags = ["User"],
      params(
        UserPathParams,
    ),
    request_body = UpdateUser,
    responses(
        (status = OK, description = "The updated user.", body = UserWithEmailAddresses),
        (status = UNPROCESSABLE_ENTITY, description = "Validation error.", body = ValidationErrors<UpdateUserValidationError>),
    )
)]
async fn update_user(
    Path(UserPathParams { user_id }): Path<UserPathParams>,
    Json(data): Json<UpdateUser>,
) -> Result<Json<UserWithEmailAddresses>, UpdateUserError> {
    data.validate().await?;

    // TODO

    let user = user::Model {
        id: user_id,
        name: "".to_owned(),
    };

    let email_addresses = vec![];

    Ok(Json(UserWithEmailAddresses {
        model: user,
        email_addresses,
    }))
}

#[utoipa::path(
    delete,
    path = "/users/{userId}",
    operation_id = "deleteUser",
    summary = "Delete user",
    description = "Delete a user.",
    tags = ["User"],
      params(
        UserPathParams,
    ),
    responses(
        (status = NO_CONTENT, description = "The user was deleted.",),
        (status = NOT_FOUND, description = "Not found error."),
    )
)]
async fn delete_user(
    Path(UserPathParams { user_id: _user_id }): Path<UserPathParams>,
) -> Result<StatusCode, StatusCode> {
    // TODO

    Ok(StatusCode::NO_CONTENT)
}
