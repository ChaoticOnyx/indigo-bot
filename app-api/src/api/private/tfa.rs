use crate::api::private::PrivateApi;
use app_shared::{
    chrono::{Duration, Utc},
    models::{AnyUserId, Secret, TFAToken},
    prelude::*,
};

impl PrivateApi {
    /// Возвращает существующий или создаёт новый TFA токен для аккаунта.
    #[instrument]
    pub fn get_or_create_tfa_for_account(&mut self, discord_user_id: DiscordUserId) -> TFAToken {
        trace!("get_or_create_tfa_for_account");

        if self
            .database
            .find_account(AnyUserId::DiscordId(discord_user_id))
            .is_none()
        {
            self.database.add_account(discord_user_id, Utc::now(), &[]);
        }

        self.tokens_storage.remove_expired_tokens();

        match self.tokens_storage.find_by_discord_user_id(discord_user_id) {
            None => {
                debug!("existing token not found");

                self.tokens_storage
                    .new_token(discord_user_id, Duration::seconds(60))
            }
            Some(token) => {
                debug!("existing token found");
                token.clone()
            }
        }
    }

    /// Возвращает TFA токен по его секрету.
    #[instrument]
    pub fn find_tfa_token_by_secret(&self, secret: Secret) -> Option<TFAToken> {
        trace!("find_tfa_token_by_secret");

        self.tokens_storage.find_by_secret(secret).cloned()
    }
}
