use std::str::FromStr;

use super::prelude::*;
use app_shared::chrono::Utc;
use app_shared::models::{AccountId, Rights};
use app_shared::{
    chrono::DateTime,
    models::{ApiToken, Secret},
    serde_json,
};

pub struct TokenTable;

impl TokenTable {
    #[instrument]
    pub async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        trace!("create");

        sqlx::query(
            "
create table if not exists token
(
    secret     text      not null
        constraint token_pk
                    primary key,
    expiration text      null,
    rights     jsonb     not null,
    created_at text      not null,
    creator    bigint    null,
    is_service bool      not null
);
            ",
        )
        .execute(pool)
        .await
    }

    #[instrument]
    pub async fn insert(pool: &Pool<Postgres>, token: ApiToken) -> Result<PgQueryResult, Error> {
        trace!("insert");

        sqlx::query(
            "INSERT INTO token (secret, expiration, rights, created_at, creator, is_service) VALUES ($1, $2, $3, $4, $5, $6)",
        )
            .bind(token.secret.0)
            .bind(token.expiration.map(|date| date.to_string()))
            .bind(serde_json::to_value(&token.rights).unwrap())
            .bind(token.created_at.to_string())
            .bind(token.creator.map(|account_id| account_id.0))
            .bind(token.is_service)
            .execute(pool)
            .await
    }

    #[instrument]
    pub async fn delete_by_secret(
        pool: &Pool<Postgres>,
        api_secret: Secret,
    ) -> Result<PgQueryResult, Error> {
        trace!("delete_by_secret");

        sqlx::query("DELETE FROM token WHERE secret = $1")
            .bind(api_secret.0)
            .execute(pool)
            .await
    }

    #[instrument]
    pub async fn update(
        pool: &Pool<Postgres>,
        new_secret: Secret,
        new_expiration: Option<DateTime<Utc>>,
        new_rights: Rights,
    ) -> Result<PgQueryResult, Error> {
        trace!("update");

        sqlx::query("UPDATE token SET secret = $1, expiration = $2, rights = $3 WHERE id = 1")
            .bind(new_secret.0)
            .bind(new_expiration.map(|date| date.to_string()))
            .bind(serde_json::to_value(new_rights).unwrap())
            .execute(pool)
            .await
    }

    #[instrument]
    pub async fn find_by_secret(
        pool: &Pool<Postgres>,
        api_secret: Secret,
    ) -> Result<Option<ApiToken>, Error> {
        trace!("find_by_secret");

        sqlx::query("SELECT * FROM token WHERE secret = $1")
            .bind(api_secret.0)
            .map(Self::map)
            .fetch_optional(pool)
            .await
    }

    #[instrument(skip(row))]
    fn map(row: PgRow) -> ApiToken {
        ApiToken {
            secret: Secret(row.get::<String, _>("secret")),
            expiration: row
                .get::<Option<String>, _>("expiration")
                .map(|date| DateTime::from_str(&date).unwrap()),
            rights: serde_json::from_value(row.get::<serde_json::Value, _>("rights")).unwrap(),
            created_at: DateTime::from_str(&row.get::<String, _>("created_at")).unwrap(),
            creator: row.get::<Option<i64>, _>("creator").map(AccountId),
            is_service: row.get::<bool, _>("is_service"),
        }
    }
}
