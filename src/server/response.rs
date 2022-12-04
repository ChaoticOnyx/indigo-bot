use crate::api::models::ServiceError;
use crate::api::ApiError;
use actix_http::StatusCode;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Response<T>
where
    T: Sized + Serialize,
{
    pub response: T,
}

impl<T> Response<T>
where
    T: Sized + Serialize,
{
    pub fn new(response: T) -> Self {
        Self { response }
    }
}

impl<T> ToString for Response<T>
where
    T: Sized + Serialize,
{
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl From<ApiError> for StatusCode {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::Other(_) => StatusCode::BAD_REQUEST,
        }
    }
}
