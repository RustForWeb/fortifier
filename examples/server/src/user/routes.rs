use axum::{Json, http::StatusCode, response::IntoResponse};
use fortifier::{Validate, ValidationErrors};
use thiserror::Error;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::user::{
    entities::user,
    schemas::{CreateUser, CreateUserValidationError},
};

pub struct UserRoutes;

impl UserRoutes {
    pub fn router<S>() -> OpenApiRouter<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        OpenApiRouter::new().routes(routes!(create_user))
    }
}

#[derive(Debug, Error)]
enum CreateUserError {
    #[error(transparent)]
    UnprocessableContent(#[from] ValidationErrors<CreateUserValidationError>),
}

impl IntoResponse for CreateUserError {
    fn into_response(self) -> axum::response::Response {
        todo!()
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
        (status = 201, description = "The created user.", body = user::Model),
        (status = 400, description = "Validation error.", body = ValidationErrors<CreateUserValidationError>),
        // (status = 500, description = "Internal server error.", body = ErrorBody),
    )
)]
async fn create_user(
    Json(data): Json<CreateUser>,
) -> Result<(StatusCode, Json<user::Model>), CreateUserError> {
    data.validate().await?;

    let user = user::Model {
        id: Uuid::now_v7(),
        email_address: data.email_address,
        name: data.name,
    };

    Ok((StatusCode::CREATED, Json(user)))
}
