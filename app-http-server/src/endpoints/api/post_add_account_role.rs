use actix_web::{post, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use app_api::Api;
use serde::{Deserialize, Serialize};

use crate::ResponseHelpers;
use app_shared::{
    models::{AccountId, ApiCaller, RoleId, Secret},
    prelude::*,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    pub role_id: RoleId,
}

#[instrument]
#[post("/account/{account_id}/roles")]
pub async fn endpoint(
    account_id: web::Path<i64>,
    body: web::Json<Body>,
    secret: BearerAuth,
) -> impl Responder {
    trace!("endpoint");

    let account_id = AccountId(account_id.into_inner());
    let secret = Secret(secret.token().to_string());
    let Body { role_id } = body.0;

    let response = Api::lock_async(move |api| {
        api.add_role_to_account(ApiCaller::Token(secret), account_id, role_id)
    })
    .await
    .unwrap();

    ResponseHelpers::from_api_result(response)
}
