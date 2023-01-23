use super::prelude::*;
use crate::models::{Secret, ServiceId, Webhook, WebhookConfiguration};
use chrono::DateTime;
use serde_json;
use std::str::FromStr;

pub struct WebhookTable;

impl WebhookTable {
    #[instrument]
    pub async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        trace!("create");

        sqlx::query(
            "
create table if not exists webhook
(
    id            bigserial not null
        constraint webhook_pk
            primary key,
    name          text      not null,
    secret        text      not null,
    service_id    text      not null,
    created_at    text      not null,
    configuration jsonb
);
            ",
        )
        .execute(pool)
        .await
    }

    #[instrument]
    pub async fn insert(pool: &Pool<Postgres>, webhook: Webhook) -> Result<PgQueryResult, Error> {
        trace!("insert");

        sqlx::query(
            "INSERT INTO webhook (id, name, secret, service_id, created_at, configuration) VALUES (DEFAULT, $1, $2, $3, $4, $5)",
        )
            .bind(webhook.name)
            .bind(webhook.secret.0)
            .bind(webhook.service_id.0)
            .bind(webhook.created_at.to_string())
            .bind(webhook.configuration.0)
            .execute(pool)
            .await
    }

    #[instrument]
    pub async fn find_by_secret(
        pool: &Pool<Postgres>,
        secret: Secret,
    ) -> Result<Option<Webhook>, Error> {
        trace!("find_by_secret");

        sqlx::query("SELECT * FROM webhook WHERE secret = $1")
            .bind(secret.0)
            .map(Self::map)
            .fetch_optional(pool)
            .await
    }

    #[instrument]
    pub async fn delete_by_secret(
        pool: &Pool<Postgres>,
        secret: Secret,
    ) -> Result<PgQueryResult, Error> {
        trace!("delete_by_secret");

        sqlx::query("DELETE FROM webhook WHERE secret = $1")
            .bind(secret.0)
            .execute(pool)
            .await
    }

    #[instrument(skip(row))]
    fn map(row: PgRow) -> Webhook {
        Webhook {
            name: row.get::<String, _>("name"),
            secret: Secret(row.get::<String, _>("secret")),
            service_id: ServiceId(row.get::<String, _>("service_id")),
            created_at: DateTime::from_str(&row.get::<String, _>("created_at")).unwrap(),
            configuration: WebhookConfiguration(row.get::<serde_json::Value, _>("configuration")),
        }
    }
}
