use super::prelude::*;
use app_shared::chrono::DateTime;
use app_shared::models::{AccountId, Secret, Session};
use std::str::FromStr;

pub struct SessionTable;

impl SessionTable {
    #[instrument]
    pub async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        trace!("create");

        sqlx::query(
            "
create table if not exists session
(
    secret     text      not null
        constraint session_pk
                    primary key,
    api_secret text   not null,
    account_id bigint not null,
    created_at text   not null,
    expiration text   not null,
    user_agent text   not null,
    ip         text   not null
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
            "INSERT INTO session (secret, api_secret, account_id, created_at, expiration, user_agent, ip) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
            .bind(session.secret.0)
            .bind(session.api_secret.0)
            .bind(session.account_id.0)
            .bind(session.created_at.to_string())
            .bind(session.expiration.to_string())
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

    #[instrument(skip(row))]
    fn map(row: PgRow) -> Session {
        Session {
            secret: Secret(row.get::<String, _>("secret")),
            api_secret: Secret(row.get::<String, _>("api_secret")),
            account_id: AccountId(row.get::<i64, _>("account_id")),
            created_at: DateTime::from_str(&row.get::<String, _>("created_at")).unwrap(),
            expiration: DateTime::from_str(&row.get::<String, _>("expiration")).unwrap(),
            user_agent: row.get::<String, _>("user_agent"),
            ip: row.get::<String, _>("ip"),
        }
    }
}
