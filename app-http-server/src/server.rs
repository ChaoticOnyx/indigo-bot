use crate::http_config::HttpConfig;
use crate::manifest::Manifest;
use actix_web::{App, HttpServer};
use app_shared::{
    chrono::{Duration, Utc},
    prelude::*,
    tokio,
};
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

        let config = HttpConfig::get().unwrap();

        let mut templates = Tera::new("templates/**/*.html").unwrap();
        templates.register_filter("asset_path", Manifest::asset_path);
        Templates::set_state(Templates(templates));

        let manifest = Manifest::new();
        Manifest::set_state(manifest);

        if config.hot_reload {
            Self::start_hot_relaod().await;
        }

        HttpServer::new(move || {
            App::new()
                .wrap(actix_web::middleware::NormalizePath::default())
                .service(actix_files::Files::new("/public", "./public"))
                .service(endpoints::api::scope())
                .service(endpoints::www::scope())
                .service(endpoints::www::hub::scope())
                .service(endpoints::www::not_found::endpoint)
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

            watcher
                .watch(Path::new("./public"), RecursiveMode::NonRecursive)
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
                        Templates::lock(|tera| {
                            if let Err(err) = tera.full_reload() {
                                error!("{err}");
                            }
                        });

                        debug!("reloading manifest");
                        Manifest::lock(|manifest| manifest.reload());
                    }
                }
            }
        });
    }
}
