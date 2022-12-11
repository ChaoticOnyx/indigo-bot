use crate::commands::feedback::config::FeedbackConfig;
use app_shared::{
    prelude::*,
    serenity::{model::prelude::Ready, prelude::Context},
};

use crate::commands::feedback::helpers::create_and_pin_message;

#[instrument(skip(ctx))]
pub async fn ready(ctx: &Context, _ready: &Ready) {
    trace!("ready");

    let mut config = FeedbackConfig::get().await.unwrap();

    let channel = &config.channel_id;
    let template = config
        .template
        .replace(
            "{min_feature_up_votes}",
            &config.min_feature_up_votes.to_string(),
        )
        .replace("{vote_up_emoji}", &config.vote_up_emoji.to_string())
        .replace("{vote_down_emoji}", &config.vote_down_emoji.to_string());

    if let Some(message_id) = config.template_message_id {
        let message = config.channel_id.message(&ctx.http, message_id).await.ok();

        if let Some(mut message) = message {
            debug!("refreshing template message");

            message
                .edit(&ctx.http, |edit| edit.content(&template))
                .await
                .unwrap();
        } else {
            debug!("template message was deleted, creating a new one");

            let new_message = create_and_pin_message(ctx, channel, &template).await;
            config.template_message_id = Some(new_message.id);
            config.save().await;
        }
    } else {
        debug!("pinned message with report template not found, creating a new one");

        let new_message = create_and_pin_message(ctx, channel, &template).await;
        config.template_message_id = Some(new_message.id);
        config.save().await;
    }
}
