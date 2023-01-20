use actix_web::{get, web::Query, Responder};

use app_api::Api;
use app_shared::{models::AnyUserId, prelude::*};
use serde::{Deserialize, Serialize};

use crate::ResponseHelpers;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    pub user_id: SS14UserId,
}

#[instrument]
#[get("/ss14")]
pub async fn endpoint(payload: Query<Payload>) -> impl Responder {
    trace!("endpoint");

    let user_id = payload.0.user_id;
    let account = Api::lock_async(move |api| {
        api.private_api
            .find_account_by_id(AnyUserId::SS14Guid(user_id))
    })
    .await
    .unwrap();

    ResponseHelpers::from_api_result(account)
}
