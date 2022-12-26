use app_shared::{
    chrono::Duration,
    models::{Secret, TFAToken},
    prelude::*,
};

#[derive(Debug, Clone, Default)]
pub struct TFATokensStorage {
    tokens: Vec<TFAToken>,
}

impl TFATokensStorage {
    #[instrument(skip(self))]
    fn remove_expired_tokens(&mut self) {
        trace!("remove_expired_tokens");
        self.tokens.retain(|t| !t.is_expired());
    }

    #[instrument(skip(self))]
    pub fn new_token(&mut self, discord_user_id: DiscordUserId, duration: Duration) -> TFAToken {
        trace!("new_token");

        self.remove_expired_tokens();

        let mut secret;

        loop {
            secret = Secret::new_random_tfa_secret();

            if self.tokens.iter().any(|t| t.secret == secret) {
                continue;
            }

            break;
        }

        let token = TFAToken::new(secret, discord_user_id, duration);
        self.tokens.push(token.clone());

        token
    }

    #[instrument(skip(self))]
    pub fn find_by_secret(&mut self, secret: Secret) -> Option<&TFAToken> {
        trace!("find_by_secret");

        self.remove_expired_tokens();

        self.tokens
            .iter()
            .find(|t| t.secret == secret && !t.is_expired())
    }

    #[instrument(skip(self))]
    pub fn find_by_discord_user_id(&mut self, discord_user_id: DiscordUserId) -> Option<&TFAToken> {
        trace!("find_by_user_id");

        self.remove_expired_tokens();

        self.tokens
            .iter()
            .find(|token| token.discord_user_id == discord_user_id && !token.is_expired())
    }
}
