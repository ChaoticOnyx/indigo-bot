use app_shared::{
    prelude::*,
    serenity::{prelude::GatewayIntents, Client},
    tokio::runtime::Runtime,
    DiscordConfig,
};

use super::handler::Handler;

#[derive(Debug)]
pub struct BotClient {
    rt: Runtime,
}

impl BotClient {
    pub fn new() -> Self {
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
