use crate::extractors::AuthorizedSession;
use actix_http::Payload;
use actix_web::{
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
pub struct AuthRedirector {
    pub redirect_to: String,
    #[builder(default)]
    pub ignore: Vec<String>,
    #[builder(default)]
    pub affected_paths: Vec<String>,
}

impl<S> Transform<S, ServiceRequest> for AuthRedirector
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
            redirector: self.clone(),
        }))
    }
}

pub struct AuthRedirectorMiddleware<S> {
    service: Rc<S>,
    redirector: AuthRedirector,
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
        let redirector = self.redirector.clone();

        async move {
            let (request, _) = req.parts();
            let request_path = request.path();

            if redirector
                .ignore
                .iter()
                .any(|path| request_path.starts_with(path))
                || request_path.starts_with(&redirector.redirect_to)
            {
                return service.call(req).await;
            } else if !redirector
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
                let response = HttpResponseBuilder::new(StatusCode::TEMPORARY_REDIRECT)
                    .insert_header((header::LOCATION, redirector.redirect_to))
                    .finish();

                Ok(req.into_response(response))
            } else {
                service.call(req).await
            }
        }
        .boxed_local()
    }
}
