use chrono::Duration;
use serenity::model::{prelude::UserId, user::User};

use super::models::{TFAToken, TokenSecret};

#[derive(Debug, Clone, Default)]
pub struct TFATokensStorage {
    tokens: Vec<TFAToken>,
}

impl TFATokensStorage {
    pub fn remove_expired_tokens(&mut self) {
        self.tokens.retain(|t| !t.is_expired());
    }

    pub fn new_token(&mut self, user: User, duration: Duration) -> TFAToken {
        let mut secret;

        loop {
            secret = TokenSecret::new_random();

            if self.tokens.iter().any(|t| t.secret == secret) {
                continue;
            }

            break;
        }

        let token = TFAToken::new(secret, user, duration);
        self.tokens.push(token.clone());

        token
    }

    pub fn find_by_secret(&self, secret: TokenSecret) -> Option<&TFAToken> {
        self.tokens
            .iter()
            .find(|t| t.secret == secret && !t.is_expired())
    }

    pub fn find_by_user_id(&self, user_id: UserId) -> Option<&TFAToken> {
        self.tokens
            .iter()
            .find(|token| token.user.id == user_id && !token.is_expired())
    }
}
