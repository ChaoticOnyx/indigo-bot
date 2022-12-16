use app_api::Api;
use app_macros::tokio_blocking;
use app_shared::{
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
    let token = Api::lock(tokio_blocking!(|api| {
        api.get_or_create_tfa_token(user).await
    }));

    new_message
        .reply_mention(
            &ctx.http,
            format!(
                "Ваш 2FA токен: `{}` истекающий <t:{}:R>",
                token.secret,
                Timestamp::from(token.expiration).unix_timestamp()
            ),
        )
        .await
        .unwrap();
}
