use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::core::component_group::ComponentGroup;
use crate::presentation::routers::v1::component_groups::{CreateRequest, UpdateRequest};

#[derive(OpenApi)]
#[openapi(
    info(title = "Keepalive API", version = "0.1.0"),
    servers((url = "/api/v1", description = "Version 1")),
    paths(
        crate::presentation::routers::v1::component_groups::list,
        crate::presentation::routers::v1::component_groups::create,
        crate::presentation::routers::v1::component_groups::find,
        crate::presentation::routers::v1::component_groups::update,
        crate::presentation::routers::v1::component_groups::delete,
    ),
    components(schemas(ComponentGroup, CreateRequest, UpdateRequest))
)]
pub struct ApiDoc;

pub fn docs_router() -> Router {
    Router::new().merge(Scalar::with_url("/docs", ApiDoc::openapi()))
}
