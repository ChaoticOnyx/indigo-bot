use crate::api::models::{ServiceError, ServiceId, WebhookPayload, WebhookResponse};
use crate::api::services::echo_service::EchoService;
use crate::api::Service;
use std::collections::BTreeMap;

use crate::prelude::*;

#[derive(Debug)]
pub struct ServicesStorage {
    services: BTreeMap<ServiceId, Box<dyn Service>>,
}

impl ServicesStorage {
    #[instrument]
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
        }
    }

    #[instrument(skip(self))]
    pub fn register(&mut self) {
        self.services.insert(
            ServiceId("echo".to_string()),
            Box::new(EchoService::default()),
        );
    }

    #[instrument(skip(self))]
    pub fn is_service_exists(&self, service_id: &ServiceId) -> bool {
        self.services.contains_key(service_id)
    }

    #[instrument(skip(self))]
    pub async fn handle(
        &self,
        api: &Api,
        service_id: &ServiceId,
        payload: WebhookPayload,
    ) -> Result<WebhookResponse, ServiceError> {
        let service = self.services.get(service_id).unwrap();

        service.handle(payload, api).await
    }
}
