use crate::api::private::PrivateApi;
use app_shared::models::Secret;
use app_shared::prelude::*;

impl PrivateApi {
    /// Создаёт уникальный секрет для webhook.
    #[instrument]
    pub fn create_unique_webhook_secret(&self) -> Secret {
        trace!("create_unique_webhook_secret");

        loop {
            let secret = Secret::new_random_webhook_secret();

            if self
                .database
                .find_webhook_by_secret(secret.clone())
                .is_none()
            {
                break secret;
            }
        }
    }
}
