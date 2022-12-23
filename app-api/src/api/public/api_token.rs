use crate::Api;
use app_macros::validate_api_secret;
use app_shared::{
    chrono::Duration,
    models::{ApiError, ApiToken, Rights, Secret, TokenRights},
    prelude::*,
};

impl Api {
    #[instrument]
    pub fn create_api_token(
        &self,
        api_secret: Secret,
        rights: Rights,
        duration: Option<Duration>,
        is_service: bool,
    ) -> Result<ApiToken, ApiError> {
        trace!("create_api_token");

        let token = validate_api_secret!(api_secret);

        if (is_service
            && !token
                .rights
                .token
                .contains(TokenRights::SERVICE_TOKEN_CREATE))
            || (!is_service && !token.rights.token.contains(TokenRights::TOKEN_CREATE))
        {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        if token.rights < rights {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        let new_token = ApiToken::new(
            self.private_api.create_unique_api_secret(),
            rights,
            token.creator,
            duration,
            is_service,
        );

        if new_token.is_expired() {
            return Err(ApiError::Other("new token is already expired".to_string()));
        }

        self.private_api.database.add_api_token(new_token.clone());

        Ok(new_token)
    }

    #[instrument]
    pub fn delete_api_token(&self, api_secret: Secret, target: Secret) -> Result<(), ApiError> {
        trace!("delete_api_token");

        let token = validate_api_secret!(api_secret);
        let target_token = self.private_api.database.find_api_token_by_secret(target);

        let Some(target_token) = target_token else {
            return Err(ApiError::Other("target token does not exist".to_string()))
        };

        if token.rights < target_token.rights {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        };

        if (target_token.is_service
            && !token
                .rights
                .token
                .contains(TokenRights::SERVICE_TOKEN_DELETE))
            || (!target_token.is_service && !token.rights.token.contains(TokenRights::TOKEN_DELETE))
        {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        self.private_api
            .database
            .delete_api_token_by_secret(target_token.secret);

        Ok(())
    }
}
