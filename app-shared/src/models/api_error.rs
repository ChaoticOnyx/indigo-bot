use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ApiError {
    Unauthorized(String),
    Forbidden(String),
    Other(String),
    Internal(String),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Unauthorized(msg) => f.write_str(msg),
            ApiError::Forbidden(msg) => f.write_str(msg),
            ApiError::Other(msg) => f.write_str(msg),
            ApiError::Internal(msg) => f.write_str(msg),
        }
    }
}
