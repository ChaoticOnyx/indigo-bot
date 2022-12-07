use actix_http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use app_api::ApiError;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ResponseHelpers;

impl ResponseHelpers {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>(status: StatusCode, response: T) -> HttpResponse
    where
        T: Sized + Serialize,
    {
        HttpResponseBuilder::new(status).json(response)
    }

    pub fn from_api_error(error: ApiError) -> HttpResponse {
        match error {
            ApiError::Unauthorized(err) => {
                HttpResponseBuilder::new(StatusCode::UNAUTHORIZED).json(err)
            }
            ApiError::Forbidden(err) => HttpResponseBuilder::new(StatusCode::FORBIDDEN).json(err),
            ApiError::Other(err) => HttpResponseBuilder::new(StatusCode::BAD_REQUEST).json(err),
        }
    }

    pub fn from_api_result<T>(result: Result<T, ApiError>) -> HttpResponse
    where
        T: Sized + Serialize,
    {
        match result {
            Ok(result) => HttpResponseBuilder::new(StatusCode::OK).json(result),
            Err(err) => Self::from_api_error(err),
        }
    }
}
