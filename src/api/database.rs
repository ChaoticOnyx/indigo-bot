use std::str::FromStr;

use chrono::DateTime;
use serenity::model::prelude::{ChannelId, MessageId, UserId};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Row, Sqlite};

use crate::{
    api::models::{BugReport, BugReportDescriptor, FeatureVote, FeatureVoteDescriptor},
    prelude::*,
};

#[derive(Debug)]
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    #[instrument]
    pub async fn connect(conn: &str) -> Self {
        info!("setting up database");

        let pool = SqlitePoolOptions::new().connect(conn).await.unwrap();

        Self { pool }
    }

    #[instrument(skip(self))]
    pub async fn migrate(&self) {
        info!("migrating");

        // Add migrations here!
        Self::migration_init(&self.pool).await;

        info!("migration done");
    }

    #[instrument(skip(pool))]
    async fn migration_init(pool: &Pool<Sqlite>) {
        info!("migration: migration_init");

        // TABLE feature_message
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS feature_message
            (
                id INTEGER NOT null
                   CONSTRAINT id
                   PRIMARY KEY AUTOINCREMENT,
                channel_id INTEGER NOT null,
                message_id INTEGER NOT null,
                user_id INTEGER NOT null,
				is_vote_ended INTEGER NOT null,
				created_at TEXT NOT null
            );
        ",
        )
        .execute(pool)
        .await
        .unwrap();

        // TABLE bug_message
        sqlx::query(
            "
			CREATE TABLE IF NOT EXISTS bug_message
			(
				id INTEGER NOT null
					CONSTRAINT id
					PRIMARY KEY AUTOINCREMENT,
				user_id INTEGER NOT null,
				issue_number INTEGER NOT null
			);
		",
        )
        .execute(pool)
        .await
        .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn add_bug_report(&self, bug_report: BugReport) {
        debug!("add_bug_report");

        let BugReport {
            descriptor,
            author_id,
        } = bug_report;
        let BugReportDescriptor(issue_id) = descriptor;

        sqlx::query(
            "
			INSERT INTO bug_message (user_id, issue_number)
			VALUES (?, ?);
			",
        )
        .bind(author_id.0 as i64)
        .bind(issue_id.0 as i64)
        .execute(&self.pool)
        .await
        .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn add_feature_vote(&self, feature: FeatureVote) {
        debug!("add_feature_vote");

        let FeatureVote {
            descriptor,
            author_id,
            created_at,
        } = feature;
        let FeatureVoteDescriptor(message_id, channel_id) = descriptor;

        sqlx::query(
            "
            INSERT INTO feature_message (channel_id, message_id, user_id, is_vote_ended, created_at)
            VALUES (?, ?, ?, 0, ?);
        ",
        )
        .bind(channel_id.0 as i64)
        .bind(message_id.0 as i64)
        .bind(author_id.0 as i64)
        .bind(created_at.to_string())
        .execute(&self.pool)
        .await
        .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn is_vote_ended(&self, descriptor: FeatureVoteDescriptor) -> bool {
        debug!("is_vote_ended");

        let FeatureVoteDescriptor(message_id, channel_id) = descriptor;
        sqlx::query("SELECT id FROM feature_message WHERE channel_id = ? AND message_id = ? AND is_vote_ended = 0")
            .bind(channel_id.0 as i64)
            .bind(message_id.0 as i64)
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .is_empty()
    }

    #[instrument(skip(self))]
    pub async fn get_feature_vote(&self, descriptor: FeatureVoteDescriptor) -> Option<FeatureVote> {
        debug!("get_feature_vote");

        let FeatureVoteDescriptor(message_id, channel_id) = descriptor;
        sqlx::query("SELECT * FROM feature_message WHERE channel_id = ? AND message_id = ?")
            .bind(channel_id.0 as i64)
            .bind(message_id.0 as i64)
            .map(|row| FeatureVote {
                author_id: UserId(row.get::<i64, &str>("user_id") as u64),
                created_at: DateTime::from_str(row.get("created_at")).unwrap(),
                descriptor: FeatureVoteDescriptor(
                    MessageId(row.get::<i64, &str>("message_id") as u64),
                    ChannelId(row.get::<i64, &str>("channel_id") as u64),
                ),
            })
            .fetch_one(&self.pool)
            .await
            .ok()
    }

    #[instrument(skip(self))]
    pub async fn end_feature_vote(&self, descriptor: FeatureVoteDescriptor) {
        debug!("end_feature_vote");

        let FeatureVoteDescriptor(message_id, channel_id) = descriptor;
        sqlx::query("UPDATE feature_message SET (is_vote_ended) = (1) WHERE channel_id = ? AND message_id = ?")
            .bind(channel_id.0 as i64)
            .bind(message_id.0 as i64)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}
