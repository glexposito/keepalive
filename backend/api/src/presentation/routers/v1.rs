use axum::Router;

use crate::infra::stores::component_group::Store as ComponentGroupStore;

pub mod component_groups;

pub fn router(store: ComponentGroupStore) -> Router {
    Router::new()
        .nest("/component-groups", component_groups::router())
        .with_state(store)
}
