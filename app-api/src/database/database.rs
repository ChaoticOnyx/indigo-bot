use crate::database::tables::{
    AccountTable, BugMessageTable, FeatureMessageTable, TokenTable, WebhookTable,
};
use app_shared::chrono::Utc;
use app_shared::{
    chrono::DateTime,
    models::{
        Account, AnyUserId, ApiToken, BugReport, FeatureVote, FeatureVoteDescriptor, Secret,
        Webhook,
    },
    prelude::*,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

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

        FeatureMessageTable::create(pool).await.unwrap();
        BugMessageTable::create(pool).await.unwrap();
        AccountTable::create(pool).await.unwrap();
        TokenTable::create(pool).await.unwrap();
        WebhookTable::create(pool).await.unwrap();
    }

    #[instrument(skip(self))]
    pub async fn add_webhook(&self, webhook: Webhook) {
        trace!("add_webhook");

        WebhookTable::insert(&self.pool, webhook).await.unwrap();
    }

    #[instrument(skip(self))]
    pub async fn find_webhook_by_secret(&self, secret: Secret) -> Option<Webhook> {
        trace!("find_webhook_by_secret");

        WebhookTable::find_by_secret(&self.pool, secret)
            .await
            .unwrap()
    }

    #[instrument(skip(self))]
    pub async fn delete_webhook_by_secret(&self, secret: Secret) {
        trace!("delete_webhook_by_secret");

        WebhookTable::delete_by_secret(&self.pool, secret)
            .await
            .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn update_root_token(&self, token: ApiToken) {
        trace!("update_root_token");

        let has_token = TokenTable::find_by_id(&self.pool, 1)
            .await
            .unwrap()
            .is_some();

        if has_token {
            debug!("updating root token");

            TokenTable::update(&self.pool, token.secret, token.expiration, token.rights)
                .await
                .unwrap();
        } else {
            debug!("creating new root token");

            TokenTable::insert(&self.pool, token).await.unwrap();
        }
    }

    #[instrument(skip(self))]
    pub async fn add_api_token(&self, token: ApiToken) {
        trace!("add_api_token");

        TokenTable::insert(&self.pool, token).await.unwrap();
    }

    #[instrument(skip(self))]
    pub async fn delete_api_token_by_secret(&self, secret: Secret) {
        trace!("remove_api_token");

        TokenTable::delete_by_secret(&self.pool, secret)
            .await
            .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn find_api_token_by_secret(&self, api_secret: Secret) -> Option<ApiToken> {
        trace!("find_api_token_by_secret");

        TokenTable::find_by_secret(&self.pool, api_secret)
            .await
            .unwrap()
    }

    #[instrument(skip(self))]
    pub async fn find_account(&self, user_id: AnyUserId) -> Option<Account> {
        trace!("find_account");

        AccountTable::find_by_user_id(&self.pool, user_id)
            .await
            .unwrap()
    }

    #[instrument(skip(self))]
    pub async fn add_account(&self, discord_user_id: DiscordUserId, created_at: DateTime<Utc>) {
        trace!("add_account");

        AccountTable::insert(&self.pool, discord_user_id, created_at)
            .await
            .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn connect_account(&self, user_id: AnyUserId, new_user_id: AnyUserId) {
        trace!("connect_account");

        AccountTable::update_user_id(&self.pool, user_id, new_user_id)
            .await
            .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn add_bug_report(&self, bug_report: BugReport) {
        trace!("add_bug_report");

        BugMessageTable::insert(&self.pool, bug_report)
            .await
            .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn add_feature_vote(&self, feature: FeatureVote) {
        trace!("add_feature_vote");

        FeatureMessageTable::insert(&self.pool, feature)
            .await
            .unwrap();
    }

    #[instrument(skip(self))]
    pub async fn is_vote_ended(&self, descriptor: FeatureVoteDescriptor) -> bool {
        trace!("is_vote_ended");

        match FeatureMessageTable::find_by_descriptor(&self.pool, descriptor)
            .await
            .unwrap()
        {
            None => true,
            Some(vote) => vote.is_vote_ended,
        }
    }

    #[instrument(skip(self))]
    pub async fn get_feature_vote(&self, descriptor: FeatureVoteDescriptor) -> Option<FeatureVote> {
        trace!("get_feature_vote");

        FeatureMessageTable::find_by_descriptor(&self.pool, descriptor)
            .await
            .unwrap()
    }

    #[instrument(skip(self))]
    pub async fn end_feature_vote(&self, descriptor: FeatureVoteDescriptor) {
        trace!("end_feature_vote");

        FeatureMessageTable::end_vote(&self.pool, descriptor)
            .await
            .unwrap();
    }
}
