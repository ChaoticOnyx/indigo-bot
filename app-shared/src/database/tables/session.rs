use super::prelude::*;
use crate::models::{AccountId, Secret, Session};
use chrono::{DateTime, Utc};

pub struct SessionTable;

impl SessionTable {
    #[instrument]
    pub async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        trace!("create");

        sqlx::query(
            "
create table if not exists session
(
    secret      text      not null
        constraint session_pk
                    primary key,
    api_secret  text        not null,
	csrf_secret text        not null,
    account_id  bigint      not null,
    created_at  timestamptz not null,
    expiration  timestamptz not null,
    user_agent  text        not null,
    ip          text        not null
);
",
        )
        .execute(pool)
        .await
    }

    #[instrument]
    pub async fn insert(pool: &Pool<Postgres>, session: Session) -> Result<PgQueryResult, Error> {
        trace!("insert");

        sqlx::query(
            "INSERT INTO session (secret, api_secret, csrf_secret, account_id, created_at, expiration, user_agent, ip) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
            .bind(session.secret.0)
            .bind(session.api_secret.0)
            .bind(session.csrf_token.0)
            .bind(session.account_id.0)
            .bind(session.created_at)
            .bind(session.expiration)
            .bind(session.user_agent)
            .bind(session.ip)
            .execute(pool)
            .await
    }

    #[instrument]
    pub async fn get_all(pool: &Pool<Postgres>) -> Result<Vec<Session>, Error> {
        trace!("get_all");

        sqlx::query("SELECT * FROM session")
            .map(Self::map)
            .fetch_all(pool)
            .await
    }

    #[instrument]
    pub async fn delete_by_secret(
        pool: &Pool<Postgres>,
        session_secret: Secret,
    ) -> Result<PgQueryResult, Error> {
        trace!("delete_by_secret");

        sqlx::query("DELETE FROM session WHERE secret = $1")
            .bind(session_secret.0)
            .execute(pool)
            .await
    }

    #[instrument]
    pub async fn find_by_secret(
        pool: &Pool<Postgres>,
        session_secret: Secret,
    ) -> Result<Option<Session>, Error> {
        trace!("find_by_secret");

        sqlx::query("SELECT * FROM session WHERE secret = $1")
            .bind(session_secret.0)
            .map(Self::map)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_account_id(
        pool: &Pool<Postgres>,
        account_id: AccountId,
    ) -> Result<Vec<Session>, Error> {
        trace!("find_by_account_id");

        sqlx::query("SELECT * FROM session WHERE account_id = $1")
            .bind(account_id.0)
            .map(Self::map)
            .fetch_all(pool)
            .await
    }

    #[instrument]
    pub async fn find_by_csrf_secret(
        pool: &Pool<Postgres>,
        csrf_secret: Secret,
    ) -> Result<Option<Session>, Error> {
        trace!("find_by_csrf_secret");

        sqlx::query("SELECT * FROM session WHERE csrf_secret = $1")
            .bind(csrf_secret.0)
            .map(Self::map)
            .fetch_optional(pool)
            .await
    }

    #[instrument(skip(row))]
    fn map(row: PgRow) -> Session {
        Session {
            secret: Secret(row.get::<String, _>("secret")),
            api_secret: Secret(row.get::<String, _>("api_secret")),
            csrf_token: Secret(row.get::<String, _>("csrf_secret")),
            account_id: AccountId(row.get::<i64, _>("account_id")),
            created_at: row.get::<DateTime<Utc>, _>("created_at"),
            expiration: row.get::<DateTime<Utc>, _>("expiration"),
            user_agent: row.get::<String, _>("user_agent"),
            ip: row.get::<String, _>("ip"),
        }
    }
}
