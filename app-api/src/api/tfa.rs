use crate::Api;
use app_shared::{
    chrono::Duration,
    models::{AnyUserId, ApiCaller, ApiError, Secret, TFAToken},
    prelude::*,
    Database,
};

impl Api {
    /// Возвращает существующий или создаёт новый TFA токен для аккаунта.
    #[instrument]
    pub fn get_or_create_tfa_for_account(
        &mut self,
        caller: ApiCaller,
        discord_user_id: DiscordUserId,
    ) -> Result<TFAToken, ApiError> {
        trace!("get_or_create_tfa_for_account");

        if Database::lock(|database| {
            database
                .find_account(AnyUserId::DiscordId(discord_user_id))
                .is_none()
        }) {
            let discord_user = self
                .discord_api
                .get_discord_user(discord_user_id)
                .ok_or_else(|| ApiError::Other("Пользователя не существует".to_string()))?;

            self.create_account(
                caller,
                discord_user.name.clone(),
                discord_user
                    .avatar_url()
                    .unwrap_or_else(|| discord_user.default_avatar_url()),
                discord_user_id,
            )?;
        }

        match self.tokens_storage.find_by_discord_user_id(discord_user_id) {
            None => {
                debug!("existing token not found");

                Ok(self
                    .tokens_storage
                    .new_token(discord_user_id, Duration::seconds(60)))
            }
            Some(token) => {
                debug!("existing token found");
                Ok(token.clone())
            }
        }
    }

    /// Возвращает TFA токен по его секрету.
    #[instrument]
    pub fn find_tfa_token_by_secret(&mut self, secret: Secret) -> Option<TFAToken> {
        trace!("find_tfa_token_by_secret");

        self.tokens_storage.find_by_secret(secret).cloned()
    }
}
