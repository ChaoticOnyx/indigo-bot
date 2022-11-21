use std::{collections::HashMap, env};

use crate::database::Database;
use crate::prelude::*;
use handler::Handler;
use serenity::prelude::GatewayIntents;
use settings::Settings;
use tracing_loki::url::Url;
use tracing_subscriber::{prelude::*, Layer};

mod commands;
mod database;
mod github;
mod global_state;
mod handler;
mod prelude;
mod session;
mod settings;

async fn setup_logging() {
    use tracing_subscriber::filter::LevelFilter;

    let mut args = env::args();
    let mut level = LevelFilter::INFO;

    if args.any(|a| a == "--debug") {
        level = LevelFilter::DEBUG;
    }

    let settings = Settings::get_state().await;

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
            .with(layer)
            .with(tracing_subscriber::fmt::layer().pretty().with_filter(level))
            .init();

        tokio::spawn(task);
    } else {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().pretty().with_filter(level))
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
    Session::set_state(Session { user: None }).await;

    // Database
    let db = Database::connect(&settings.database.connect).await;
    db.migrate().await;
    Database::set_state(db).await;

    // GitHub
    let github = Github::new(settings.github.token);
    Github::set_state(github).await;

    // Discord
    let mut client = serenity::Client::builder(
        settings.discord.token,
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
