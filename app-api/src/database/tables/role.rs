use super::prelude::*;
use app_shared::{
    hex_color::HexColor,
    models::{Role, RoleId},
    serde_json,
};

pub struct RoleTable;

impl RoleTable {
    #[instrument]
    pub async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        trace!("create");

        sqlx::query(
            "
create table if not exists role
(
    id     bigserial not null
        constraint role_pk
            primary key,
    name   text not null,
    color  bigint not null,
    rights jsonb not null
);
",
        )
        .execute(pool)
        .await
    }

    #[instrument]
    pub async fn insert(pool: &Pool<Postgres>, role: Role) -> Result<PgQueryResult, Error> {
        trace!("insert");

        sqlx::query("INSERT INTO role (id, name, color, rights) VALUES (DEFAULT, $1, $2, $3)")
            .bind(role.name)
            .bind(role.color.to_u24() as i32)
            .bind(serde_json::to_value(&role.rights).unwrap())
            .execute(pool)
            .await
    }

    #[instrument]
    pub async fn find_by_id(pool: &Pool<Postgres>, id: RoleId) -> Result<Option<Role>, Error> {
        trace!("find_by_id");

        sqlx::query("SELECT * FROM role WHERE id = $1")
            .bind(id.0)
            .map(Self::map)
            .fetch_optional(pool)
            .await
    }

    #[instrument]
    pub async fn find_by_ids(pool: &Pool<Postgres>, ids: &[RoleId]) -> Result<Vec<Role>, Error> {
        trace!("find_by_ids");

        sqlx::query("SELECT * FROM role WHERE id = ANY ($1);")
            .bind(ids.iter().map(|role_id| role_id.0).collect::<Vec<i64>>())
            .map(Self::map)
            .fetch_all(pool)
            .await
    }

    #[instrument(skip(row))]
    fn map(row: PgRow) -> Role {
        Role {
            id: RoleId(row.get::<i64, _>("id")),
            name: row.get::<String, _>("name"),
            color: HexColor::from_u24(row.get::<i64, _>("color") as u32),
            rights: serde_json::from_value(row.get::<serde_json::Value, _>("rights")).unwrap(),
        }
    }
}
