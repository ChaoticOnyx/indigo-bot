use app_macros::config;
use app_shared::serenity::model::id::{ChannelId, MessageId};

#[config]
#[derive(Debug)]
pub struct FeedbackConfig {
    pub template: String,
    pub channel_id: ChannelId,
    pub template_message_id: Option<MessageId>,
    pub vote_up_emoji: String,
    pub vote_down_emoji: String,
    pub min_feature_up_votes: u64,
}
