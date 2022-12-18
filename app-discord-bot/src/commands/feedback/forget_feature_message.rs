use app_api::Api;
use app_macros::tokio_blocking;
use app_shared::{
    models::FeatureVoteDescriptor,
    prelude::*,
    serenity::{
        model::prelude::{ChannelId, MessageId},
        prelude::Context,
    },
};

#[instrument(skip(_ctx))]
pub fn forget_feature_message(_ctx: &Context, channel_id: ChannelId, message_id: MessageId) {
    trace!("forget_message_delete");

    Api::lock(tokio_blocking!(|api| {
        api.private_api
            .end_feature_vote(FeatureVoteDescriptor(message_id, channel_id))
            .await
    }));
}
