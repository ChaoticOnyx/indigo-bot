use app_shared::serenity::{prelude::GatewayIntents, Client};

use super::handler::Handler;

pub struct BotClient;

impl BotClient {
    pub async fn run(token: impl AsRef<str>) {
        let mut client = Client::builder(
            token,
            GatewayIntents::non_privileged()
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::GUILD_MESSAGE_REACTIONS,
        )
        .event_handler(Handler)
        .await
        .unwrap();

        client.start().await.unwrap();
    }
}
