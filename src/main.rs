use std::{collections::HashMap, env};

use crate::prelude::*;
use bot::BotClient;
use server::Server;
use tracing_loki::url::Url;
use tracing_subscriber::{prelude::*, Layer};

mod api;
mod bot;
mod macros;
mod prelude;
mod server;
mod state;

async fn setup_logging() {
    use tracing_subscriber::filter::LevelFilter;

    let mut args = env::args();
    let mut filter = LevelFilter::INFO;

    if args.any(|a| a == "--debug") {
        filter = LevelFilter::DEBUG;
    }

    let settings = Settings::clone_state().await;

    if settings.loki.enabled {
        let mut labels = HashMap::new();

        labels.insert("App".to_string(), "IndigoBot".to_string());

        let (layer, task) = tracing_loki::layer(
            Url::parse(&settings.loki.url.unwrap()).unwrap(),
            labels,
            HashMap::new(),
        )
        .unwrap();

        tracing_subscriber::registry()
            .with(layer.with_filter(filter))
            .with(
                tracing_subscriber::fmt::layer()
                    .pretty()
                    .with_filter(filter),
            )
            .init();

        tokio::spawn(task);
    } else {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .pretty()
                    .with_filter(filter),
            )
            .init();
    }
}

#[instrument]
#[tokio::main]
async fn main() {
    // Settings
    let settings = Settings::load();
    Settings::set_state(settings.clone()).await;

    setup_logging().await;

    // Session
    DiscordSession::set_state(DiscordSession { user: None }).await;

    // Api
    let api = Api::new(&settings).await;
    Api::set_state(api).await;

    // Discord
    let discord_handle = tokio::spawn(async {
        BotClient::run(settings.discord.token).await;
    });

    Server::run(settings.server.address).await;
    discord_handle.await.unwrap();
}
