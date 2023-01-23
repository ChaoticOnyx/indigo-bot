use actix_http::{header, Method, StatusCode};
use actix_web::{routes, web::Form, HttpRequest, HttpResponse, HttpResponseBuilder};
use serde::{Deserialize, Serialize};

use app_api::Api;
use app_shared::{
    chrono::{DateTime, Utc},
    models::{Secret, Session},
    prelude::*,
    UserAgentParser,
};

use crate::response::ResponseHelpers;
use crate::{extractors::AuthenticatedUser, html_response::HtmlResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionInfo {
    pub os: String,
    pub browser: String,
    pub ip: String,
    pub created_at: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum SessionsAction {
    #[serde(rename = "all")]
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionsForm {
    pub csrf_token: Secret,
    #[serde(flatten)]
    pub action: SessionsAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RenderContext {
    pub user: AuthenticatedUser,
    pub sessions: Vec<SessionInfo>,
    pub form: SessionsForm,
    pub errors: Vec<String>,
}

#[instrument]
async fn handle(context: &mut RenderContext) -> Option<HttpResponse> {
    trace!("session_menu");

    let RenderContext { user, .. } = context.clone();
    let mut sessions: Vec<Session> =
        Api::lock_async(move |api| api.get_account_sessions(user.account.id))
            .await
            .unwrap();

    return match context.form.action.clone() {
        SessionsAction::All => {
            {
                let target_sessions = sessions.clone();
                Api::lock_async(move |api| {
                    for session in target_sessions {
                        api.delete_session(session.secret.clone()).ok();
                    }
                })
                .await
                .unwrap();
            }

            sessions.clear();

            Some(
                HttpResponseBuilder::new(StatusCode::SEE_OTHER)
                    .insert_header((header::LOCATION, "/auth"))
                    .finish(),
            )
        }
    };
}

#[instrument]
async fn context(user: &AuthenticatedUser, form: Option<&SessionsForm>) -> RenderContext {
    trace!("context");

    let mut sessions_info = Vec::new();
    let user = user.clone();
    let sessions: Vec<Session> =
        Api::lock_async(move |api| api.get_account_sessions(user.account.id))
            .await
            .unwrap();

    let parser: UserAgentParser = UserAgentParser::clone_state();

    for session in sessions {
        let session_info = parser.parse(&session.user_agent);

        sessions_info.push(SessionInfo {
            os: session_info
                .os
                .unwrap_or_else(|| String::from("Неизвестная ОС")),
            browser: session_info
                .browser
                .unwrap_or_else(|| String::from("Неизвестный браузер")),
            ip: session.ip,
            created_at: session.created_at,
            expiration: session.expiration,
        })
    }

    RenderContext {
        form: form.cloned().unwrap_or_else(|| SessionsForm {
            action: SessionsAction::All,
            csrf_token: user.session.csrf_token.clone(),
        }),
        user: user.clone(),
        sessions: sessions_info,
        errors: Vec::new(),
    }
}

#[instrument]
async fn render(context: RenderContext) -> HttpResponse {
    trace!("render");

    HtmlResponse::from_template("account/sessions.html", Some(context)).await
}

#[instrument]
#[routes]
#[get("/sessions")]
#[post("/sessions")]
pub async fn endpoint(
    request: HttpRequest,
    user: AuthenticatedUser,
    form: Option<Form<SessionsForm>>,
) -> HttpResponse {
    trace!("endpoint");

    let form = form.map(|map| map.0);
    let mut ctx = context(&user, form.as_ref()).await;

    if form.is_some() && request.method() == Method::POST {
        let csrf_token = form.unwrap().csrf_token;

        if !Api::lock_async(move |api| api.is_csrf_secret_valid(csrf_token))
            .await
            .unwrap()
        {
            return ResponseHelpers::new(StatusCode::BAD_REQUEST, "Некорректный CSRF токен.");
        }

        if let Some(response) = handle(&mut ctx).await {
            return response;
        }
    }

    render(ctx).await
}
