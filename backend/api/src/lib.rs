pub mod core;
pub mod infra;
pub mod presentation;

use axum::Router;
use infra::stores::component_group::Store as ComponentGroupStore;

pub fn app(store: ComponentGroupStore) -> Router {
    let api = Router::new()
        .nest("/component-groups", presentation::routers::component_groups::router())
        .with_state(store);

    Router::new()
        .merge(api)
        .merge(presentation::docs::docs_router())
}
