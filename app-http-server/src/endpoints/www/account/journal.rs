use std::collections::HashMap;

use crate::{extractors::AuthenticatedUser, html_response::HtmlResponse};
use actix_web::{get, web::Query, HttpRequest, Responder};
use app_api::Api;
use app_shared::{
    models::{Account, AccountId, Actor, JournalEntryCursor, Role, RoleId},
    prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RenderContext {
    pub user: AuthenticatedUser,
    pub cursor: JournalEntryCursor,
    pub roles: HashMap<RoleId, Role>,
    pub accounts: HashMap<AccountId, Account>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationQuery {
    pub offset: Option<usize>,
}

async fn context(user: &AuthenticatedUser, pagination: PaginationQuery) -> RenderContext {
    let empty_cursor = JournalEntryCursor::new(
        pagination.offset.unwrap_or(0),
        Some(Actor::User(user.account.id)),
        10,
    );

    let cursor = empty_cursor.clone();
    let (cursor, roles, accounts) = Api::lock_async(move |api| {
        let cursor = api.get_journal_entries(cursor);

        let roles = api
            .get_roles()
            .into_iter()
            .map(|role| (role.id, role))
            .collect();

        let accounts = api
            .get_accounts()
            .into_iter()
            .map(|account| (account.id, account))
            .collect();

        (cursor, roles, accounts)
    })
    .await
    .unwrap();

    RenderContext {
        user: user.clone(),
        cursor: cursor.unwrap_or(empty_cursor),
        roles,
        accounts,
    }
}

#[instrument]
async fn render(context: RenderContext) -> impl Responder {
    trace!("render");

    return HtmlResponse::from_template("account/journal.html", Some(context)).await;
}

#[instrument]
#[get("/journal")]
pub async fn endpoint(
    request: HttpRequest,
    user: AuthenticatedUser,
    pagination: Query<PaginationQuery>,
) -> impl Responder {
    trace!("endpoint");

    let ctx = context(&user, pagination.0).await;

    render(ctx).await
}
