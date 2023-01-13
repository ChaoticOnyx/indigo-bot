use super::prelude::*;
use app_shared::models::{AccountId, AnyUserId};

#[derive(Debug, Clone)]
pub struct AccountIntegrationsTable {
    pub account_id: AccountId,
    pub discord_user_id: DiscordUserId,
    pub byond_ckey: Option<ByondUserId>,
    pub ss14_guid: Option<SS14UserId>,
}

impl AccountIntegrationsTable {
    #[instrument]
    pub async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        trace!("create");

        sqlx::query(
            "
create table if not exists account_integrations
(
    account_id      bigserial not null
        constraint account_integrations_pk
            primary key,
    discord_user_id bigint not null,
    byond_ckey      text,
    ss14_guid       text
);
",
        )
        .execute(pool)
        .await
    }

    #[instrument]
    pub async fn insert(
        pool: &Pool<Postgres>,
        account_integrations: AccountIntegrationsTable,
    ) -> Result<PgQueryResult, Error> {
        trace!("insert");

        sqlx::query("INSERT INTO account_integrations (account_id, discord_user_id, byond_ckey, ss14_guid) VALUES ($1, $2, $3, $4)")
            .bind(account_integrations.account_id.0)
            .bind(account_integrations.discord_user_id.0 as i64)
            .bind(account_integrations.byond_ckey.map(|byond_ckey| byond_ckey.0))
            .bind(account_integrations.ss14_guid.map(|ss14_guid| ss14_guid.0))
            .execute(pool)
            .await
    }

    #[instrument]
    pub async fn find_by_id(
        pool: &Pool<Postgres>,
        user_id: AnyUserId,
    ) -> Result<Option<AccountIntegrationsTable>, Error> {
        trace!("find_by_id");

        let query = match user_id {
            AnyUserId::DiscordId(user_id) => {
                sqlx::query("SELECT * FROM account_integrations WHERE discord_user_id = $1")
                    .bind(user_id.0 as i64)
            }
            AnyUserId::ByondCkey(ckey) => {
                sqlx::query("SELECT * FROM account_integrations WHERE byond_ckey = $1").bind(ckey.0)
            }
            AnyUserId::SS14Guid(guid) => {
                sqlx::query("SELECT * FROM account_integrations WHERE ss14_guid = $1").bind(guid.0)
            }
            AnyUserId::AccountId(id) => {
                sqlx::query("SELECT * FROM account_integrations WHERE account_id = $1").bind(id.0)
            }
        };

        query.map(Self::map).fetch_optional(pool).await
    }

    #[instrument]
    pub async fn set_integration(
        pool: &Pool<Postgres>,
        user_id: AnyUserId,
        integration_id: AnyUserId,
    ) -> Result<PgQueryResult, Error> {
        trace!("connect_account");

        let set_part = match integration_id {
            AnyUserId::DiscordId(_) => "discord_user_id = $1",
            AnyUserId::ByondCkey(_) => "byond_ckey = $1",
            AnyUserId::SS14Guid(_) => "ss14_guid = $1",
            AnyUserId::AccountId(_) => {
                panic!("can't change account id")
            }
        };

        let where_part = match user_id {
            AnyUserId::DiscordId(_) => "discord_user_id = $2",
            AnyUserId::ByondCkey(_) => "byond_ckey = $2",
            AnyUserId::SS14Guid(_) => "ss14_guid = $2",
            AnyUserId::AccountId(_) => "account_id = $2",
        };

        let query_string =
            format!("UPDATE account_integrations SET {set_part} WHERE {where_part};");
        let query = sqlx::query(&query_string);

        // Bind $1
        let query = match integration_id {
            AnyUserId::DiscordId(discord_user_id) => query.bind(discord_user_id.0 as i64),
            AnyUserId::ByondCkey(ckey) => query.bind(ckey.0),
            AnyUserId::SS14Guid(guid) => query.bind(guid.0),
            AnyUserId::AccountId(_) => unreachable!(),
        };

        // Bind $2
        let query = match user_id {
            AnyUserId::DiscordId(discord_user_id) => query.bind(discord_user_id.0 as i64),
            AnyUserId::ByondCkey(ckey) => query.bind(ckey.0),
            AnyUserId::SS14Guid(guid) => query.bind(guid.0),
            AnyUserId::AccountId(id) => query.bind(id.0),
        };

        query.execute(pool).await
    }

    #[instrument(skip(row))]
    fn map(row: PgRow) -> Self {
        Self {
            account_id: AccountId(row.get::<i64, _>("account_id")),
            discord_user_id: DiscordUserId(row.get::<i64, _>("discord_user_id") as u64),
            byond_ckey: row.get::<Option<String>, _>("byond_ckey").map(ByondUserId),
            ss14_guid: row.get::<Option<String>, _>("ss14_guid").map(SS14UserId),
        }
    }
}
