use crate::{
    endpoints, http_config::HttpConfig, manifest::Manifest, middleware::AuthRedirectorBuilder,
    templates::Templates,
};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::TrailingSlash, App, HttpServer};
use app_shared::{
    chrono::{Duration, Utc},
    prelude::*,
    tokio,
};
use notify::{RecursiveMode, Watcher};
use std::{path::Path, sync::mpsc};
use tera::Tera;

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
            Self::start_hot_reload().await;
        }

        let key = Key::from(config.cookies_key.as_bytes());

        HttpServer::new(move || {
            App::new()
                .wrap(
                    AuthRedirectorBuilder::default()
                        .redirect_to("/hub/auth")
                        .affected_paths(vec!["/hub/".to_string()])
                        .build()
                        .unwrap(),
                )
                .wrap(SessionMiddleware::new(
                    CookieSessionStore::default(),
                    key.clone(),
                ))
                .wrap(actix_web::middleware::NormalizePath::new(
                    TrailingSlash::Trim,
                ))
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

    async fn start_hot_reload() {
        tokio::runtime::Handle::current().spawn(async {
            let (tx, rx) = mpsc::channel();
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
