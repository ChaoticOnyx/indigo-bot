use std::collections::HashMap;
use std::str::FromStr;

use app_api::Api;
use app_discord_bot::BotClient;
use app_http_server::Server;
use tracing_loki::url::Url;
use tracing_subscriber::{prelude::*, Layer};

use app_shared::{prelude::*, tokio, ConfigLoader, DiscordSession, Settings};

async fn setup_logging() {
    use tracing_subscriber::filter::LevelFilter;

    let settings = Settings::clone_state().await;
    let filter = LevelFilter::from_str(&settings.logging.log_level).unwrap();

    if settings.logging.loki.enabled {
        let mut labels = HashMap::new();

        labels.insert("App".to_string(), "IndigoBot".to_string());

        let (layer, task) = tracing_loki::layer(
            Url::parse(&settings.logging.loki.url.unwrap()).unwrap(),
            labels,
            HashMap::new(),
        )
        .unwrap();

        tracing_subscriber::registry()
            .with(
                layer.with_filter(LevelFilter::from_str(&settings.logging.loki.log_level).unwrap()),
            )
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

    // Config Loader
    ConfigLoader::set_state(ConfigLoader::new("./configs").await).await;

    // Session
    DiscordSession::set_state(DiscordSession { user: None }).await;

    // Api
    let api = Api::new().await;
    Api::set_state(api).await;

    // Discord
    let discord_handle = tokio::spawn(async {
        BotClient::run().await;
    });

    Server::run().await;
    discord_handle.await.unwrap();
}
