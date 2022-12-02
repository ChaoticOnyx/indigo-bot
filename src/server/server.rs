use std::net::ToSocketAddrs;

use actix_web::{App, HttpServer};

use super::endpoints;

pub struct Server;

impl Server {
    pub async fn run(addrs: impl ToSocketAddrs) {
        HttpServer::new(move || {
            App::new()
                .service(endpoints::get::identity)
                .service(endpoints::get::connect_byond)
                .service(endpoints::post::create_api_token)
        })
        .bind(addrs)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}
