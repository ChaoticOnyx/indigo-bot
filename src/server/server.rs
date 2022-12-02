use std::net::ToSocketAddrs;

use actix_web::{App, HttpServer};

use super::endpoints;

pub struct Server;

impl Server {
    pub async fn run(addrs: impl ToSocketAddrs) {
        HttpServer::new(move || {
            App::new()
                .service(endpoints::get::identity)
                .service(endpoints::post::connect_byond)
                .service(endpoints::post::create_api_token)
                .service(endpoints::delete::delete_api_token)
                // BYOND-friendly API
                .service(endpoints::byond::get::connect_byond)
        })
        .bind(addrs)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}
