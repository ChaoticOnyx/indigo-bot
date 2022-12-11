use crate::http_config::HttpConfig;
use actix_web::{App, HttpServer};
use app_shared::prelude::*;
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
}
