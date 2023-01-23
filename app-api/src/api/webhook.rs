use crate::{Api, Journal};
use app_macros::validate_api_secret;
use app_shared::prelude::*;
use app_shared::{
    models::{
        ActionType, Actor, ApiCaller, ApiError, Secret, ServiceError, ServiceId, Webhook,
        WebhookConfiguration, WebhookPayload, WebhookResponse,
    },
    Database,
};

impl Api {
    /// Создаёт уникальный секрет для webhook.
    #[instrument]
    pub fn create_unique_webhook_secret(&self) -> Secret {
        trace!("create_unique_webhook_secret");

        loop {
            let secret = Secret::new_random_webhook_secret();

            if Database::lock(|database| database.find_webhook_by_secret(secret.clone()).is_none())
            {
                break secret;
            }
        }
    }

    /// Создаёт вебхук.
    #[instrument]
    pub fn create_webhook(
        &self,
        caller: ApiCaller,
        target: ServiceId,
        name: String,
        configuration: WebhookConfiguration,
    ) -> Result<Webhook, ApiError> {
        trace!("create_webhook");

        let mut actor = Actor::System;

        if let ApiCaller::Token(secret) = caller {
            let token = validate_api_secret!(secret);

            if !token
                .rights
                .service
                .can_create_webhooks_for_service(&target)
            {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            }

            actor = if let Some(account_id) = token.creator {
                Actor::User(account_id)
            } else {
                Actor::System
            };
        }

        if !self.services_storage.is_service_exists(&target) {
            return Err(ApiError::Other("Некорректный сервис".to_string()));
        }

        if name.trim().is_empty() {
            return Err(ApiError::Other("Пустое название вебхука".to_string()));
        }

        match self
            .services_storage
            .configure_webhook(&target, &configuration)
        {
            Ok(_) => (),
            Err(err) => return Err(ApiError::Other(format!("Некорректная конфигурация: {err}"))),
        }

        let webhook = Webhook::new(
            name,
            self.create_unique_webhook_secret(),
            target,
            None,
            configuration,
        );

        Database::lock(|database| database.add_webhook(webhook.clone()));

        Journal::lock(|journal| {
            journal.log(
                actor,
                Some(Actor::Webhook(webhook.name.clone())),
                ActionType::WebhookCreated,
            )
        });

        Ok(webhook)
    }

    /// Удаляет вебхук.
    #[instrument]
    pub fn delete_webhook(
        &self,
        caller: ApiCaller,
        webhook_secret: Secret,
    ) -> Result<(), ApiError> {
        trace!("delete_webhook");

        let mut actor = Actor::System;
        let webhook = Database::lock(|database| database.find_webhook_by_secret(webhook_secret));

        if let ApiCaller::Token(secret) = caller {
            let token = validate_api_secret!(secret);

            // Никакого брутфорса вебхуков без прав!
            if !token.rights.service.can_delete_webhooks_at_all() {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            }

            let Some(ref webhook) = webhook else {
				return Err(ApiError::Other("Некорректный вебхук".to_string()))
			};

            if !token
                .rights
                .service
                .can_delete_webhooks_for_service(&webhook.service_id)
            {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            }

            actor = if let Some(account_id) = token.creator {
                Actor::User(account_id)
            } else {
                Actor::System
            };
        }

        let Some(webhook) = webhook else {
			return Err(ApiError::Other("Некорректный вебхук".to_string()))
		};

        Database::lock(|database| database.delete_webhook_by_secret(webhook.secret));

        Journal::lock(|journal| {
            journal.log(
                actor,
                Some(Actor::Webhook(webhook.name)),
                ActionType::WebhookDeleted,
            )
        });

        Ok(())
    }

    /// Обрабатывает событие от вебхуков.
    #[instrument]
    pub fn handle_webhook(
        &self,
        webhook_secret: Secret,
        payload: WebhookPayload,
    ) -> Result<WebhookResponse, ServiceError> {
        trace!("handle_webhook");

        let webhook = Database::lock(|database| database.find_webhook_by_secret(webhook_secret));

        let Some(webhook) = webhook else {
            return Err(ServiceError::Any("Некорректный вебхук".to_string()))
        };

        self.services_storage
            .handle(&webhook.service_id, &webhook.configuration, &payload)
    }
}
