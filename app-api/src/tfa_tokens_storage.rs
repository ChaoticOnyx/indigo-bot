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
    pub fn remove_expired_tokens(&mut self) {
        debug!("remove_expired_tokens");
        self.tokens.retain(|t| !t.is_expired());
    }

    #[instrument(skip(self))]
    pub fn new_token(&mut self, user: DiscordUser, duration: Duration) -> TFAToken {
        debug!("new_token");

        let mut secret;

        loop {
            secret = Secret::new_random_tfa_secret();

            if self.tokens.iter().any(|t| t.secret == secret) {
                continue;
            }

            break;
        }

        let token = TFAToken::new(secret, user, duration);
        self.tokens.push(token.clone());

        token
    }

    #[instrument(skip(self))]
    pub fn find_by_secret(&self, secret: Secret) -> Option<&TFAToken> {
        debug!("find_by_secret");

        self.tokens
            .iter()
            .find(|t| t.secret == secret && !t.is_expired())
    }

    #[instrument(skip(self))]
    pub fn find_by_user_id(&self, user_id: DiscordUserId) -> Option<&TFAToken> {
        debug!("find_by_user_id");

        self.tokens
            .iter()
            .find(|token| token.user.id == user_id && !token.is_expired())
    }
}
