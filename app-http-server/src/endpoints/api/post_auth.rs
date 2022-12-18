use actix_http::StatusCode;
use actix_session::Session as Cookies;
use actix_web::http::header;
use actix_web::web::{Form, Query};
use actix_web::{post, HttpResponseBuilder, Responder};
use serde::{Deserialize, Serialize};

use crate::constants::COOKIES_SESSION_KEY;
use crate::response::ResponseHelpers;
use app_api::Api;
use app_macros::tokio_blocking;
use app_shared::{models::Secret, prelude::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormData {
    pub tfa_secret: Secret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectData {
    pub redirect_to: String,
}

#[instrument(skip(cookies))]
#[post("/auth")]
pub async fn endpoint(
    redirect: Option<Query<RedirectData>>,
    form: Form<FormData>,
    cookies: Cookies,
) -> impl Responder {
    trace!("endpoint");

    if cookies
        .get::<String>(COOKIES_SESSION_KEY)
        .unwrap()
        .is_some()
    {
        return ResponseHelpers::new(StatusCode::BAD_REQUEST, "already authorized");
    }

    let tfa = form.0.tfa_secret;
    let session = Api::lock(tokio_blocking!(|api| {
        api.create_session_by_tfa(tfa).await
    }));

    match session {
        Err(err) => ResponseHelpers::from_api_error(err),
        Ok(session) => {
            cookies.insert(COOKIES_SESSION_KEY, session.secret).unwrap();

            match redirect {
                None => HttpResponseBuilder::new(StatusCode::TEMPORARY_REDIRECT)
                    .insert_header((header::LOCATION, "/"))
                    .finish(),
                Some(redirect) => HttpResponseBuilder::new(StatusCode::TEMPORARY_REDIRECT)
                    .insert_header((header::LOCATION, redirect.0.redirect_to))
                    .finish(),
            }
        }
    }
}
