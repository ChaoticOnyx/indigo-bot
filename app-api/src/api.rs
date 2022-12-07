use std::fmt::{Display, Formatter};

use app_macros::validate_api_secret;
use app_shared::{
    chrono::{Duration, Utc},
    models::{
        Account, AnyUserId, ApiToken, BugReport, FeatureVote, FeatureVoteDescriptor, NewAccount,
        Rights, Secret, ServiceError, ServiceId, TFAToken, TokenRightsFlags, UserRightsFlags,
        Webhook, WebhookConfiguration, WebhookPayload, WebhookResponse,
    },
    octocrab::models::IssueId,
    prelude::*,
    serde::Serialize,
    state::Settings,
};

use crate::{Database, Github, ServicesStorage, TFATokensStorage};

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ApiError {
    Unauthorized(String),
    Forbidden(String),
    Other(String),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Unauthorized(msg) => f.write_str(msg),
            ApiError::Forbidden(msg) => f.write_str(msg),
            ApiError::Other(msg) => f.write_str(msg),
        }
    }
}

#[derive(Debug)]
pub struct Api {
    database: Database,
    github: Github,
    tokens_storage: TFATokensStorage,
    services_storage: ServicesStorage,
}

impl Api {
    #[instrument]
    pub async fn new(settings: &Settings) -> Self {
        info!("creating api");

        // GitHub
        let github = Github::new(settings.github.token.clone());

        // Database
        let database = Database::connect(&settings.database.connect).await;
        database.migrate().await;

        // Tokens storage
        let tokens_storage = TFATokensStorage::default();

        let root_token = ApiToken::new(
            Settings::clone_state().await.api.root_secret,
            Rights::full(),
            None,
        );
        database.update_root_token(root_token).await;

        let mut services_storage = ServicesStorage::new();
        services_storage.register();

        Self {
            database,
            github,
            tokens_storage,
            services_storage,
        }
    }

    #[instrument]
    pub async fn new_feature_vote(&self, vote: FeatureVote) {
        debug!("new_feature_vote api");

        self.database.add_feature_vote(vote).await;
    }

    #[instrument]
    pub async fn end_feature_vote(&self, descriptor: FeatureVoteDescriptor) {
        debug!("end_feature_vote api");

        self.database.end_feature_vote(descriptor).await;
    }

    #[instrument]
    pub async fn is_vote_ended(&self, descriptor: FeatureVoteDescriptor) -> bool {
        debug!("is_vote_ended api");

        self.database.is_vote_ended(descriptor).await
    }

    #[instrument]
    pub async fn get_feature_vote(&self, descriptor: FeatureVoteDescriptor) -> Option<FeatureVote> {
        debug!("get_feature_vote api");

        self.database.get_feature_vote(descriptor).await
    }

    #[instrument]
    pub async fn add_bug_report(&self, bug_report: BugReport) {
        debug!("add_bug_report");

        self.database.add_bug_report(bug_report).await;
    }

    #[instrument]
    pub async fn create_feature_issue(&self, title: String, description: String) -> IssueId {
        info!("create_feature_issue");

        let settings = Settings::clone_state().await;

        self.github
            .create_issue(
                settings.commands.feedback.features_repository,
                title,
                description,
                settings.commands.feedback.feature_issue_labels,
            )
            .await
    }

    #[instrument]
    pub async fn create_bug_issue(&self, title: String, description: String) -> IssueId {
        info!("create_bug_issue");

        let settings = Settings::clone_state().await;

        self.github
            .create_issue(
                settings.commands.feedback.bugs_repository,
                title,
                description,
                settings.commands.feedback.bug_issue_labels,
            )
            .await
    }

    #[instrument]
    pub async fn get_or_create_tfa_token(&mut self, user: DiscordUser) -> TFAToken {
        debug!("get_or_create_tfa_token");

        if self
            .database
            .find_account(AnyUserId::DiscordId(user.id))
            .await
            .is_none()
        {
            self.database
                .add_account(NewAccount {
                    created_at: Utc::now(),
                    discord_id: user.id,
                })
                .await;
        }

        self.tokens_storage.remove_expired_tokens();

        match self.tokens_storage.find_by_user_id(user.id) {
            None => {
                debug!("existing token not found");

                self.tokens_storage.new_token(user, Duration::seconds(60))
            }
            Some(token) => {
                debug!("existing token found");
                token.clone()
            }
        }
    }

    #[instrument]
    pub async fn find_tfa_token_by_secret(&self, secret: Secret) -> Option<TFAToken> {
        debug!("find_tfa_token_by_secret");

        self.tokens_storage.find_by_secret(secret).cloned()
    }

    #[instrument]
    pub async fn find_account_by_tfa_token_secret(&self, secret: Secret) -> Option<Account> {
        debug!("find_account_by_tfa_token_secret");

        let token = self.tokens_storage.find_by_secret(secret);

        let Some(token) = token else {
            return None;
        };

        self.find_account_by_id(AnyUserId::DiscordId(token.user.id))
            .await
    }

    #[instrument]
    pub async fn find_account_by_id(&self, user_id: AnyUserId) -> Option<Account> {
        debug!("find_account_by_id");

        self.database.find_account(user_id).await
    }

    #[instrument]
    pub async fn connect_byond_account_by_2fa(
        &self,
        api_secret: Secret,
        tfa_secret: Secret,
        ckey: ByondUserId,
    ) -> Result<(), ApiError> {
        info!("connect_byond_account_by_2fa");

        let account = self.find_account_by_tfa_token_secret(tfa_secret).await;

        let Some(account) = account else {
            return Err(ApiError::Other("account not found".to_string()))
        };

        self.connect_byond_account(api_secret, AnyUserId::InternalId(account.id), ckey)
            .await?;

        Ok(())
    }

    #[instrument]
    pub async fn connect_byond_account(
        &self,
        api_secret: Secret,
        user_id: AnyUserId,
        ckey: ByondUserId,
    ) -> Result<(), ApiError> {
        info!("connect_byond_account");

        let token = validate_api_secret!(api_secret);

        if !token
            .rights
            .user
            .flags
            .contains(UserRightsFlags::CONNECT_WRITE)
        {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        if ckey.0.trim().is_empty() {
            return Err(ApiError::Other("ckey is empty".to_string()));
        }

        let account = self.find_account_by_id(user_id.clone()).await;

        let Some(account) = account else {
            warn!("account not found");
            return Err(ApiError::Other("account not found".to_string()));
        };

        if account.byond_ckey.is_some() {
            warn!("byond account is already connected");
            return Err(ApiError::Other(
                "byond account is already connected".to_string(),
            ));
        }

        self.database
            .connect_account(user_id, AnyUserId::ByondCkey(ckey))
            .await;

        Ok(())
    }

    #[instrument]
    pub async fn create_api_token(
        &self,
        api_secret: Secret,
        rights: Rights,
        duration: Option<Duration>,
    ) -> Result<ApiToken, ApiError> {
        info!("create_api_token");

        let token = validate_api_secret!(api_secret);

        let new_secret = loop {
            let secret = Secret::new_random_api_secret();

            if self
                .database
                .find_api_token_by_secret(secret.clone())
                .await
                .is_none()
            {
                break secret;
            }
        };

        let new_token = ApiToken::new(new_secret, rights, duration);

        if new_token.is_expired() {
            return Err(ApiError::Other("new token is already expired".to_string()));
        }

        if !token
            .rights
            .token
            .flags
            .contains(TokenRightsFlags::TOKEN_CREATE)
        {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        if !token
            .rights
            .has_more_or_equal_rights_than(&new_token.rights)
        {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        self.database.add_api_token(new_token.clone()).await;

        Ok(new_token)
    }

    #[instrument]
    pub async fn delete_api_token(
        &self,
        api_secret: Secret,
        target: Secret,
    ) -> Result<(), ApiError> {
        info!("delete_api_token");

        let token = validate_api_secret!(api_secret);
        let target_token = self.database.find_api_token_by_secret(target).await;

        let Some(target_token) = target_token else {
            return Err(ApiError::Other("target token does not exist".to_string()))
        };

        if !token
            .rights
            .has_more_or_equal_rights_than(&target_token.rights)
        {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        };

        self.database
            .delete_api_token_by_secret(target_token.secret)
            .await;

        Ok(())
    }

    #[instrument]
    pub async fn create_webhook(
        &self,
        api_secret: Secret,
        target: ServiceId,
        name: String,
        configuration: WebhookConfiguration,
    ) -> Result<Webhook, ApiError> {
        info!("create_webhook");

        let token = validate_api_secret!(api_secret);

        if !token.rights.service.can_create_tokens_for_service(&target) {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        if !self.services_storage.is_service_exists(&target) {
            return Err(ApiError::Other("invalid service".to_string()));
        }

        if name.trim().is_empty() {
            return Err(ApiError::Other("webhook name is empty".to_string()));
        }

        match self
            .services_storage
            .configure_webhook(self, &target, &configuration)
            .await
        {
            Ok(_) => (),
            Err(err) => return Err(ApiError::Other(format!("invalid configuration: {err}"))),
        }

        let secret = Secret::new_random_webhook_secret();
        let webhook = Webhook {
            name,
            secret,
            service_id: target,
            created_at: Utc::now(),
            configuration,
        };

        self.database.add_webhook(webhook.clone()).await;

        Ok(webhook)
    }

    #[instrument]
    pub async fn delete_webhook(
        &self,
        api_secret: Secret,
        webhook_secret: Secret,
    ) -> Result<(), ApiError> {
        info!("delete_webhook");

        let token = validate_api_secret!(api_secret);
        let webhook = self.database.find_webhook_by_secret(webhook_secret).await;

        let Some(webhook) = webhook else {
            return Err(ApiError::Other("invalid webhook secret".to_string()))
        };

        if !token
            .rights
            .service
            .can_delete_tokens_for_service(&webhook.service_id)
        {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        self.database.delete_webhook_by_secret(webhook.secret).await;

        Ok(())
    }

    #[instrument]
    pub async fn handle_webhook(
        &self,
        webhook_secret: Secret,
        payload: WebhookPayload,
    ) -> Result<WebhookResponse, ServiceError> {
        let webhook = self
            .database
            .find_webhook_by_secret(webhook_secret.clone())
            .await;

        let Some(webhook) = webhook else {
            return Err(ServiceError::Any("invalid webhook".to_string()))
        };

        self.services_storage
            .handle(self, &webhook.service_id, &webhook.configuration, &payload)
            .await
    }
}
