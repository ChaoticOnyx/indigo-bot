use crate::templates::Templates;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, HttpResponseBuilder,
};
use app_shared::prelude::*;
use serde::Serialize;
use tera::Context;

pub struct HtmlResponse;

impl HtmlResponse {
    #[instrument(skip(context))]
    pub async fn from_template(
        template_name: &'static str,
        context: Option<impl Serialize>,
    ) -> HttpResponse {
        trace!("from_template");

        let context = match context {
            Some(context) => Context::from_serialize(context).unwrap(),
            None => Context::new(),
        };

        let html_result: Result<String, tera::Error> =
            Templates::lock_async(move |tera| tera.render(template_name, &context))
                .await
                .unwrap();

        match html_result {
            Ok(html) => HttpResponseBuilder::new(StatusCode::OK)
                .content_type(ContentType::html())
                .body(html),
            Err(err) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("{err:#?}")),
        }
    }
}
