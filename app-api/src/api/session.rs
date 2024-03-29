use crate::Api;
use app_shared::{
    chrono::Duration,
    chrono::{DateTime, Utc},
    models::{AccountId, AnyUserId, ApiError, ApiToken, Secret, Session},
    prelude::*,
    Database,
};

impl Api {
    #[instrument]
    fn cleanup_sessions(&self) {
        trace!("cleanup_sessions");

        Database::lock(|database| database.delete_expired_sessions());
    }

    /// Продлевает сессию и создаёт новые секреты.
    #[instrument]
    pub fn extend_session(
        &self,
        session_secret: Secret,
        user_agent: String,
        ip: String,
    ) -> Result<Session, ApiError> {
        trace!("extend_session");

        self.cleanup_sessions();

        let Some(session) = Database::lock(|database| database.find_session_by_secret(session_secret.clone())) else {
            return Err(ApiError::Other("Некорректная сессия".to_string()))
        };

        self.delete_session(session_secret)?;

        self.create_session_for_account(
            AnyUserId::AccountId(session.account_id),
            Some(session.created_at),
            user_agent,
            ip,
        )
    }

    /// Удаляет сессию.
    #[instrument]
    pub fn delete_session(&self, session_secret: Secret) -> Result<(), ApiError> {
        trace!("delete_session");

        self.cleanup_sessions();

        let Some(session) = Database::lock(|database| database.find_session_by_secret(session_secret)) else {
            return Err(ApiError::Other("Некорректная сессия".to_string()))
        };

        Database::lock(|database| database.delete_session_by_secret(session.secret));
        Database::lock(|database| database.delete_api_token_by_secret(session.api_secret));

        Ok(())
    }

    /// Создаёт уникальный секрет для сессии.
    #[instrument]
    pub fn create_unique_session_secret(&self) -> Secret {
        trace!("create_unique_session_secret");

        self.cleanup_sessions();

        loop {
            let secret = Secret::new_random_session_secret();

            if Database::lock(|database| database.find_session_by_secret(secret.clone()).is_none())
            {
                break secret;
            }
        }
    }

    /// Создаёт уникальный CSRF секрет.
    #[instrument]
    pub fn create_unique_csrf_secret(&self) -> Secret {
        trace!("create_unique_csrf_secret");

        self.cleanup_sessions();

        loop {
            let secret = Secret::new_random_csrf_secret();

            if Database::lock(|database| {
                database
                    .find_session_by_csrf_secret(secret.clone())
                    .is_none()
            }) {
                break secret;
            }
        }
    }

    /// Возвращает сессию по её секрету.
    #[instrument]
    pub fn find_session_by_secret(&self, session_secret: Secret) -> Option<Session> {
        trace!("find_session_by_secret");

        self.cleanup_sessions();
        Database::lock(|database| database.find_session_by_secret(session_secret))
    }

    /// Создаёт сессию для указанного аккаунта.
    #[instrument]
    pub fn create_session_for_account(
        &self,
        user_id: AnyUserId,
        custom_creation_date: Option<DateTime<Utc>>,
        user_agent: String,
        ip: String,
    ) -> Result<Session, ApiError> {
        trace!("create_session_for_account");

        self.cleanup_sessions();
        let Some(account) = Database::lock(|database| database.find_account(user_id)) else {
            return Err(ApiError::Internal("Некорректный user_id".to_string()))
        };

        let api_token = ApiToken::new(
            self.create_unique_api_secret(),
            self.get_account_rights(account.id, None),
            None,
            Some(Duration::days(3)),
            false,
            custom_creation_date,
        );

        Database::lock(|database| database.add_api_token(api_token.clone()));

        let session = Session::new(
            self.create_unique_session_secret(),
            api_token.secret,
            self.create_unique_csrf_secret(),
            account.id,
            custom_creation_date,
            Duration::days(3),
            user_agent,
            ip,
        );

        Database::lock(|database| database.add_session(session.clone()));

        Ok(session)
    }

    #[instrument]
    pub fn is_csrf_secret_valid(&self, csrf_secret: Secret) -> bool {
        trace!("is_csrf_secret_valid");

        Database::lock(|database| database.find_session_by_csrf_secret(csrf_secret).is_some())
    }

    #[instrument]
    pub fn get_account_sessions(&self, account_id: AccountId) -> Vec<Session> {
        trace!("get_sessions_for_account");

        Database::lock(|database| database.get_account_sessions(account_id))
    }

    /// Создаёт сессию в обмен на TFA.
    #[instrument]
    pub fn create_session_by_tfa(
        &mut self,
        tfa_secret: Secret,
        user_agent: String,
        ip: String,
    ) -> Result<Session, ApiError> {
        trace!("create_session_by_tfa");

        let account = self.find_account_by_tfa_token_secret(tfa_secret)?;

        let session = self.create_session_for_account(
            AnyUserId::AccountId(account.id),
            None,
            user_agent,
            ip,
        )?;

        Ok(session)
    }
}
