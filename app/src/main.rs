use std::collections::HashMap;
use std::str::FromStr;

use app_api::Api;
use app_discord_bot::BotClient;
use app_http_server::Server;
use tracing_loki::url::Url;
use tracing_subscriber::{prelude::*, Layer};

use app_shared::{
    prelude::*, tokio, ConfigLoader, DiscordSession, PersistentStorage, Settings, UserAgentParser,
};

fn setup_logging() {
    use tracing_subscriber::filter::LevelFilter;

    let settings = Settings::clone_state();
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

        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async {
                tokio::spawn(task).await.unwrap();
            })
        });
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
fn main() {
    // Settings
    let settings = Settings::load();
    Settings::set_state(settings);

    // Persistent storage
    PersistentStorage::set_state(PersistentStorage::from_file().unwrap_or_default());

    setup_logging();

    // User agent parser
    UserAgentParser::set_state(UserAgentParser::default());

    // Config Loader
    ConfigLoader::set_state(ConfigLoader::new("./configs"));

    // Session
    DiscordSession::set_state(DiscordSession { user: None });

    // Api
    let api = Api::default();
    Api::set_state(api);

    // Discord
    let discord_thread = std::thread::spawn(|| {
        let client = BotClient::default();
        client.run();
    });

    let server_thread = std::thread::spawn(|| {
        let server = Server::default();
        server.run();
    });

    discord_thread.join().unwrap();
    server_thread.join().unwrap();
}
