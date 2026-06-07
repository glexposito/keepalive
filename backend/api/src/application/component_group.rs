use crate::{
    application::error::UseCaseError, core::component_group::ComponentGroup,
    infra::stores::component_group::Store,
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
