use crate::discord_config::DiscordConfig;
use app_shared::{
    prelude::*,
    serenity::{prelude::GatewayIntents, Client},
    tokio::runtime::Runtime,
};

use super::handler::Handler;

#[derive(Debug)]
pub struct BotClient {
    rt: Runtime,
}

impl BotClient {
    #[instrument]
    pub fn new() -> Self {
        trace!("new");

        Self {
            rt: app_shared::tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap(),
        }
    }

    #[instrument]
    pub fn run(&self) {
        trace!("run");

        let config = DiscordConfig::get().unwrap();

        self.rt.block_on(async {
            let mut client = Client::builder(&config.token, GatewayIntents::all())
                .event_handler(Handler)
                .await
                .unwrap();

            client.start().await.unwrap();
        });
    }
}
