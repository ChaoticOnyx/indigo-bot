use crate::constants::COOKIES_SESSION_KEY;
use actix_web::cookie::time::OffsetDateTime;
use actix_web::cookie::{Cookie, SameSite};
use app_shared::models::Session;

pub struct SessionCookie;

impl SessionCookie {
    pub fn new(session: Session) -> Cookie<'static> {
        let mut session_cookie = Cookie::new(COOKIES_SESSION_KEY, session.secret.0);
        let expiration =
            OffsetDateTime::from_unix_timestamp(session.expiration.timestamp()).unwrap();

        session_cookie.set_expires(Some(expiration));
        session_cookie.set_http_only(true);
        session_cookie.set_secure(true);
        session_cookie.set_same_site(SameSite::Strict);
        session_cookie.set_path("/");

        session_cookie
    }
}
