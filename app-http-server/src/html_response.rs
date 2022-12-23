use crate::templates::Templates;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use app_shared::{prelude::*, serde_json};
use tera::Context;

pub struct HtmlResponse;

impl HtmlResponse {
    #[instrument]
    pub async fn from_template(
        template_name: &'static str,
        context: Option<serde_json::Value>,
    ) -> HttpResponse {
        trace!("from_template");

        let context = match context {
            Some(context) => Context::from_serialize(context).unwrap(),
            None => Context::new(),
        };

        let html = Templates::lock_async(move |tera| tera.render(template_name, &context).unwrap())
            .await
            .unwrap();

        HttpResponseBuilder::new(StatusCode::OK)
            .content_type(ContentType::html())
            .body(html)
    }
}
