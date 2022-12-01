use crate::prelude::*;

#[instrument]
pub async fn is_token_valid(token: &str) -> bool {
    Settings::clone_state()
        .await
        .server
        .tokens
        .iter()
        .any(|t| t == token)
}
