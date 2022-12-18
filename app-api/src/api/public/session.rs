use crate::Api;
use app_shared::models::{AnyUserId, ApiError, Secret, Session};
use app_shared::prelude::*;

impl Api {
    /// Создаёт сессию в обмен на TFA.
    #[instrument]
    pub async fn create_session_by_tfa(&self, tfa_secret: Secret) -> Result<Session, ApiError> {
        trace!("create_session_by_tfa");

        let Some(account) = self.private_api.find_account_by_tfa_token_secret(tfa_secret).await else {
            return Err(ApiError::Other("invalid tfa token".to_string()))
        };

        let session = self
            .private_api
            .create_session_for_account(AnyUserId::AccountId(account.id))
            .await?;

        Ok(session)
    }
}
