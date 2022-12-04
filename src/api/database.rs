use std::str::FromStr;

use chrono::DateTime;
use serenity::model::prelude::{ChannelId, MessageId};
use sqlx::{postgres::PgPoolOptions, Postgres};
use sqlx::{Pool, Row};

use crate::api::models::{Account, AnyUserId, ApiToken, NewAccount, Secret, ServiceId, Webhook};
use crate::{
    api::models::{BugReport, BugReportDescriptor, FeatureVote, FeatureVoteDescriptor},
    prelude::*,
};

#[derive(Debug)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    #[instrument]
    pub async fn connect(conn: &str) -> Self {
        info!("setting up database");

        let pool = PgPoolOptions::new().connect(conn).await.unwrap();

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
    async fn migration_init(pool: &Pool<Postgres>) {
        info!("migration: migration_init");

        // TABLE feature_message
        sqlx::query(
            "
create table if not exists feature_message
(
    id            bigserial not null
        constraint feature_message_pk
            primary key,
    channel_id    bigint not null,
    message_id    bigint not null,
    user_id       bigint not null,
    is_vote_ended boolean not null,
    created_at    text    not null
);
",
        )
        .execute(pool)
        .await
        .unwrap();

        // TABLE bug_message
        sqlx::query(
            "
create table if not exists bug_message
(
    id           bigserial not null
        constraint bug_message_pk
            primary key,
    user_id      bigint not null,
    issue_number bigint not null
);
",
        )
        .execute(pool)
        .await
        .unwrap();

        // TABLE account
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
    created_at text not null
);
",
        )
        .execute(pool)
        .await
        .unwrap();

        // TABLE token
        sqlx::query(
            "
create table if not exists token
(
    id         bigserial not null
        constraint token_pk
            primary key,
    secret     text      not null,
    expiration text      null,
    rights     jsonb     not null,
    created_at text      not null
);
            ",
        )
        .execute(pool)
        .await
        .unwrap();

        // TABLE webhook
        sqlx::query(
            "
create table if not exists webhook
(
    id         bigserial not null
        constraint webhook_pk
            primary key,
    secret     text      not null,
    service_id text      not null,
    created_at text      not null
);
            ",
        )
        .execute(pool)
        .await
        .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn add_webhook(&self, webhook: Webhook) {
        info!("add_webhook");

        sqlx::query(
            "INSERT INTO webhook (id, secret, service_id, created_at) VALUES (DEFAULT, $1, $2, $3)",
        )
        .bind(webhook.secret.0)
        .bind(webhook.service_id.0)
        .bind(webhook.created_at.to_string())
        .execute(&self.pool)
        .await
        .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn find_webhook_by_secret(&self, secret: Secret) -> Option<Webhook> {
        debug!("find_webhook_by_secret");

        sqlx::query("SELECT * FROM webhook WHERE secret = $1")
            .bind(secret.0)
            .map(|row| Webhook {
                secret: Secret(row.get::<String, _>("secret")),
                service_id: ServiceId(row.get::<String, _>("service_id")),
                created_at: DateTime::from_str(&row.get::<String, _>("created_at")).unwrap(),
            })
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    #[instrument(skip(self))]
    pub async fn delete_webhook_by_secret(&self, secret: Secret) {
        info!("delete_webhook_by_secret");

        sqlx::query("DELETE FROM webhook WHERE secret = $1")
            .bind(secret.0)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn update_root_token(&self, token: ApiToken) {
        debug!("update_root_token");

        let has_token = sqlx::query("SELECT * FROM token WHERE id = 1")
            .fetch_optional(&self.pool)
            .await
            .unwrap()
            .is_some();

        if has_token {
            debug!("updating root token");

            sqlx::query("UPDATE token SET secret = $1, expiration = $2, rights = $3 WHERE id = 1")
                .bind(token.secret.0)
                .bind(token.expiration.map(|date| date.to_string()))
                .bind(serde_json::to_value(&token.rights).unwrap())
                .execute(&self.pool)
                .await
                .unwrap();
        } else {
            debug!("creating new root token");

            sqlx::query(
                "INSERT INTO token (id, secret, expiration, rights, created_at) VALUES (DEFAULT, $1, $2, $3, $4)",
            )
            .bind(token.secret.0)
            .bind(token.expiration.map(|date| date.to_string()))
            .bind(serde_json::to_value(&token.rights).unwrap())
            .bind(token.created_at.to_string())
            .execute(&self.pool)
            .await
            .unwrap();
        }
    }

    #[instrument(skip(self))]
    pub async fn add_api_token(&self, token: ApiToken) {
        info!("add_api_token");

        sqlx::query(
            "INSERT INTO token (id, secret, expiration, rights, created_at) VALUES (DEFAULT, $1, $2, $3, $4)",
        )
        .bind(token.secret.0)
        .bind(token.expiration.map(|date| date.to_string()))
        .bind(serde_json::to_value(&token.rights).unwrap())
        .bind(token.created_at.to_string())
        .execute(&self.pool)
        .await
        .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn delete_api_token_by_secret(&self, secret: Secret) {
        info!("remove_api_token");

        sqlx::query("DELETE FROM token WHERE secret = $1")
            .bind(secret.0)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn find_api_token_by_secret(&self, api_secret: Secret) -> Option<ApiToken> {
        debug!("find_api_token_by_secret");

        let token = sqlx::query("SELECT * FROM token WHERE secret = $1")
            .bind(api_secret.0)
            .map(|row| ApiToken {
                secret: Secret(row.get::<String, _>("secret")),
                expiration: row
                    .get::<Option<String>, _>("expiration")
                    .map(|date| DateTime::from_str(&date).unwrap()),
                rights: serde_json::from_value(row.get::<serde_json::Value, _>("rights")).unwrap(),
                created_at: DateTime::from_str(&row.get::<String, _>("created_at")).unwrap(),
            })
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        token
    }

    #[instrument(skip(self))]
    pub async fn find_account(&self, user_id: AnyUserId) -> Option<Account> {
        debug!("find_account");

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
            AnyUserId::InternalId(id) => {
                sqlx::query("SELECT * FROM account WHERE id = $1").bind(id as i64)
            }
        };

        query
            .map(|row| Account {
                id: row.get::<i64, _>("id") as u64,
                discord_id: discord::id::UserId(row.get::<i64, _>("discord_id") as u64),
                byond_ckey: row
                    .get::<Option<String>, _>("byond_ckey")
                    .map(|ckey| byond::UserId(ckey)),
                ss14_guid: row
                    .get::<Option<String>, _>("ss14_guid")
                    .map(|guid| ss14::UserId(guid)),
                created_at: DateTime::from_str(&row.get::<String, _>("created_at")).unwrap(),
            })
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    #[instrument(skip(self))]
    pub async fn add_account(&self, new_user: NewAccount) {
        info!("add_account");

        let NewAccount {
            discord_id,
            created_at,
        } = new_user;

        sqlx::query(
            "
INSERT INTO account (id, discord_id, byond_ckey, ss14_guid, created_at)
VALUES (DEFAULT, $1, null, null, $2)
",
        )
        .bind(discord_id.0 as i64)
        .bind(created_at.to_string())
        .execute(&self.pool)
        .await
        .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn connect_account(&self, user_id: AnyUserId, new_user_id: AnyUserId) {
        info!("connect_account");

        let set_part = match new_user_id {
            AnyUserId::DiscordId(_) => "discord_id = $1",
            AnyUserId::ByondCkey(_) => "byond_ckey = $1",
            AnyUserId::SS14Guid(_) => "ss14_guid = $1",
            AnyUserId::InternalId(_) => {
                panic!("can't change internal id")
            }
        };

        let where_part = match user_id {
            AnyUserId::DiscordId(_) => "discord_id = $2",
            AnyUserId::ByondCkey(_) => "byond_ckey = $2",
            AnyUserId::SS14Guid(_) => "ss14_guid = $2",
            AnyUserId::InternalId(_) => "id = $2",
        };

        let query_string = format!("UPDATE account SET {set_part} WHERE {where_part};");
        let query = sqlx::query(&query_string);

        // Bind $1
        let query = match new_user_id {
            AnyUserId::DiscordId(discord_id) => query.bind(discord_id.0 as i64),
            AnyUserId::ByondCkey(ckey) => query.bind(ckey.0),
            AnyUserId::SS14Guid(guid) => query.bind(guid.0),
            AnyUserId::InternalId(_) => unreachable!(),
        };

        // Bind $2
        let query = match user_id {
            AnyUserId::DiscordId(discord_id) => query.bind(discord_id.0 as i64),
            AnyUserId::ByondCkey(ckey) => query.bind(ckey.0),
            AnyUserId::SS14Guid(guid) => query.bind(guid.0),
            AnyUserId::InternalId(id) => query.bind(id as i64),
        };

        query.execute(&self.pool).await.unwrap();
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
INSERT INTO bug_message (id, user_id, issue_number)
VALUES (DEFAULT, $1, $2);
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
INSERT INTO feature_message (id, channel_id, message_id, user_id, is_vote_ended, created_at)
VALUES (DEFAULT, $1, $2, $3, false, $4);
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
        sqlx::query("SELECT id FROM feature_message WHERE channel_id = $1 AND message_id = $2 AND is_vote_ended = false")
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
        sqlx::query("SELECT * FROM feature_message WHERE channel_id = $1 AND message_id = $2")
            .bind(channel_id.0 as i64)
            .bind(message_id.0 as i64)
            .map(|row| FeatureVote {
                author_id: discord::id::UserId(row.get::<i64, _>("user_id") as u64),
                created_at: DateTime::from_str(row.get("created_at")).unwrap(),
                descriptor: FeatureVoteDescriptor(
                    MessageId(row.get::<i64, _>("message_id") as u64),
                    ChannelId(row.get::<i64, _>("channel_id") as u64),
                ),
            })
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    #[instrument(skip(self))]
    pub async fn end_feature_vote(&self, descriptor: FeatureVoteDescriptor) {
        debug!("end_feature_vote");

        let FeatureVoteDescriptor(message_id, channel_id) = descriptor;
        sqlx::query("UPDATE feature_message SET is_vote_ended = true WHERE channel_id = $1 AND message_id = $2")
            .bind(channel_id.0 as i64)
            .bind(message_id.0 as i64)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}
