use crate::prelude::*;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serenity::{
    model::id::{ChannelId, MessageId, UserId},
    prelude::Mutex,
};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Row, Sqlite};

static DB: Lazy<Mutex<Option<Database>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Clone)]
pub struct Database {
    pool: Pool<Sqlite>,
}

#[async_trait]
impl GlobalState for Database {
    #[instrument]
    async fn get_state() -> Database {
        let lock = DB.lock().await;

        lock.clone().unwrap()
    }

    #[instrument]
    async fn set_state(db: Database) {
        let mut lock = DB.lock().await;

        *lock = Some(db);
    }
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
    pub async fn add_bug_message(&self, user_id: UserId, issue_number: i64) {
        info!("add_bug_message");

        sqlx::query(
            "
			INSERT INTO bug_message (user_id, issue_number)
			VALUES (?, ?);
			",
        )
        .bind(user_id.0 as i64)
        .bind(issue_number as i64)
        .execute(&self.pool)
        .await
        .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn add_feature_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        user_id: UserId,
        created_ad: DateTime<Utc>,
    ) {
        info!("add_feature_message");

        sqlx::query(
            "
            INSERT INTO feature_message (channel_id, message_id, user_id, is_vote_ended, created_at)
            VALUES (?, ?, ?, 0, ?);
        ",
        )
        .bind(channel_id.0 as i64)
        .bind(message_id.0 as i64)
        .bind(user_id.0 as i64)
        .bind(created_ad.to_string())
        .execute(&self.pool)
        .await
        .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn has_not_vote_ended_feature_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> bool {
        debug!("has_not_vote_ended_feature_message");

        !sqlx::query("SELECT id FROM feature_message WHERE channel_id = ? AND message_id = ? AND is_vote_ended = 0")
            .bind(channel_id.0 as i64)
            .bind(message_id.0 as i64)
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .is_empty()
    }

    #[instrument(skip(self))]
    pub async fn get_feature_message_author(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> UserId {
        debug!("get_feature_message_author");

        let row = sqlx::query(
            "SELECT user_id FROM feature_message WHERE channel_id = ? AND message_id = ?",
        )
        .bind(channel_id.0 as i64)
        .bind(message_id.0 as i64)
        .fetch_one(&self.pool)
        .await
        .unwrap();

        let id: i64 = row.get(0);

        UserId(id as u64)
    }

    #[instrument(skip(self))]
    pub async fn end_vote_feature_message(&self, channel_id: ChannelId, message_id: MessageId) {
        debug!("end_vote_feature_message");

        sqlx::query("UPDATE feature_message SET (is_vote_ended) = (1) WHERE channel_id = ? AND message_id = ?")
            .bind(channel_id.0 as i64)
            .bind(message_id.0 as i64)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}
