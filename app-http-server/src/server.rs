use crate::{
    endpoints, filters,
    http_config::HttpConfig,
    manifest::Manifest,
    middleware::{AuthRedirectorOptionsBuilder, SessionExtenderOptionsBuilder},
    templates::Templates,
};
use actix_web::{middleware::TrailingSlash, web, App, HttpServer};
use app_shared::{
    chrono::{Duration, Utc},
    prelude::*,
    tokio,
    tokio::runtime::Runtime,
};
use notify::{RecursiveMode, Watcher};
use std::{path::Path, sync::mpsc};
use tera::Tera;

#[derive(Debug)]
pub struct Server {
    rt: Runtime,
}

impl Server {
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
        info!("run");

        let config = HttpConfig::get().unwrap();

        self.rt.block_on(async {
            let mut templates = Tera::new("templates/**/*.html").unwrap();
            templates.register_filter("asset_path", filters::asset_path_filter);
            templates.register_filter("role_bits", filters::rights_to_bits_filter);
            templates.register_filter("main_role", filters::main_role_filter);
            Templates::set_state(Templates(templates));

            let manifest = Manifest::new();
            Manifest::set_state(manifest);

            if config.hot_reload {
                Self::start_hot_reload().await;
            }

            HttpServer::new(move || {
                App::new()
                    .wrap(
                        AuthRedirectorOptionsBuilder::default()
                            .redirect_to("/auth")
                            .affected_paths(vec![String::from("/account")])
                            .build()
                            .unwrap(),
                    )
                    .wrap(
                        SessionExtenderOptionsBuilder::default()
                            .extend_before(Duration::days(1))
                            .build()
                            .unwrap(),
                    )
                    .wrap(actix_web::middleware::NormalizePath::new(
                        TrailingSlash::Trim,
                    ))
                    .service(actix_files::Files::new("/public", "./public"))
                    .service(endpoints::api::scope())
                    .service(endpoints::www::scope())
                    .default_service(web::route().to(endpoints::www::not_found::endpoint))
            })
            .bind(config.address)
            .unwrap()
            .run()
            .await
            .unwrap();
        });
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
                        Templates::lock_async(|tera| {
                            if let Err(err) = tera.full_reload() {
                                error!("{err}");
                            }
                        })
                        .await
                        .unwrap();

                        debug!("reloading manifest");
                        Manifest::lock(|manifest| manifest.reload());
                    }
                }
            }
        });
    }
}
