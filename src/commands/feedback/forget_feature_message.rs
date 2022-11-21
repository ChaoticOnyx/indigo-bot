use serenity::{
    model::prelude::{ChannelId, MessageId},
    prelude::Context,
};

use crate::prelude::*;

#[instrument(skip(_ctx))]
pub async fn forget_feature_message(_ctx: &Context, channel_id: ChannelId, message_id: MessageId) {
    debug!("forget_message_delete");

    let db = Database::get_state().await;

    db.end_vote_feature_message(channel_id, message_id).await;
}
