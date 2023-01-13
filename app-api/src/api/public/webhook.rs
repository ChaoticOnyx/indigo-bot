use crate::Api;
use app_macros::validate_api_secret;
use app_shared::{
    models::{
        ApiError, Secret, ServiceError, ServiceId, Webhook, WebhookConfiguration, WebhookPayload,
        WebhookResponse,
    },
    prelude::*,
};

impl Api {
    #[instrument]
    pub fn create_webhook(
        &self,
        api_secret: Secret,
        target: ServiceId,
        name: String,
        configuration: WebhookConfiguration,
    ) -> Result<Webhook, ApiError> {
        trace!("create_webhook");

        let token = validate_api_secret!(api_secret);

        if !token
            .rights
            .service
            .can_create_webhooks_for_service(&target)
        {
            return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
        }

        if !self.private_api.services_storage.is_service_exists(&target) {
            return Err(ApiError::Other("Некорректный сервис".to_string()));
        }

        if name.trim().is_empty() {
            return Err(ApiError::Other("Пустое название вебхука".to_string()));
        }

        match self
            .private_api
            .services_storage
            .configure_webhook(self, &target, &configuration)
        {
            Ok(_) => (),
            Err(err) => return Err(ApiError::Other(format!("Некорректная конфигурация: {err}"))),
        }

        let webhook = Webhook::new(
            name,
            self.private_api.create_unique_webhook_secret(),
            target,
            None,
            configuration,
        );
        self.private_api.database.add_webhook(webhook.clone());

        Ok(webhook)
    }

    #[instrument]
    pub fn delete_webhook(
        &self,
        api_secret: Secret,
        webhook_secret: Secret,
    ) -> Result<(), ApiError> {
        trace!("delete_webhook");

        let token = validate_api_secret!(api_secret);

        // Никакого брутфорса вебхуков без прав!
        if !token.rights.service.can_delete_webhooks_at_all() {
            return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
        }

        let webhook = self
            .private_api
            .database
            .find_webhook_by_secret(webhook_secret);

        let Some(webhook) = webhook else {
            return Err(ApiError::Other("Некорректный вебхук".to_string()))
        };

        if !token
            .rights
            .service
            .can_delete_webhooks_for_service(&webhook.service_id)
        {
            return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
        }

        self.private_api
            .database
            .delete_webhook_by_secret(webhook.secret);

        Ok(())
    }

    #[instrument]
    pub fn handle_webhook(
        &self,
        webhook_secret: Secret,
        payload: WebhookPayload,
    ) -> Result<WebhookResponse, ServiceError> {
        trace!("handle_webhook");

        let webhook = self
            .private_api
            .database
            .find_webhook_by_secret(webhook_secret);

        let Some(webhook) = webhook else {
            return Err(ServiceError::Any("Некорректный вебхук".to_string()))
        };

        self.private_api.services_storage.handle(
            self,
            &webhook.service_id,
            &webhook.configuration,
            &payload,
        )
    }
}
