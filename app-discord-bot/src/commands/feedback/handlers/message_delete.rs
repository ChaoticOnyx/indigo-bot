use crate::commands::feedback::config::FeedbackConfig;
use app_shared::{
    prelude::*,
    serenity::{
        model::prelude::{ChannelId, GuildId, MessageId},
        prelude::Context,
    },
};

use crate::commands::feedback::forget_feature_message;

#[instrument(skip(ctx))]
pub async fn message_delete(
    ctx: &Context,
    channel_id: ChannelId,
    deleted_message_id: MessageId,
    _guild_id: Option<GuildId>,
) {
    trace!("message_delete");

    let config = FeedbackConfig::get().await.unwrap();

    if channel_id != config.channel_id {
        return;
    }

    forget_feature_message(ctx, channel_id, deleted_message_id).await;
}
