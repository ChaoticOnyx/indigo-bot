use crate::cookies::SessionCookie;
use crate::extractors::AuthenticatedUser;
use actix_http::Payload;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, FromRequest,
};
use app_api::Api;
use app_shared::{
    chrono::Duration,
    futures_util::future::LocalBoxFuture,
    futures_util::future::{ready, Ready},
    prelude::*,
    futures_util::FutureExt,
    chrono::Utc
};
use derive_builder::Builder;
use std::rc::Rc;

#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct SessionExtenderOptions {
    pub extend_before: Duration,
}

impl<S> Transform<S, ServiceRequest> for SessionExtenderOptions
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = SessionExtenderMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SessionExtenderMiddleware {
            service: Rc::new(service),
            options: self.clone(),
        }))
    }
}

pub struct SessionExtenderMiddleware<S> {
    service: Rc<S>,
    options: SessionExtenderOptions,
}

impl<S> Service<ServiceRequest> for SessionExtenderMiddleware<S>
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
            let Some(user) = AuthenticatedUser::from_request(request, &mut Payload::None)
                .await
                .ok() else {
                
                return service.call(req).await;
            };

            if !user.session.is_expired() && user.session.expiration - Utc::now() > options.extend_before {
                return service.call(req).await;
            }

            let Some(user_agent) = request
                .headers()
                .get(header::USER_AGENT) else {
                return service.call(req).await;
            };

            let Ok(user_agent) = user_agent.to_str().map(|user_agent| user_agent.to_string()) else {
                return service.call(req).await;
            };

            let ip = request
                .connection_info()
                .peer_addr()
                .map(|ip| ip.to_string())
                .unwrap_or_else(String::new);

            let mut response = service.call(req).await?;
            
            let Ok(new_session) = Api::lock_async(|api| {
                api.extend_session(user.session.secret, user_agent, ip)
            }).await.unwrap() else {
                return Ok(response);
            };

            response
                .response_mut()
                .add_cookie(&SessionCookie::from_session(new_session))
                .unwrap();

            Ok(response)
        }
        .boxed_local()
    }
}
