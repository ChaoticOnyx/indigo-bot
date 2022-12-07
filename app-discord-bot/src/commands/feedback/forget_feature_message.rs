use app_api::Api;
use app_shared::{
    models::FeatureVoteDescriptor,
    prelude::*,
    serenity::{
        model::prelude::{ChannelId, MessageId},
        prelude::Context,
    },
};

#[instrument(skip(_ctx))]
pub async fn forget_feature_message(_ctx: &Context, channel_id: ChannelId, message_id: MessageId) {
    debug!("forget_message_delete");

    Api::lock(async_closure!(|api| {
        api.end_feature_vote(FeatureVoteDescriptor(message_id, channel_id))
            .await
    }))
    .await;
}
