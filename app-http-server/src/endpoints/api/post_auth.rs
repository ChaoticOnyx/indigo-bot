use actix_http::StatusCode;
use actix_web::http::header;
use actix_web::web::Json;
use actix_web::{post, HttpRequest, HttpResponseBuilder, Responder};
use serde::{Deserialize, Serialize};

use crate::constants::COOKIES_SESSION_KEY;
use crate::cookies::SessionCookie;
use crate::response::ResponseHelpers;
use app_api::Api;
use app_shared::{
    models::{ApiError, Secret, Session},
    prelude::*,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    pub tfa_secret: Secret,
}

#[post("/auth")]
pub async fn endpoint(request: HttpRequest, form: Json<Payload>) -> impl Responder {
    trace!("endpoint");

    if request.cookie(COOKIES_SESSION_KEY).is_some() {
        return ResponseHelpers::new(StatusCode::BAD_REQUEST, "Уже авторизован");
    };

    let Some(user_agent) = request
        .headers()
        .get(header::USER_AGENT) else {
        return ResponseHelpers::new(StatusCode::BAD_REQUEST, "Пустой User Agent");
    };

    let Ok(user_agent) = user_agent.to_str().map(|user_agent| user_agent.to_string()) else {
        return ResponseHelpers::new(StatusCode::BAD_REQUEST, "Некорректный User agent");
    };

    let ip = request
        .connection_info()
        .peer_addr()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|| String::new());

    let tfa = form.0.tfa_secret;
    let session: Result<Session, ApiError> =
        Api::lock_async(|api| api.create_session_by_tfa(tfa, user_agent, ip))
            .await
            .unwrap();

    match session {
        Err(err) => ResponseHelpers::from_api_error(err),
        Ok(session) => HttpResponseBuilder::new(StatusCode::OK)
            .cookie(SessionCookie::new(session))
            .finish(),
    }
}
