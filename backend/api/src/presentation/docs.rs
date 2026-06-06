use axum::{response::Html, routing::get, Json, Router};
use utoipa::OpenApi;

use crate::core::component_group::ComponentGroup;
use crate::presentation::routers::component_groups::{CreateRequest, UpdateRequest};

#[derive(OpenApi)]
#[openapi(
    info(title = "Keepalive API", version = "0.1.0"),
    paths(
        crate::presentation::routers::component_groups::list,
        crate::presentation::routers::component_groups::create,
        crate::presentation::routers::component_groups::find,
        crate::presentation::routers::component_groups::update,
        crate::presentation::routers::component_groups::delete,
    ),
    components(schemas(ComponentGroup, CreateRequest, UpdateRequest))
)]
pub struct ApiDoc;

pub fn docs_router() -> Router {
    Router::new()
        .route("/openapi.json", get(|| async { Json(ApiDoc::openapi()) }))
        .route("/docs", get(|| async {
            Html(r#"<!doctype html>
<html>
  <head>
    <title>Keepalive API</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <script id="api-reference" data-url="/openapi.json"></script>
    <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
  </body>
</html>"#)
        }))
}
