use app_shared::{
    prelude::*,
    serenity::{
        model::prelude::{ChannelId, GuildId, MessageId},
        prelude::Context,
    },
    state::Settings,
};

use crate::commands::feedback::forget_feature_message;

#[instrument(skip(ctx))]
pub async fn message_delete(
    ctx: &Context,
    channel_id: ChannelId,
    deleted_message_id: MessageId,
    _guild_id: Option<GuildId>,
) {
    debug!("message_delete");

    if channel_id != Settings::clone_state().await.commands.feedback.channel_id {
        return;
    }

    forget_feature_message(ctx, channel_id, deleted_message_id).await;
}
