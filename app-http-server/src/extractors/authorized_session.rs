use crate::constants::COOKIES_SESSION_KEY;
use actix_http::Payload;
use actix_web::{error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use app_api::Api;
use app_shared::{
    futures_util::future::LocalBoxFuture,
    futures_util::FutureExt,
    models::{Secret, Session},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct AuthorizedSession(pub Session);

impl FromRequest for AuthorizedSession {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        trace!("from_request");

        let req = req.clone();

        async move {
            let Some(session_secret) = req.cookie(COOKIES_SESSION_KEY).map(|cookie| Secret(cookie.value().to_string())) else {
                return Err(ErrorUnauthorized("has no session"))
            };

            let session: Option<Session> =
                Api::lock_async(|api| api.private_api.find_session_by_secret(session_secret)).await.unwrap();

            let Some(session) = session else {
                return Err(ErrorUnauthorized("invalid session"))
            };

            if session.is_expired() {
                return Err(ErrorUnauthorized("expired session"));
            }

            Ok(AuthorizedSession(session))
        }
        .boxed_local()
    }
}
