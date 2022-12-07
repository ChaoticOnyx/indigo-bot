use super::prelude::*;
use app_shared::models::{BugReport, BugReportDescriptor};

pub struct BugMessageTable;

impl BugMessageTable {
    #[instrument]
    pub async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        debug!("create");

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
    }

    #[instrument]
    pub async fn insert(
        pool: &Pool<Postgres>,
        bug_report: BugReport,
    ) -> Result<PgQueryResult, Error> {
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
        .execute(pool)
        .await
    }
}
