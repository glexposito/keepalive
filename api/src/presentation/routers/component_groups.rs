use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    core::component_group::ComponentGroup,
    infra::stores::component_group::{Store as ComponentGroupStore, UpdateInput},
    presentation::error::AppError,
};

pub fn router() -> Router<ComponentGroupStore> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(find).patch(update).delete(delete))
}

#[derive(Deserialize, ToSchema)]
pub(crate) struct CreateRequest {
    pub name: String,
    pub display_order: Option<i32>,
}

#[derive(Deserialize, ToSchema)]
pub(crate) struct UpdateRequest {
    pub name: Option<String>,
    pub display_order: Option<i32>,
}

#[utoipa::path(
    get, path = "/component-groups",
    responses((status = 200, body = Vec<ComponentGroup>)),
    tag = "Component Groups"
)]
pub(crate) async fn list(
    State(store): State<ComponentGroupStore>,
) -> Result<Json<Vec<ComponentGroup>>, AppError> {
    Ok(Json(store.find_all().await?))
}

#[utoipa::path(
    post, path = "/component-groups",
    request_body = CreateRequest,
    responses((status = 201, body = ComponentGroup)),
    tag = "Component Groups"
)]
pub(crate) async fn create(
    State(store): State<ComponentGroupStore>,
    Json(body): Json<CreateRequest>,
) -> Result<(StatusCode, Json<ComponentGroup>), AppError> {
    let group = store.insert(body.name, body.display_order.unwrap_or(0)).await?;
    Ok((StatusCode::CREATED, Json(group)))
}

#[utoipa::path(
    get, path = "/component-groups/{id}",
    params(("id" = Uuid, Path, description = "Component group ID")),
    responses((status = 200, body = ComponentGroup), (status = 404, description = "Not found")),
    tag = "Component Groups"
)]
pub(crate) async fn find(
    State(store): State<ComponentGroupStore>,
    Path(id): Path<Uuid>,
) -> Result<Json<ComponentGroup>, AppError> {
    store.find_by_id(id).await?.ok_or(AppError::NotFound).map(Json)
}

#[utoipa::path(
    patch, path = "/component-groups/{id}",
    params(("id" = Uuid, Path, description = "Component group ID")),
    request_body = UpdateRequest,
    responses((status = 200, body = ComponentGroup), (status = 404, description = "Not found")),
    tag = "Component Groups"
)]
pub(crate) async fn update(
    State(store): State<ComponentGroupStore>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateRequest>,
) -> Result<Json<ComponentGroup>, AppError> {
    store
        .update(id, UpdateInput { name: body.name, display_order: body.display_order })
        .await?
        .ok_or(AppError::NotFound)
        .map(Json)
}

#[utoipa::path(
    delete, path = "/component-groups/{id}",
    params(("id" = Uuid, Path, description = "Component group ID")),
    responses((status = 204, description = "Deleted"), (status = 404, description = "Not found")),
    tag = "Component Groups"
)]
pub(crate) async fn delete(
    State(store): State<ComponentGroupStore>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    match store.delete(id).await? {
        true => Ok(StatusCode::NO_CONTENT),
        false => Err(AppError::NotFound),
    }
}
