use axum::{extract::FromRef, Router};

use crate::infra::stores::component_group::Store as ComponentGroupStore;

pub mod component_groups;

/// `axum-valid`'s `Garde<Json<T>>` extractor needs a validation context, and
/// since none of our request DTOs use one (`type Context = ()`), it must be
/// derivable from the router state — see the `Garde` docs for details.
impl FromRef<ComponentGroupStore> for () {
    fn from_ref(_state: &ComponentGroupStore) -> Self {}
}

pub fn router(store: ComponentGroupStore) -> Router {
    Router::new()
        .nest("/component-groups", component_groups::router())
        .with_state(store)
}
