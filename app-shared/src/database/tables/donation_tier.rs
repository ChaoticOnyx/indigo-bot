use crate::models::{DonationTierId, RoleId};

use super::prelude::*;

#[derive(Debug, Clone)]
pub struct DonationTierTable {
    pub id: DonationTierId,
    pub name: String,
    pub role_id: RoleId,
}

impl DonationTierTable {
    #[instrument]
    pub async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        trace!("create");

        sqlx::query(
            "
create table if not exists donation_tier
(
    id       bigserial not null
        constraint donation_tier_pk
            primary key,
    name     text not null,
    role_id  bigint not null
);
",
        )
        .execute(pool)
        .await
    }

    #[instrument]
    pub async fn insert(
        pool: &Pool<Postgres>,
        table: DonationTierTable,
    ) -> Result<PgQueryResult, Error> {
        trace!("insert");

        let DonationTierTable { id, name, role_id } = table;

        sqlx::query(
            "
INSERT INTO donation_tier (id, name, role_id)
VALUES (DEFAULT, $1, $2);
",
        )
        .bind(id.0)
        .bind(name)
        .bind(role_id.00)
        .execute(pool)
        .await
    }

    #[instrument]
    pub async fn all(pool: &Pool<Postgres>) -> Result<Vec<DonationTierTable>, Error> {
        trace!("all");

        sqlx::query(
            "
SELECT * FROM donation_tier
",
        )
        .map(Self::map)
        .fetch_all(pool)
        .await
    }

    #[instrument]
    pub async fn find_for_roles(
        pool: &Pool<Postgres>,
        roles: &[RoleId],
    ) -> Result<Vec<DonationTierTable>, Error> {
        trace!("find_for_roles");

        let roles: Vec<i64> = roles.iter().map(|role| role.0).collect();

        sqlx::query(
            "
SELECT * FROM donation_tier WHERE role_id = ANY($1)
",
        )
        .bind(roles)
        .map(Self::map)
        .fetch_all(pool)
        .await
    }

    #[instrument(skip(row))]
    fn map(row: PgRow) -> Self {
        Self {
            id: DonationTierId(row.get::<i64, _>("id")),
            name: row.get::<String, _>("name"),
            role_id: RoleId(row.get::<i64, _>("role_id")),
        }
    }
}
