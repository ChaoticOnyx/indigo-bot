use app_shared::{
    chrono::{Duration, Utc},
    models::{AnyUserId, Secret, TFAToken},
    prelude::*,
};

use crate::Api;

impl Api {
    #[instrument]
    pub async fn get_or_create_tfa_token(&mut self, user: DiscordUser) -> TFAToken {
        trace!("get_or_create_tfa_token");

        if self
            .database
            .find_account(AnyUserId::DiscordId(user.id))
            .await
            .is_none()
        {
            self.database.add_account(user.id, Utc::now(), &[]).await;
        }

        self.tokens_storage.remove_expired_tokens();

        match self.tokens_storage.find_by_user_id(user.id) {
            None => {
                debug!("existing token not found");

                self.tokens_storage.new_token(user, Duration::seconds(60))
            }
            Some(token) => {
                debug!("existing token found");
                token.clone()
            }
        }
    }

    #[instrument]
    pub async fn find_tfa_token_by_secret(&self, secret: Secret) -> Option<TFAToken> {
        trace!("find_tfa_token_by_secret");

        self.tokens_storage.find_by_secret(secret).cloned()
    }
}
