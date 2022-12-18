use crate::api::private::PrivateApi;
use app_shared::{
    chrono::Duration,
    models::{AnyUserId, ApiError, ApiToken, Secret, Session},
    prelude::*,
};

impl PrivateApi {
    /// Создаёт уникальный секрет для сессии.
    #[instrument]
    pub async fn create_unique_session_secret(&self) -> Secret {
        trace!("create_unique_session_secret");

        let new_secret = loop {
            let secret = Secret::new_random_session_secret();

            if self
                .database
                .find_session_by_secret(secret.clone())
                .await
                .is_none()
            {
                break secret;
            }
        };

        new_secret
    }

    /// Возвращает сессию по её секрету.
    #[instrument]
    pub async fn find_session_by_secret(&self, session_secret: Secret) -> Option<Session> {
        trace!("find_session_by_secret");

        self.database.find_session_by_secret(session_secret).await
    }

    /// Создаёт сессию для указанного аккаунта.
    #[instrument]
    pub async fn create_session_for_account(
        &self,
        user_id: AnyUserId,
    ) -> Result<Session, ApiError> {
        trace!("create_session_for_account");

        let Some(account) = self.database.find_account(user_id).await else {
            return Err(ApiError::Internal("invalid user_id".to_string()))
        };

        let api_token = ApiToken::new(
            self.create_unique_api_secret().await,
            self.get_account_rights(AnyUserId::AccountId(account.id), None)
                .await,
            None,
            Some(Duration::days(3)),
            false,
        );

        self.database.add_api_token(api_token.clone()).await;

        let session = Session::new(
            self.create_unique_session_secret().await,
            api_token.secret,
            account.id,
            None,
            Duration::days(3),
            String::new(),
            String::new(),
        );

        self.database.create_session(session.clone()).await;

        Ok(session)
    }
}
