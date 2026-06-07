use thiserror::Error;

use crate::application::validation::ValidationErrors;

/// Shared outcome of a use case's preconditions failing. Deliberately opaque
/// about *why* something unexpected happened — the use case shouldn't need to
/// know or care which infrastructure produced the failure; that's presentation's
/// job to log and turn into a 500.
#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("validation failed")]
    Validation(ValidationErrors),
    #[error("unexpected failure")]
    Unexpected,
}
