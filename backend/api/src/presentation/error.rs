use std::collections::BTreeMap;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_valid::{GardeRejection, ValidationRejection};
use http_api_problem::HttpApiProblem;
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::application::{error::UseCaseError, validation::ValidationErrors};

/// RFC 7807 problem-details document shape returned by every error response,
/// documented here so `utoipa` can render an example for `400`/`404`/`422` etc.
#[derive(Serialize, ToSchema)]
pub(crate) struct ProblemDetails {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub status: Option<u16>,
    pub title: Option<String>,
    pub detail: Option<String>,
    pub errors: Option<BTreeMap<String, Vec<String>>>,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("validation failed")]
    Validation(ValidationErrors),
    #[error("unexpected failure")]
    Unexpected,
    #[error("database error: {0}")]
    Db(#[from] DbErr),
}

impl From<UseCaseError> for AppError {
    fn from(err: UseCaseError) -> Self {
        match err {
            UseCaseError::Validation(errors) => AppError::Validation(errors),
            UseCaseError::Unexpected => AppError::Unexpected,
        }
    }
}

/// Translates `axum-valid`'s rejection — produced when `Garde<Json<T>>` fails to
/// extract or validate a request body — into the same problem shape every other
/// error in the API uses, so the boundary check stays consistent end to end.
impl From<GardeRejection<JsonRejection>> for AppError {
    fn from(rejection: GardeRejection<JsonRejection>) -> Self {
        match rejection {
            ValidationRejection::Valid(report) => AppError::Validation(report.into()),
            ValidationRejection::Inner(rejection) => AppError::BadRequest(rejection.body_text()),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let problem = match self {
            AppError::NotFound => HttpApiProblem::with_title_and_type(StatusCode::NOT_FOUND),
            AppError::BadRequest(detail) => {
                HttpApiProblem::with_title_and_type(StatusCode::BAD_REQUEST).detail(detail)
            }
            AppError::Validation(errors) => {
                HttpApiProblem::with_title_and_type(StatusCode::UNPROCESSABLE_ENTITY)
                    .value("errors", &errors.into_fields())
            }
            AppError::Unexpected | AppError::Db(_) => {
                HttpApiProblem::with_title_and_type(StatusCode::INTERNAL_SERVER_ERROR)
            }
        };

        problem.into_response()
    }
}
