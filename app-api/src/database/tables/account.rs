use super::prelude::*;
use app_shared::{
    chrono::{DateTime, Utc},
    models::{AccountId, Role, RoleId},
};
use std::str::FromStr;

pub struct AccountTable {
    pub id: AccountId,
    pub username: String,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,
    pub roles: Vec<RoleId>,
}

impl AccountTable {
    #[instrument]
    pub async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        trace!("create");

        sqlx::query(
            "
create table if not exists account
(
    id         bigserial not null
        constraint account_pk
            primary key,
    username   text not null,
    avatar_url text not null,
    created_at text not null,
    roles      bigint[] not null
);
",
        )
        .execute(pool)
        .await
    }

    #[instrument]
    pub async fn insert(
        pool: &Pool<Postgres>,
        username: String,
        avatar_url: String,
        created_at: DateTime<Utc>,
        roles: &[Role],
    ) -> Result<AccountId, Error> {
        trace!("insert");

        sqlx::query(
            "
INSERT INTO account (id, username, avatar_url, created_at, roles)
VALUES (DEFAULT, $1, $2, $3, $4)
RETURNING id
",
        )
        .bind(username)
        .bind(avatar_url)
        .bind(created_at.to_string())
        .bind(
            roles
                .iter()
                .map(|role| role.id.0 as i64)
                .collect::<Vec<i64>>(),
        )
        .map(|row| AccountId(row.get::<i64, _>("id")))
        .fetch_one(pool)
        .await
    }

    #[instrument]
    pub async fn all(pool: &Pool<Postgres>) -> Result<Vec<AccountTable>, Error> {
        trace!("all");

        sqlx::query("SELECT * FROM account")
            .map(Self::map)
            .fetch_all(pool)
            .await
    }

    #[instrument]
    pub async fn find_by_id(
        pool: &Pool<Postgres>,
        account_id: AccountId,
    ) -> Result<Option<AccountTable>, Error> {
        trace!("find_by_id");

        sqlx::query("SELECT * FROM account WHERE id = $1")
            .bind(account_id.0)
            .map(Self::map)
            .fetch_optional(pool)
            .await
    }

    #[instrument]
    pub async fn find_many_by_role(
        pool: &Pool<Postgres>,
        role_id: RoleId,
    ) -> Result<Vec<AccountTable>, Error> {
        trace!("find_many_by_role");

        sqlx::query("SELECT * FROM account WHERE $1 = ANY(roles)")
            .bind(role_id.0)
            .map(Self::map)
            .fetch_all(pool)
            .await
    }

    #[instrument]
    pub async fn find_by_username(
        pool: &Pool<Postgres>,
        username: String,
    ) -> Result<Option<AccountTable>, Error> {
        trace!("find_by_username");

        sqlx::query("SELECT * FROM account WHERE username = $1")
            .bind(username)
            .map(Self::map)
            .fetch_optional(pool)
            .await
    }

    #[instrument]
    pub async fn add_role(
        pool: &Pool<Postgres>,
        account_id: AccountId,
        role_id: RoleId,
    ) -> Result<PgQueryResult, Error> {
        sqlx::query("UPDATE account SET roles = roles || $1 WHERE id = $2")
            .bind(role_id.0)
            .bind(account_id.0)
            .execute(pool)
            .await
    }

    #[instrument]
    pub async fn remove_role(
        pool: &Pool<Postgres>,
        account_id: AccountId,
        role_id: RoleId,
    ) -> Result<PgQueryResult, Error> {
        sqlx::query("UPDATE account SET roles = array_remove(roles, $1) WHERE id = $2")
            .bind(role_id.0)
            .bind(account_id.0)
            .execute(pool)
            .await
    }

    pub async fn update_username(
        pool: &Pool<Postgres>,
        account_id: AccountId,
        new_username: String,
    ) -> Result<PgQueryResult, Error> {
        sqlx::query("UPDATE account SET username = $1 WHERE id = $2")
            .bind(new_username)
            .bind(account_id.0)
            .execute(pool)
            .await
    }

    pub async fn update_avatar_url(
        pool: &Pool<Postgres>,
        account_id: AccountId,
        new_avatar_url: String,
    ) -> Result<PgQueryResult, Error> {
        sqlx::query("UPDATE account SET avatar_url = $1 WHERE id = $2")
            .bind(new_avatar_url)
            .bind(account_id.0)
            .execute(pool)
            .await
    }

    #[instrument(skip(row))]
    fn map(row: PgRow) -> Self {
        Self {
            id: AccountId(row.get::<i64, _>("id")),
            username: row.get::<String, _>("username"),
            avatar_url: row.get::<String, _>("avatar_url"),
            created_at: DateTime::from_str(&row.get::<String, _>("created_at")).unwrap(),
            roles: row
                .get::<Vec<i64>, _>("roles")
                .into_iter()
                .map(RoleId)
                .collect(),
        }
    }
}
