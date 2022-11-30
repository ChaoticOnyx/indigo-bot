use std::fmt::Display;

use chrono::{DateTime, Duration, Utc};
use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use serenity::model::user::User;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TokenSecret(pub String);

impl TokenSecret {
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();

        let secret: String = vec![
            random_digit(&mut rng),
            random_digit(&mut rng),
            random_digit(&mut rng),
            random_digit(&mut rng),
        ]
        .into_iter()
        .map(|n| n.to_string())
        .collect();

        Self(secret)
    }
}

impl From<String> for TokenSecret {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl Display for TokenSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TFAToken {
    pub user: User,
    pub secret: TokenSecret,
    pub expiration: DateTime<Utc>,
}

fn random_digit(rng: &mut ThreadRng) -> u8 {
    ((rng.gen::<u8>() / 10) % 10) as u8
}

impl TFAToken {
    pub fn new(secret: TokenSecret, user: User, duration: Duration) -> Self {
        Self {
            user,
            secret,
            expiration: Utc::now() + duration,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiration
    }
}
