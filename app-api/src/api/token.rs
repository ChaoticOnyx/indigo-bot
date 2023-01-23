use crate::{Api, Journal};
use app_macros::validate_api_secret;
use app_shared::{
    chrono::Duration,
    models::{ActionType, Actor, ApiCaller, ApiError, ApiToken, Rights, Secret, TokenRights},
    prelude::*,
    Database,
};

impl Api {
    /// Создаёт API токен.
    #[instrument]
    pub fn create_api_token(
        &self,
        caller: ApiCaller,
        rights: Rights,
        duration: Option<Duration>,
        is_service: bool,
    ) -> Result<ApiToken, ApiError> {
        trace!("create_api_token");

        let mut actor = Actor::System;
        let mut creator = None;

        if let ApiCaller::Token(secret) = caller {
            let token = validate_api_secret!(secret);

            if (is_service
                && !token
                    .rights
                    .token
                    .contains(TokenRights::SERVICE_TOKEN_CREATE))
                || (!is_service && !token.rights.token.contains(TokenRights::TOKEN_CREATE))
            {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            }

            if token.rights < rights {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            }

            actor = if let Some(account_id) = token.creator {
                creator = Some(account_id);
                Actor::User(account_id)
            } else {
                Actor::System
            };
        }

        let new_token = ApiToken::new(
            self.create_unique_api_secret(),
            rights,
            creator,
            duration,
            is_service,
            None,
        );

        if new_token.is_expired() {
            return Err(ApiError::Other("Новый токен уже устаревший".to_string()));
        }

        Database::lock(|database| database.add_api_token(new_token.clone()));

        Journal::lock(|journal| {
            journal.log(actor, creator.map(Actor::User), ActionType::ApiTokenCreated)
        });

        Ok(new_token)
    }

    /// Удаляет API токен.
    #[instrument]
    pub fn delete_api_token(&self, caller: ApiCaller, target: Secret) -> Result<(), ApiError> {
        trace!("delete_api_token");

        let mut actor = Actor::System;
        let target_token = Database::lock(|database| database.find_api_token_by_secret(target));

        if let ApiCaller::Token(secret) = caller {
            let token = validate_api_secret!(secret);

            // Никакого брутфорса токенов без прав!
            if !token.rights.token.contains(TokenRights::TOKEN_DELETE)
                && token
                    .rights
                    .token
                    .contains(TokenRights::SERVICE_TOKEN_DELETE)
            {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            }

            let Some(ref target_token) = target_token else {
            	return Err(ApiError::Other("Целевой токен не существует".to_string()))
        	};

            if token.rights < target_token.rights {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            };

            if (target_token.is_service
                && !token
                    .rights
                    .token
                    .contains(TokenRights::SERVICE_TOKEN_DELETE))
                || (!target_token.is_service
                    && !token.rights.token.contains(TokenRights::TOKEN_DELETE))
            {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            }

            actor = if let Some(account_id) = token.creator {
                Actor::User(account_id)
            } else {
                Actor::System
            };
        }

        let Some(target_token) = target_token else {
			return Err(ApiError::Other("Целевой токен не существует".to_string()))
		};

        Database::lock(|database| database.delete_api_token_by_secret(target_token.secret));

        Journal::lock(|journal| {
            journal.log(
                actor,
                target_token.creator.map(Actor::User),
                ActionType::ApiTokenDeleted,
            )
        });

        Ok(())
    }

    /// Создаёт уникальный секрет для Api токена.
    #[instrument]
    pub fn create_unique_api_secret(&self) -> Secret {
        trace!("create_unique_api_secret");

        loop {
            let secret = Secret::new_random_api_secret();

            if Database::lock(|database| {
                database.find_api_token_by_secret(secret.clone()).is_none()
            }) {
                break secret;
            }
        }
    }
}
