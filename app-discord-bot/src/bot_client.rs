use crate::discord_config::DiscordConfig;
use app_shared::{
    prelude::*,
    serenity::{prelude::GatewayIntents, Client},
};

use super::handler::Handler;

pub struct BotClient;

impl BotClient {
    #[instrument]
    pub async fn run() {
        trace!("run");

        let config = DiscordConfig::get().await.unwrap();
        let mut client = Client::builder(&config.token, GatewayIntents::all())
            .event_handler(Handler)
            .await
            .unwrap();

        client.start().await.unwrap();
    }
}
