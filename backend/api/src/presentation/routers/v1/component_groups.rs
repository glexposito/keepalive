use axum::{
    extract::{rejection::JsonRejection, Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use axum_valid::{Garde, GardeRejection};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    application::component_group::{create_component_group, CreateComponentGroup},
    core::component_group::ComponentGroup,
    infra::stores::component_group::{Store as ComponentGroupStore, UpdateInput},
    presentation::error::{AppError, ProblemDetails},
};

pub fn router() -> Router<ComponentGroupStore> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(find).patch(update).delete(delete))
}

#[derive(Deserialize, ToSchema, garde::Validate)]
pub(crate) struct CreateRequest {
    #[garde(custom(non_blank))]
    pub name: String,
    #[garde(range(min = 0))]
    pub display_order: Option<i32>,
}

fn non_blank(value: &str, _ctx: &()) -> garde::Result {
    match value.trim().is_empty() {
        true => Err(garde::Error::new("must not be blank")),
        false => Ok(()),
    }
}

#[derive(Deserialize, ToSchema, garde::Validate)]
pub(crate) struct UpdateRequest {
    #[garde(inner(custom(non_blank)))]
    pub name: Option<String>,
    #[garde(inner(range(min = 0)))]
    pub display_order: Option<i32>,
}

/// List component groups
///
/// Returns all component groups ordered by display order.
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

/// Create a component group
///
/// Creates a new component group with the given name and optional display order.
#[utoipa::path(
    post, path = "/component-groups",
    request_body = CreateRequest,
    responses(
        (status = 201, body = ComponentGroup),
        (status = 400, description = "Malformed request body", body = ProblemDetails),
        (status = 422, description = "Validation failed", body = ProblemDetails),
    ),
    tag = "Component Groups"
)]
pub(crate) async fn create(
    State(store): State<ComponentGroupStore>,
    body: Result<Garde<Json<CreateRequest>>, GardeRejection<JsonRejection>>,
) -> Result<(StatusCode, Json<ComponentGroup>), AppError> {
    let Garde(Json(body)) = body?;
    let group = create_component_group(
        &store,
        CreateComponentGroup { name: body.name, display_order: body.display_order },
    )
    .await?;
    Ok((StatusCode::CREATED, Json(group)))
}

/// Get a component group
///
/// Returns a single component group by its ID.
#[utoipa::path(
    get, path = "/component-groups/{id}",
    params(("id" = Uuid, Path, description = "Component group ID")),
    responses(
        (status = 200, body = ComponentGroup),
        (status = 404, description = "Not found", body = ProblemDetails),
    ),
    tag = "Component Groups"
)]
pub(crate) async fn find(
    State(store): State<ComponentGroupStore>,
    Path(id): Path<Uuid>,
) -> Result<Json<ComponentGroup>, AppError> {
    store.find_by_id(id).await?.ok_or(AppError::NotFound).map(Json)
}

/// Update a component group
///
/// Updates the name and/or display order of an existing component group.
#[utoipa::path(
    patch, path = "/component-groups/{id}",
    params(("id" = Uuid, Path, description = "Component group ID")),
    request_body = UpdateRequest,
    responses(
        (status = 200, body = ComponentGroup),
        (status = 400, description = "Malformed request body", body = ProblemDetails),
        (status = 404, description = "Not found", body = ProblemDetails),
        (status = 422, description = "Validation failed", body = ProblemDetails),
    ),
    tag = "Component Groups"
)]
pub(crate) async fn update(
    State(store): State<ComponentGroupStore>,
    Path(id): Path<Uuid>,
    body: Result<Garde<Json<UpdateRequest>>, GardeRejection<JsonRejection>>,
) -> Result<Json<ComponentGroup>, AppError> {
    let Garde(Json(body)) = body?;
    store
        .update(id, UpdateInput { name: body.name, display_order: body.display_order })
        .await?
        .ok_or(AppError::NotFound)
        .map(Json)
}

/// Delete a component group
///
/// Permanently removes a component group by its ID.
#[utoipa::path(
    delete, path = "/component-groups/{id}",
    params(("id" = Uuid, Path, description = "Component group ID")),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "Not found", body = ProblemDetails),
    ),
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
