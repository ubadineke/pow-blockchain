use actix_web::ResponseError;

use crate::error::StateError;

#[derive(Debug, thiserror::Error)]
pub enum ApiError{
  #[error("State error: {0}")]
  State(StateError)
}

impl From<StateError> for ApiError{
  fn from(value: StateError) -> Self {
      ApiError::State(value)
  }
}

impl ResponseError for ApiError{
}