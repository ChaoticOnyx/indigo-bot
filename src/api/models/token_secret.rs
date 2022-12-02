use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

const CHARSET: &[u8] = b"ABCDEFGHKLMNOPQRSTUVWXYZ0123456789";

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TokenSecret(pub String);

impl TokenSecret {
    pub fn new_random_api_secret() -> Self {
        let secret: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(60)
            .map(char::from)
            .collect();

        Self(secret)
    }

    pub fn new_random_tfa_secret() -> Self {
        let mut rng = rand::thread_rng();

        let secret: String = (0..=6)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
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
