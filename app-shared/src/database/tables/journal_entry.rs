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
