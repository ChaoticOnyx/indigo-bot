use std::collections::BTreeMap;

use app_shared::{
    models::{ServiceError, ServiceId, WebhookConfiguration, WebhookPayload, WebhookResponse},
    prelude::*,
    tokio::runtime::Runtime,
};

use crate::services::RoundEndService;
use crate::Api;

use super::{ChatToDiscordService, EchoService, Service};

#[derive(Debug)]
pub struct ServicesStorage {
    services: BTreeMap<ServiceId, Box<dyn Service>>,
    rt: Runtime,
}

impl ServicesStorage {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            rt: app_shared::tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap(),
        }
    }

    #[instrument(skip(self))]
    pub fn register(&mut self) {
        trace!("register");

        self.services.insert(
            ServiceId("echo".to_string()),
            Box::new(EchoService::default()),
        );

        self.services.insert(
            ServiceId("chat_to_discord".to_string()),
            Box::new(ChatToDiscordService::default()),
        );

        self.services.insert(
            ServiceId("round_end".to_string()),
            Box::new(RoundEndService::default()),
        );
    }

    #[instrument(skip(self))]
    pub fn is_service_exists(&self, service_id: &ServiceId) -> bool {
        trace!("is_service_exists");

        self.services.contains_key(service_id)
    }

    #[instrument(skip(self))]
    pub fn configure_webhook(
        &self,
        api: &Api,
        service_id: &ServiceId,
        configuration: &WebhookConfiguration,
    ) -> Result<(), ServiceError> {
        trace!("configure_webhook");

        let service = self.services.get(service_id).unwrap();

        self.rt
            .block_on(async { service.configure(configuration, api).await })
    }

    #[instrument(skip(self))]
    pub fn handle(
        &self,
        api: &Api,
        service_id: &ServiceId,
        configuration: &WebhookConfiguration,
        payload: &WebhookPayload,
    ) -> Result<WebhookResponse, ServiceError> {
        trace!("handle");

        let service = self.services.get(service_id).unwrap();
        let result = self
            .rt
            .block_on(async { service.handle(configuration, payload, api).await });

        debug!("{result:#?}");

        result
    }
}
