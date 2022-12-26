use actix_http::Payload;
use crate::constants::COOKIES_SESSION_KEY;
use actix_web::{error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};
use app_api::Api;
use app_shared::{
    futures_util::future::LocalBoxFuture,
    futures_util::FutureExt,
    models::{Account, Secret, Session},
    prelude::*,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub session: Session,
    pub account: Account
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        trace!("from_request");

        let req = req.clone();

        async move {
            let Some(session_secret) = req.cookie(COOKIES_SESSION_KEY).map(|cookie| Secret(cookie.value().to_string())) else {
                return Err(ErrorUnauthorized("Отсутствует сессия"))
            };

            let user: Option<AuthenticatedUser> =
                Api::lock_async(|api| {
                    let session = api.private_api.find_session_by_secret(session_secret.clone())?;
                    let account = api.find_account_by_session(session_secret).ok()?;
                    
                    Some(AuthenticatedUser {
                        session,
                        account
                    })
                }).await.unwrap();

            let Some(user) = user else {
                return Err(ErrorUnauthorized("Некорректная сессия"))
            };

            if user.session.is_expired() {
                return Err(ErrorUnauthorized("Сессия просрочена"));
            }

            Ok(user)
        }
        .boxed_local()
    }
}
