use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    core::component_group::ComponentGroup,
    infra::stores::component_group::{Store as ComponentGroupStore, UpdateInput},
    presentation::error::AppError,
};

pub fn router() -> Router<ComponentGroupStore> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", get(find).patch(update).delete(delete))
}

async fn list(State(store): State<ComponentGroupStore>) -> Result<Json<Vec<ComponentGroup>>, AppError> {
    Ok(Json(store.find_all().await?))
}

#[derive(Deserialize)]
struct CreateRequest {
    name: String,
    display_order: Option<i32>,
}

async fn create(
    State(store): State<ComponentGroupStore>,
    Json(body): Json<CreateRequest>,
) -> Result<(StatusCode, Json<ComponentGroup>), AppError> {
    let group = store.insert(body.name, body.display_order.unwrap_or(0)).await?;
    Ok((StatusCode::CREATED, Json(group)))
}

async fn find(
    State(store): State<ComponentGroupStore>,
    Path(id): Path<Uuid>,
) -> Result<Json<ComponentGroup>, AppError> {
    store.find_by_id(id).await?.ok_or(AppError::NotFound).map(Json)
}

#[derive(Deserialize)]
struct UpdateRequest {
    name: Option<String>,
    display_order: Option<i32>,
}

async fn update(
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

async fn delete(
    State(store): State<ComponentGroupStore>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    match store.delete(id).await? {
        true => Ok(StatusCode::NO_CONTENT),
        false => Err(AppError::NotFound),
    }
}
