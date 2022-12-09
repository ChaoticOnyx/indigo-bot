use super::prelude::*;
use app_shared::{
    chrono::{DateTime, Utc},
    models::{Account, AccountId, AnyUserId, Role, RoleId},
};
use std::str::FromStr;

pub struct AccountTable;

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
    discord_id bigint not null,
    byond_ckey text,
    ss14_guid  text,
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
        discord_user_id: DiscordUserId,
        created_at: DateTime<Utc>,
        roles: &[Role],
    ) -> Result<PgQueryResult, Error> {
        trace!("insert");

        sqlx::query(
            "
INSERT INTO account (id, discord_id, byond_ckey, ss14_guid, created_at, roles)
VALUES (DEFAULT, $1, null, null, $2, $3)
",
        )
        .bind(discord_user_id.0 as i64)
        .bind(created_at.to_string())
        .bind(
            roles
                .iter()
                .map(|role| role.id.0 as i64)
                .collect::<Vec<i64>>(),
        )
        .execute(pool)
        .await
    }

    #[instrument]
    pub async fn find_by_user_id(
        pool: &Pool<Postgres>,
        user_id: AnyUserId,
    ) -> Result<Option<Account>, Error> {
        trace!("find_by_user_id");

        let query = match user_id {
            AnyUserId::DiscordId(user_id) => {
                sqlx::query("SELECT * FROM account WHERE discord_id = $1").bind(user_id.0 as i64)
            }
            AnyUserId::ByondCkey(ckey) => {
                sqlx::query("SELECT * FROM account WHERE byond_ckey = $1").bind(ckey.0)
            }
            AnyUserId::SS14Guid(guid) => {
                sqlx::query("SELECT * FROM account WHERE ss14_guid = $1").bind(guid.0)
            }
            AnyUserId::AccountId(id) => {
                sqlx::query("SELECT * FROM account WHERE id = $1").bind(id.0 as i64)
            }
        };

        query.map(Self::map).fetch_optional(pool).await
    }

    #[instrument]
    pub async fn update_user_id(
        pool: &Pool<Postgres>,
        user_id: AnyUserId,
        new_user_id: AnyUserId,
    ) -> Result<PgQueryResult, Error> {
        trace!("connect_account");

        let set_part = match new_user_id {
            AnyUserId::DiscordId(_) => "discord_id = $1",
            AnyUserId::ByondCkey(_) => "byond_ckey = $1",
            AnyUserId::SS14Guid(_) => "ss14_guid = $1",
            AnyUserId::AccountId(_) => {
                panic!("can't change account id")
            }
        };

        let where_part = match user_id {
            AnyUserId::DiscordId(_) => "discord_id = $2",
            AnyUserId::ByondCkey(_) => "byond_ckey = $2",
            AnyUserId::SS14Guid(_) => "ss14_guid = $2",
            AnyUserId::AccountId(_) => "id = $2",
        };

        let query_string = format!("UPDATE account SET {set_part} WHERE {where_part};");
        let query = sqlx::query(&query_string);

        // Bind $1
        let query = match new_user_id {
            AnyUserId::DiscordId(discord_id) => query.bind(discord_id.0 as i64),
            AnyUserId::ByondCkey(ckey) => query.bind(ckey.0),
            AnyUserId::SS14Guid(guid) => query.bind(guid.0),
            AnyUserId::AccountId(_) => unreachable!(),
        };

        // Bind $2
        let query = match user_id {
            AnyUserId::DiscordId(discord_id) => query.bind(discord_id.0 as i64),
            AnyUserId::ByondCkey(ckey) => query.bind(ckey.0),
            AnyUserId::SS14Guid(guid) => query.bind(guid.0),
            AnyUserId::AccountId(id) => query.bind(id.0 as i64),
        };

        query.execute(pool).await
    }

    #[instrument]
    pub async fn add_role(
        pool: &Pool<Postgres>,
        user_id: AnyUserId,
        role_id: RoleId,
    ) -> Result<PgQueryResult, Error> {
        let query = match user_id {
            AnyUserId::DiscordId(_) => {
                sqlx::query("UPDATE account SET roles = roles || $1 WHERE discord_id = $2")
            }
            AnyUserId::ByondCkey(_) => {
                sqlx::query("UPDATE account SET roles = roles || $1 WHERE byond_ckey = $2")
            }
            AnyUserId::SS14Guid(_) => {
                sqlx::query("UPDATE account SET roles = roles || $1 WHERE ss14_guid = $2")
            }
            AnyUserId::AccountId(_) => {
                sqlx::query("UPDATE account SET roles = roles || $1 WHERE id = $2")
            }
        }
        .bind(role_id.0);

        let query = match user_id {
            AnyUserId::DiscordId(discord_id) => query.bind(discord_id.0 as i64),
            AnyUserId::ByondCkey(ckey) => query.bind(ckey.0),
            AnyUserId::SS14Guid(ss14_guid) => query.bind(ss14_guid.0),
            AnyUserId::AccountId(id) => query.bind(id.0),
        };

        query.execute(pool).await
    }

    #[instrument(skip(row))]
    fn map(row: PgRow) -> Account {
        Account {
            id: AccountId(row.get::<i64, _>("id")),
            discord_id: DiscordUserId(row.get::<i64, _>("discord_id") as u64),
            byond_ckey: row.get::<Option<String>, _>("byond_ckey").map(ByondUserId),
            ss14_guid: row.get::<Option<String>, _>("ss14_guid").map(SS14UserId),
            created_at: DateTime::from_str(&row.get::<String, _>("created_at")).unwrap(),
            roles: row
                .get::<Vec<i64>, _>("roles")
                .into_iter()
                .map(RoleId)
                .collect(),
        }
    }
}
