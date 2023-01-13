use crate::api::private::PrivateApi;
use app_shared::{models::Secret, prelude::*};

impl PrivateApi {
    /// Создаёт уникальный секрет для Api токена.
    #[instrument]
    pub fn create_unique_api_secret(&self) -> Secret {
        trace!("create_unique_api_secret");

        loop {
            let secret = Secret::new_random_api_secret();

            if self
                .database
                .find_api_token_by_secret(secret.clone())
                .is_none()
            {
                break secret;
            }
        }
    }
}
