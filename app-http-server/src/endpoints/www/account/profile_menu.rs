use actix_http::{Method, StatusCode};

use actix_web::{routes, web::Form, HttpRequest, HttpResponse};
use app_shared::{
    models::{AnyUserId, Secret},
    prelude::*,
};

use crate::{
    extractors::AuthenticatedUser, html_response::HtmlResponse, response::ResponseHelpers,
    FormErrors,
};
use app_api::Api;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileForm {
    pub username: String,
    pub avatar_url: String,
    pub csrf_token: Secret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RenderContext {
    pub user: AuthenticatedUser,
    pub form: ProfileForm,
    pub errors: FormErrors,
}

#[instrument]
async fn handle(context: &mut RenderContext) {
    trace!("handle");

    let RenderContext { user, form, .. } = context.clone();
    let errors = &mut context.errors;

    let csrf_secret = form.csrf_token.clone();
    if !Api::lock_async(move |api| api.private_api.is_csrf_secret_valid(csrf_secret))
        .await
        .unwrap()
    {
        errors
            .entry("csrf_token".to_string())
            .or_default()
            .push("Некорректный CSRF токен".to_string())
    }

    let new_username = form.username.clone();
    if user.account.username != new_username {
        if let Err(err) = Api::lock_async(move |api| {
            api.private_api
                .change_username(AnyUserId::AccountId(user.account.id), new_username)
        })
        .await
        .unwrap()
        {
            errors
                .entry("username".to_string())
                .or_default()
                .push(err.to_string());
        }
    }

    let new_avatar_url = form.avatar_url.clone();
    if user.account.avatar_url != form.avatar_url {
        if let Err(err) = Api::lock_async(move |api| {
            api.private_api
                .change_avatar_url(AnyUserId::AccountId(user.account.id), new_avatar_url)
        })
        .await
        .unwrap()
        {
            errors
                .entry("avatar_url".to_string())
                .or_default()
                .push(err.to_string());
        }
    }
}

#[instrument]
fn context(user: &AuthenticatedUser, form: Option<&ProfileForm>) -> RenderContext {
    trace!("context");

    let form = form.cloned().unwrap_or_else(|| ProfileForm {
        username: user.account.username.clone(),
        avatar_url: user.account.avatar_url.clone(),
        csrf_token: user.session.csrf_token.clone(),
    });

    RenderContext {
        form,
        user: user.clone(),
        errors: FormErrors::default(),
    }
}

#[instrument]
async fn render(context: RenderContext) -> HttpResponse {
    trace!("render");

    return HtmlResponse::from_template("account/profile.html", Some(context)).await;
}

#[instrument]
#[routes]
#[get("/profile")]
#[post("/profile")]
pub async fn endpoint(
    request: HttpRequest,
    user: AuthenticatedUser,
    form: Option<Form<ProfileForm>>,
) -> HttpResponse {
    trace!("endpoint");

    let form = form.map(|map| map.0);
    let mut ctx = context(&user, form.as_ref());

    if form.is_some() && request.method() == Method::POST {
        let csrf_token = form.unwrap().csrf_token.clone();

        if !Api::lock_async(move |api| api.private_api.is_csrf_secret_valid(csrf_token))
            .await
            .unwrap()
        {
            return ResponseHelpers::new(StatusCode::BAD_REQUEST, "Некорректный CSRF токен.");
        }

        handle(&mut ctx).await;
    }

    render(ctx).await
}
