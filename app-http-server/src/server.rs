use std::net::ToSocketAddrs;

use actix_web::{App, HttpServer};
use app_shared::prelude::*;
use tera::Tera;

use super::endpoints;
use crate::templates::Templates;

pub struct Server;

impl Server {
    #[instrument(skip(addrs))]
    pub async fn run(addrs: impl ToSocketAddrs) {
        info!("run");

        let templates = Tera::new("templates/**/*.html").unwrap();
        Templates::set_state(Templates(templates)).await;

        HttpServer::new(move || {
            App::new()
                .service(actix_files::Files::new("/static", "./static"))
                .service(endpoints::hub())
                .service(endpoints::api())
        })
        .bind(addrs)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}
