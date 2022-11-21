use serenity::{
    model::prelude::{ChannelId, GuildId, MessageId},
    prelude::Context,
};

use crate::{commands::feedback::forget_feature_message, prelude::*};

#[instrument(skip(ctx))]
pub async fn message_delete(
    ctx: &Context,
    channel_id: ChannelId,
    deleted_message_id: MessageId,
    _guild_id: Option<GuildId>,
) {
    debug!("message_delete");

    forget_feature_message(ctx, channel_id, deleted_message_id).await;
}
