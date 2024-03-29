use app_api::Api;
use app_shared::{
    models::ApiCaller,
    prelude::*,
    serenity::{
        model::{prelude::Message, Timestamp},
        prelude::Context,
    },
};

#[instrument(skip(ctx))]
pub async fn message(ctx: &Context, new_message: &Message) {
    if !new_message.is_private() {
        return;
    }

    if !new_message.content.starts_with("!2fa") {
        return;
    }

    let user = new_message.author.clone();
    let token =
        Api::lock_async(move |api| api.get_or_create_tfa_for_account(ApiCaller::System, user.id))
            .await
            .unwrap();

    let response_message = match token {
        Ok(token) => format!(
            "Ваш 2FA токен: `{}` истекающий <t:{}:R>",
            token.secret,
            Timestamp::from(token.expiration).unix_timestamp()
        ),
        Err(err) => format!("Произошла ошибка! {err}"),
    };

    new_message
        .reply_mention(&ctx.http, response_message)
        .await
        .unwrap();
}
