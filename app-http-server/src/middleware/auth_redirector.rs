use crate::constants::COOKIES_SESSION_KEY;
use crate::extractors::AuthorizedSession;
use actix_http::Payload;
use actix_web::{
    cookie::Cookie,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::{header, StatusCode},
    Error, FromRequest, HttpResponseBuilder,
};
use app_shared::futures_util::{
    future::{ready, LocalBoxFuture, Ready},
    FutureExt,
};
use derive_builder::Builder;
use std::rc::Rc;

#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct AuthRedirectorOptions {
    pub redirect_to: String,
    #[builder(default)]
    pub ignore: Vec<String>,
    #[builder(default)]
    pub affected_paths: Vec<String>,
}

impl<S> Transform<S, ServiceRequest> for AuthRedirectorOptions
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = AuthRedirectorMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthRedirectorMiddleware {
            service: Rc::new(service),
            options: self.clone(),
        }))
    }
}

pub struct AuthRedirectorMiddleware<S> {
    service: Rc<S>,
    options: AuthRedirectorOptions,
}

impl<S> Service<ServiceRequest> for AuthRedirectorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let options = self.options.clone();

        async move {
            let (request, _) = req.parts();
            let request_path = request.path();

            if options
                .ignore
                .iter()
                .any(|path| request_path.starts_with(path))
                || request_path.starts_with(&options.redirect_to)
            {
                return service.call(req).await;
            } else if !options
                .affected_paths
                .iter()
                .any(|path| request_path.starts_with(path))
            {
                return service.call(req).await;
            }

            let session = AuthorizedSession::from_request(request, &mut Payload::None)
                .await
                .ok();

            if session.is_none() {
                let mut cookie = Cookie::new(COOKIES_SESSION_KEY, "");
                cookie.make_removal();

                let come_back = request_path;
                let redirect_url = format!("{}?redirect_to={}", options.redirect_to, come_back);
                let response = HttpResponseBuilder::new(StatusCode::TEMPORARY_REDIRECT)
                    .insert_header((header::LOCATION, redirect_url))
                    .cookie(cookie)
                    .finish();

                Ok(req.into_response(response))
            } else {
                service.call(req).await
            }
        }
        .boxed_local()
    }
}
