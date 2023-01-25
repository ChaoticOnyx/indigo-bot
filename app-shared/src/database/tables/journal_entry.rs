use chrono::{DateTime, Utc};
use serde_json;

use crate::models::{ActionType, Actor, JournalEntry, JournalEntryId};

use super::prelude::*;

pub struct JournalEntryTable;

impl JournalEntryTable {
    #[instrument]
    pub async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        trace!("create");

        sqlx::query(
            "
create table if not exists journal_entry
(
    id       bigserial not null
        constraint journal_entry_pk
            primary key,
    object   jsonb not null,
	datetime timestamptz not null,
	subject  jsonb,
	action jsonb not null
);
",
        )
        .execute(pool)
        .await
    }

    #[instrument]
    pub async fn insert(
        pool: &Pool<Postgres>,
        object: Actor,
        datetime: DateTime<Utc>,
        subject: Option<Actor>,
        action: ActionType,
    ) -> Result<JournalEntry, Error> {
        trace!("insert");

        sqlx::query("INSERT INTO journal_entry (id, object, datetime, subject, action) VALUES(DEFAULT, $1, $2, $3, $4) RETURNING *")
            .bind(serde_json::to_value(&object).unwrap())
            .bind(datetime)
            .bind(subject.and_then(|value| serde_json::to_value(value).ok()))
            .bind(serde_json::to_value(&action).unwrap())
            .map(Self::map)
            .fetch_one(pool).await
    }

    #[instrument]
    pub async fn find_cursor_entries(
        pool: &Pool<Postgres>,
        offset: usize,
        max_count: usize,
        subject: Option<Actor>,
    ) -> Result<Vec<JournalEntry>, Error> {
        trace!("find_cursor_entries");

        let query = if let Some(subject) = subject {
            sqlx::query("SELECT * FROM journal_entry WHERE $1 @> subject ORDER BY datetime DESC LIMIT $2 OFFSET $3")
				.bind(serde_json::to_value(subject).unwrap())
				.bind(max_count as i64)
                .bind(offset as i64)
        } else {
            sqlx::query(
                "SELECT * FROM journal_entry WHERE ORDER BY datetime DESC LIMIT $1 OFFSET $2",
            )
            .bind(max_count as i64)
            .bind(offset as i64)
        };

        query.map(Self::map).fetch_all(pool).await
    }

    #[instrument]
    pub async fn count_total_entries(
        pool: &Pool<Postgres>,
        subject: Option<Actor>,
    ) -> Result<usize, Error> {
        trace!("count_total_entries");

        let query = if let Some(subject) = subject {
            sqlx::query("SELECT COUNT(*) FROM journal_entry WHERE $1 @> subject")
                .bind(serde_json::to_value(subject).unwrap())
        } else {
            sqlx::query("SELECT COUNT(*) FROM journal_entry")
        };

        query
            .map(|row| row.get::<i64, _>("count") as usize)
            .fetch_one(pool)
            .await
    }

    #[instrument(skip(row))]
    fn map(row: PgRow) -> JournalEntry {
        JournalEntry {
            id: JournalEntryId(row.get::<i64, _>("id")),
            object: serde_json::from_value(row.get::<serde_json::Value, _>("object")).unwrap(),
            datetime: row.get::<DateTime<Utc>, _>("datetime"),
            subject: row
                .get::<Option<serde_json::Value>, _>("subject")
                .map(|value| serde_json::from_value(value).unwrap()),
            action: serde_json::from_value(row.get::<serde_json::Value, _>("action")).unwrap(),
        }
    }
}
