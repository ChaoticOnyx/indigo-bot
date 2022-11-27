use std::net::ToSocketAddrs;

use actix_web::{App, HttpServer};

use super::endpoints;

pub struct Server;

impl Server {
    pub async fn run(addrs: impl ToSocketAddrs) {
        HttpServer::new(move || {
            App::new()
                .service(endpoints::get::auth)
                .service(endpoints::get::identity)
        })
        .bind(addrs)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}
