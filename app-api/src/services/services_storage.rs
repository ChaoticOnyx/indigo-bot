use std::collections::BTreeMap;

use app_shared::{
    models::{ServiceError, ServiceId, WebhookConfiguration, WebhookPayload, WebhookResponse},
    prelude::*,
};

use crate::Api;

use super::{ChatToDiscordService, EchoService, Service};

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

        self.services.insert(
            ServiceId("chat_to_discord".to_string()),
            Box::new(ChatToDiscordService::default()),
        );
    }

    #[instrument(skip(self))]
    pub fn is_service_exists(&self, service_id: &ServiceId) -> bool {
        self.services.contains_key(service_id)
    }

    #[instrument(skip(self))]
    pub async fn configure_webhook(
        &self,
        api: &Api,
        service_id: &ServiceId,
        configuration: &WebhookConfiguration,
    ) -> Result<(), ServiceError> {
        debug!("configure_webhook");

        let service = self.services.get(service_id).unwrap();
        service.configure(configuration, api).await
    }

    #[instrument(skip(self))]
    pub async fn handle(
        &self,
        api: &Api,
        service_id: &ServiceId,
        configuration: &WebhookConfiguration,
        payload: &WebhookPayload,
    ) -> Result<WebhookResponse, ServiceError> {
        debug!("handle");

        let service = self.services.get(service_id).unwrap();
        service.handle(configuration, payload, api).await
    }
}
