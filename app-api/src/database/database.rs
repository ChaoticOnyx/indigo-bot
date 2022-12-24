use crate::database::db_config::DbConfig;
use crate::database::tables::{
    AccountTable, BugMessageTable, FeatureMessageTable, RoleTable, SessionTable, TokenTable,
    WebhookTable,
};
use app_shared::{
    chrono::DateTime,
    chrono::Utc,
    models::Role,
    models::{
        Account, AnyUserId, ApiToken, BugReport, FeatureVote, FeatureVoteDescriptor, RoleId,
        Secret, Session, Webhook,
    },
    prelude::*,
    tokio::runtime::Runtime,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Debug)]
pub struct Database {
    pool: Pool<Postgres>,
    rt: Runtime,
}

impl Database {
    #[instrument]
    pub fn connect() -> Self {
        info!("setting up database");

        let rt = app_shared::tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let config = DbConfig::get().unwrap();

        let pool =
            rt.block_on(async { PgPoolOptions::new().connect(&config.connect).await.unwrap() });

        Self { pool, rt }
    }

    #[instrument(skip(self))]
    pub fn migrate(&self) {
        info!("migrating");

        // Add migrations here!
        self.migration_init(&self.pool);

        info!("migration done");
    }

    #[instrument(skip(pool))]
    fn migration_init(&self, pool: &Pool<Postgres>) {
        info!("migration: migration_init");

        self.rt.block_on(async {
            FeatureMessageTable::create(pool).await.unwrap();
            BugMessageTable::create(pool).await.unwrap();
            AccountTable::create(pool).await.unwrap();
            TokenTable::create(pool).await.unwrap();
            WebhookTable::create(pool).await.unwrap();
            RoleTable::create(pool).await.unwrap();
            SessionTable::create(pool).await.unwrap();
        })
    }

    #[instrument(skip(self))]
    pub fn add_webhook(&self, webhook: Webhook) {
        trace!("add_webhook");

        self.rt.block_on(async {
            WebhookTable::insert(&self.pool, webhook).await.unwrap();
        });
    }

    #[instrument(skip(self))]
    pub fn find_webhook_by_secret(&self, secret: Secret) -> Option<Webhook> {
        trace!("find_webhook_by_secret");

        self.rt.block_on(async {
            WebhookTable::find_by_secret(&self.pool, secret)
                .await
                .unwrap()
        })
    }

    #[instrument(skip(self))]
    pub fn delete_webhook_by_secret(&self, secret: Secret) {
        trace!("delete_webhook_by_secret");

        self.rt.block_on(async {
            WebhookTable::delete_by_secret(&self.pool, secret)
                .await
                .unwrap();
        })
    }

    #[instrument(skip(self))]
    pub fn create_root_token_if_does_not_exist(&self, token: ApiToken) {
        trace!("update_root_token");

        self.rt.block_on(async {
            let has_token = TokenTable::find_by_secret(&self.pool, token.secret.clone())
                .await
                .unwrap()
                .is_some();

            if !has_token {
                debug!("creating new root token");

                TokenTable::insert(&self.pool, token).await.unwrap();
            }
        });
    }

    #[instrument(skip(self))]
    pub fn add_api_token(&self, token: ApiToken) {
        trace!("add_api_token");

        self.rt.block_on(async {
            TokenTable::insert(&self.pool, token).await.unwrap();
        })
    }

    #[instrument(skip(self))]
    pub fn delete_api_token_by_secret(&self, secret: Secret) {
        trace!("remove_api_token");

        self.rt.block_on(async {
            TokenTable::delete_by_secret(&self.pool, secret)
                .await
                .unwrap();
        });
    }

    #[instrument(skip(self))]
    pub fn find_api_token_by_secret(&self, api_secret: Secret) -> Option<ApiToken> {
        trace!("find_api_token_by_secret");

        self.rt.block_on(async {
            TokenTable::find_by_secret(&self.pool, api_secret)
                .await
                .unwrap()
        })
    }

    #[instrument(skip(self))]
    pub fn find_account(&self, user_id: AnyUserId) -> Option<Account> {
        trace!("find_account");

        self.rt.block_on(async {
            AccountTable::find_by_user_id(&self.pool, user_id)
                .await
                .unwrap()
        })
    }

    #[instrument(skip(self))]
    pub fn add_account(
        &self,
        discord_user_id: DiscordUserId,
        created_at: DateTime<Utc>,
        roles: &[Role],
    ) {
        trace!("add_account");

        self.rt.block_on(async {
            AccountTable::insert(&self.pool, discord_user_id, created_at, roles)
                .await
                .unwrap();
        });
    }

    #[instrument(skip(self))]
    pub fn connect_account(&self, user_id: AnyUserId, new_user_id: AnyUserId) {
        trace!("connect_account");

        self.rt.block_on(async {
            AccountTable::update_user_id(&self.pool, user_id, new_user_id)
                .await
                .unwrap();
        });
    }

    #[instrument(skip(self))]
    pub fn add_bug_report(&self, bug_report: BugReport) {
        trace!("add_bug_report");

        self.rt.block_on(async {
            BugMessageTable::insert(&self.pool, bug_report)
                .await
                .unwrap();
        });
    }

    #[instrument(skip(self))]
    pub fn add_feature_vote(&self, feature: FeatureVote) {
        trace!("add_feature_vote");

        self.rt.block_on(async {
            FeatureMessageTable::insert(&self.pool, feature)
                .await
                .unwrap();
        });
    }

    #[instrument(skip(self))]
    pub fn is_vote_ended(&self, descriptor: FeatureVoteDescriptor) -> bool {
        trace!("is_vote_ended");

        self.rt.block_on(async {
            match FeatureMessageTable::find_by_descriptor(&self.pool, descriptor)
                .await
                .unwrap()
            {
                None => true,
                Some(vote) => vote.is_vote_ended,
            }
        })
    }

    #[instrument(skip(self))]
    pub fn get_feature_vote(&self, descriptor: FeatureVoteDescriptor) -> Option<FeatureVote> {
        trace!("get_feature_vote");

        self.rt.block_on(async {
            FeatureMessageTable::find_by_descriptor(&self.pool, descriptor)
                .await
                .unwrap()
        })
    }

    #[instrument(skip(self))]
    pub fn end_feature_vote(&self, descriptor: FeatureVoteDescriptor) {
        trace!("end_feature_vote");

        self.rt.block_on(async {
            FeatureMessageTable::end_vote(&self.pool, descriptor)
                .await
                .unwrap();
        });
    }

    #[instrument(skip(self))]
    pub fn add_account_role(&self, user_id: AnyUserId, role_id: RoleId) {
        trace!("add_account_role");

        self.rt.block_on(async {
            AccountTable::add_role(&self.pool, user_id, role_id)
                .await
                .unwrap();
        });
    }

    #[instrument(skip(self))]
    pub fn get_account_roles(&self, user_id: AnyUserId) -> Vec<Role> {
        trace!("get_user_roles");

        self.rt.block_on(async {
            let user = AccountTable::find_by_user_id(&self.pool, user_id)
                .await
                .unwrap()
                .unwrap();

            RoleTable::find_by_ids(&self.pool, &user.roles)
                .await
                .unwrap()
        })
    }

    #[instrument(skip(self))]
    pub fn find_role_by_id(&self, role_id: RoleId) -> Option<Role> {
        trace!("find_role_by_id");

        self.rt
            .block_on(async { RoleTable::find_by_id(&self.pool, role_id).await.unwrap() })
    }

    #[instrument(skip(self))]
    pub fn find_session_by_secret(&self, session_secret: Secret) -> Option<Session> {
        trace!("find_session_by_secret");

        self.rt.block_on(async {
            SessionTable::find_by_secret(&self.pool, session_secret)
                .await
                .unwrap()
        })
    }

    #[instrument(skip(self))]
    pub fn add_session(&self, session: Session) {
        trace!("add_session");

        self.rt.block_on(async {
            SessionTable::insert(&self.pool, session).await.unwrap();
        });
    }

    #[instrument(skip(self))]
    pub fn delete_expired_sessions(&self) {
        trace!("remove_expired_session");

        self.rt.block_on(async {
            let sessions = SessionTable::get_all(&self.pool).await.unwrap();

            for session in sessions {
                if session.is_expired() {
                    SessionTable::delete_by_secret(&self.pool, session.secret)
                        .await
                        .unwrap();

                    TokenTable::delete_by_secret(&self.pool, session.api_secret)
                        .await
                        .unwrap();
                }
            }
        })
    }

    #[instrument(skip(self))]
    pub fn delete_session_by_secret(&self, session_secret: Secret) {
        trace!("delete_session_by_secret");

        self.rt.block_on(async {
            SessionTable::delete_by_secret(&self.pool, session_secret)
                .await
                .unwrap();
        })
    }
}
