use crate::prelude::*;
use chrono::{DateTime, Utc};
use serenity::model::prelude::{ChannelId, Message, MessageId};

#[derive(Debug, Clone, Copy)]
pub struct FeatureVoteDescriptor(pub MessageId, pub ChannelId);

impl From<Message> for FeatureVoteDescriptor {
    fn from(message: Message) -> Self {
        Self(message.id, message.channel_id)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FeatureVote {
    pub descriptor: FeatureVoteDescriptor,
    pub author_id: DiscordUserId,
    pub created_at: DateTime<Utc>,
}

impl From<Message> for FeatureVote {
    fn from(message: Message) -> Self {
        Self {
            author_id: message.author.id,
            descriptor: message.clone().into(),
            created_at: *message.timestamp,
        }
    }
}
