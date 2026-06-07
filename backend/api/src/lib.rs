pub mod application;
pub mod core;
pub mod infra;
pub mod presentation;

use axum::Router;
use infra::stores::component_group::Store as ComponentGroupStore;

pub fn app(store: ComponentGroupStore) -> Router {
    Router::new()
        .nest("/api/v1", presentation::routers::v1::router(store))
        .merge(presentation::docs::docs_router())
}
