use crate::Api;
use app_shared::models::{AnyUserId, ApiError, Secret, Session};
use app_shared::prelude::*;

impl Api {
    /// Создаёт сессию в обмен на TFA.
    #[instrument]
    pub fn create_session_by_tfa(
        &self,
        tfa_secret: Secret,
        user_agent: String,
        ip: String,
    ) -> Result<Session, ApiError> {
        trace!("create_session_by_tfa");

        let Some(account) = self.private_api.find_account_by_tfa_token_secret(tfa_secret) else {
            return Err(ApiError::Other("Некорректный TFA токен".to_string()))
        };

        let session = self.private_api.create_session_for_account(
            AnyUserId::AccountId(account.id),
            None,
            user_agent,
            ip,
        )?;

        Ok(session)
    }

    /// Продлевает сессию и создаёт новый секрет.
    #[instrument]
    pub fn extend_session(
        &self,
        session_secret: Secret,
        user_agent: String,
        ip: String,
    ) -> Result<Session, ApiError> {
        trace!("extend_session");

        self.private_api
            .extend_session(session_secret, user_agent, ip)
    }

    #[instrument]
    pub fn delete_session(&self, session_secret: Secret) -> Result<(), ApiError> {
        trace!("delete_session");

        self.private_api.delete_session(session_secret)
    }
}
