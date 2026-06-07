use uuid::Uuid;

use crate::{
    application::error::UseCaseError,
    core::component_group::ComponentGroup,
    infra::stores::component_group::{Store, UpdateInput},
};

pub struct CreateComponentGroup {
    pub name: String,
    pub display_order: Option<i32>,
}

pub async fn create_component_group(
    store: &Store,
    input: CreateComponentGroup,
) -> Result<ComponentGroup, UseCaseError> {
    let name = input.name.trim().to_string();
    store
        .insert(name, input.display_order.unwrap_or(0))
        .await
        .map_err(|_| UseCaseError::Unexpected)
}

pub struct UpdateComponentGroup {
    pub name: Option<String>,
    pub display_order: Option<i32>,
}

pub async fn update_component_group(
    store: &Store,
    id: Uuid,
    input: UpdateComponentGroup,
) -> Result<Option<ComponentGroup>, UseCaseError> {
    let name = input.name.map(|name| name.trim().to_string());
    store
        .update(id, UpdateInput { name, display_order: input.display_order })
        .await
        .map_err(|_| UseCaseError::Unexpected)
}
