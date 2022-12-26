use crate::{extractors::AuthenticatedUser, html_response::HtmlResponse};
use actix_web::{get, HttpRequest, Responder};
use app_shared::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RenderContext {
    pub user: AuthenticatedUser,
}

fn context(user: &AuthenticatedUser) -> RenderContext {
    RenderContext { user: user.clone() }
}

#[instrument]
async fn render(context: RenderContext) -> impl Responder {
    trace!("render");

    return HtmlResponse::from_template("account/integrations.html", Some(context)).await;
}

#[instrument]
#[get("/integrations")]
pub async fn endpoint(request: HttpRequest, user: AuthenticatedUser) -> impl Responder {
    trace!("endpoint");

    let ctx = context(&user);

    render(ctx).await
}
