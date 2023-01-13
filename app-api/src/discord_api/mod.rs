use app_shared::serenity::model::prelude::User;
use app_shared::{prelude::*, serenity::http::Http, tokio::runtime::Runtime, DiscordConfig};

#[derive(Debug)]
pub struct DiscordApi {
    http: Http,
    rt: Runtime,
}

impl DiscordApi {
    pub fn get_discord_user(&self, user_id: DiscordUserId) -> Option<User> {
        self.rt
            .block_on(async move { self.http.get_user(user_id.0).await.ok() })
    }
}

impl Default for DiscordApi {
    fn default() -> Self {
        let discord_config: DiscordConfig = DiscordConfig::get().unwrap();

        let http = app_shared::serenity::http::HttpBuilder::new(discord_config.token).build();
        let rt = app_shared::tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        Self { http, rt }
    }
}
