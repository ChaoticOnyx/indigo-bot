use std::collections::BTreeMap;

use app_shared::{
    models::{ServiceError, ServiceId, WebhookConfiguration, WebhookPayload, WebhookResponse},
    prelude::*,
    tokio::runtime::Runtime,
};

use super::{ChatToDiscordService, EchoService, Service};
use crate::services::RoundEndService;

#[derive(Debug)]
pub struct ServicesStorage {
    services: BTreeMap<ServiceId, Box<dyn Service>>,
    rt: Runtime,
}

impl ServicesStorage {
    #[instrument(skip(self))]
    pub fn register(&mut self) {
        trace!("register");

        self.services
            .insert(ServiceId("echo".to_string()), Box::<EchoService>::default());

        self.services.insert(
            ServiceId("chat_to_discord".to_string()),
            Box::<ChatToDiscordService>::default(),
        );

        self.services.insert(
            ServiceId("round_end".to_string()),
            Box::<RoundEndService>::default(),
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
        service_id: &ServiceId,
        configuration: &WebhookConfiguration,
    ) -> Result<(), ServiceError> {
        trace!("configure_webhook");

        let service = self.services.get(service_id).unwrap();

        self.rt
            .block_on(async { service.configure(configuration).await })
    }

    #[instrument(skip(self))]
    pub fn handle(
        &self,
        service_id: &ServiceId,
        configuration: &WebhookConfiguration,
        payload: &WebhookPayload,
    ) -> Result<WebhookResponse, ServiceError> {
        trace!("handle");

        let service = self.services.get(service_id).unwrap();

        self.rt
            .block_on(async { service.handle(configuration, payload).await })
    }
}

impl Default for ServicesStorage {
    fn default() -> Self {
        Self {
            services: BTreeMap::new(),
            rt: app_shared::tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap(),
        }
    }
}
