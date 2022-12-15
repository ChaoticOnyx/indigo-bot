use crate::http_config::HttpConfig;
use actix_web::{App, HttpServer};
use app_shared::chrono::Duration;
use app_shared::{chrono::Utc, prelude::*, tokio};
use notify::{RecursiveMode, Watcher};
use std::path::Path;
use std::sync;
use tera::Tera;

use super::endpoints;
use crate::templates::Templates;

pub struct Server;

impl Server {
    #[instrument]
    pub async fn run() {
        info!("run");

        let config = HttpConfig::get().await.unwrap();

        let templates = Tera::new("templates/**/*.html").unwrap();
        Templates::set_state(Templates(templates)).await;

        if config.hot_reload {
            Self::start_hot_relaod().await;
        }

        HttpServer::new(move || {
            App::new()
                .service(actix_files::Files::new("/static", "./static"))
                .service(endpoints::hub())
                .service(endpoints::api())
        })
        .bind(config.address)
        .unwrap()
        .run()
        .await
        .unwrap();
    }

    async fn start_hot_relaod() {
        tokio::runtime::Handle::current().spawn(async {
            let (tx, rx) = sync::mpsc::channel();
            let mut watcher = notify::recommended_watcher(tx).unwrap();
            let cooldown = Duration::seconds(1);
            let mut last_event = Utc::now();

            watcher
                .watch(Path::new("./templates"), RecursiveMode::Recursive)
                .unwrap();

            for ev in rx {
                match ev {
                    Err(err) => error!("{err}"),
                    Ok(_) => {
                        if Utc::now() - last_event > cooldown {
                            last_event = Utc::now();
                        } else {
                            continue;
                        }

                        debug!("reloading templates");
                        Templates::lock(async_closure!(|tera| {
                            tera.full_reload().unwrap();
                        }))
                        .await;
                    }
                }
            }
        });
    }
}
