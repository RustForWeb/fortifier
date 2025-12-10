use axum::{Json, Router, routing::get};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::user::routes::UserRoutes;

#[derive(OpenApi)]
#[openapi(info(
    title = "Fortifier Example API",
    description = "Example to showcase Fortifier validation.",
))]
pub struct Routes;

impl Routes {
    pub fn router<S>() -> Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        let router = OpenApiRouter::new().merge(UserRoutes::router());

        let (router, openapi) = OpenApiRouter::with_openapi(Routes::openapi())
            .nest("/api/v1", router)
            .split_for_parts();

        router
            .merge(Scalar::with_url("/api/reference", openapi.clone()))
            .route("/api/v1/openapi.json", get(|| async { Json(openapi) }))
    }
}
