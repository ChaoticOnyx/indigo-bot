use std::str::FromStr;

use super::prelude::*;
use app_shared::{
    chrono::DateTime,
    models::{FeatureVote, FeatureVoteDescriptor},
    serenity::model::id::{ChannelId, MessageId},
};

pub struct FeatureMessageTable;

impl FeatureMessageTable {
    #[instrument]
    pub async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        trace!("create");

        sqlx::query(
            "
create table if not exists feature_message
(
    id            bigserial not null
        constraint feature_message_pk
            primary key,
    channel_id    bigint not null,
    message_id    bigint not null,
    user_id       bigint not null,
    is_vote_ended boolean not null,
    created_at    text    not null
);
",
        )
        .execute(pool)
        .await
    }

    #[instrument]
    pub async fn insert(
        pool: &Pool<Postgres>,
        feature: FeatureVote,
    ) -> Result<PgQueryResult, Error> {
        trace!("insert");

        let FeatureVote {
            descriptor,
            author_id,
            created_at,
            is_vote_ended,
        } = feature;
        let FeatureVoteDescriptor(message_id, channel_id) = descriptor;

        sqlx::query(
            "
INSERT INTO feature_message (id, channel_id, message_id, user_id, is_vote_ended, created_at)
VALUES (DEFAULT, $1, $2, $3, $4, $5);
        ",
        )
        .bind(channel_id.0 as i64)
        .bind(message_id.0 as i64)
        .bind(author_id.0 as i64)
        .bind(is_vote_ended)
        .bind(created_at.to_string())
        .execute(pool)
        .await
    }

    #[instrument]
    pub async fn find_by_descriptor(
        pool: &Pool<Postgres>,
        descriptor: FeatureVoteDescriptor,
    ) -> Result<Option<FeatureVote>, Error> {
        trace!("find_by_descriptor");

        let FeatureVoteDescriptor(message_id, channel_id) = descriptor;
        sqlx::query("SELECT id FROM feature_message WHERE channel_id = $1 AND message_id = $2")
            .bind(channel_id.0 as i64)
            .bind(message_id.0 as i64)
            .map(Self::map)
            .fetch_optional(pool)
            .await
    }

    #[instrument]
    pub async fn end_vote(
        pool: &Pool<Postgres>,
        descriptor: FeatureVoteDescriptor,
    ) -> Result<PgQueryResult, Error> {
        trace!("end_vote");

        let FeatureVoteDescriptor(message_id, channel_id) = descriptor;

        sqlx::query("UPDATE feature_message SET is_vote_ended = true WHERE channel_id = $1 AND message_id = $2")
            .bind(channel_id.0 as i64)
            .bind(message_id.0 as i64)
            .execute(pool)
            .await
    }

    #[instrument(skip(row))]
    fn map(row: PgRow) -> FeatureVote {
        FeatureVote {
            author_id: DiscordUserId(row.get::<i64, _>("user_id") as u64),
            created_at: DateTime::from_str(row.get("created_at")).unwrap(),
            descriptor: FeatureVoteDescriptor(
                MessageId(row.get::<i64, _>("message_id") as u64),
                ChannelId(row.get::<i64, _>("channel_id") as u64),
            ),
            is_vote_ended: row.get::<bool, _>("is_vote_ended"),
        }
    }
}
