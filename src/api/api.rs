use crate::api::models::{
    Account, AnyUserId, ApiToken, NewAccount, Rights, TokenRightsFlags, UserRightsFlags,
};
use chrono::{Duration, Utc};
use octocrab::models::IssueId;
use serde::Serialize;

use super::{
    github::Github,
    models::{BugReport, FeatureVote, FeatureVoteDescriptor, TFAToken, TokenSecret},
    tfa_tokens_storage::TFATokensStorage,
    Database,
};
use crate::prelude::*;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ApiError {
    Unauthorized(String),
    Forbidden(String),
    Other(String),
}

#[derive(Debug)]
pub struct Api {
    database: Database,
    github: Github,
    tokens_storage: TFATokensStorage,
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

        Self {
            database,
            github,
            tokens_storage,
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
    pub async fn get_or_create_tfa_token(&mut self, user: discord::user::User) -> TFAToken {
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
    pub async fn find_tfa_token_by_secret(&self, secret: TokenSecret) -> Option<TFAToken> {
        debug!("find_tfa_token_by_secret");

        self.tokens_storage.find_by_secret(secret).cloned()
    }

    #[instrument]
    pub async fn find_account_by_tfa_token_secret(&self, secret: TokenSecret) -> Option<Account> {
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
        api_secret: TokenSecret,
        tfa_secret: TokenSecret,
        ckey: byond::UserId,
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
        api_secret: TokenSecret,
        user_id: AnyUserId,
        ckey: byond::UserId,
    ) -> Result<(), ApiError> {
        info!("connect_byond_account");

        let token = self.database.find_api_token_by_secret(api_secret).await;
        let Some(token) = token else {
            return Err(ApiError::Unauthorized("invalid api secret".to_string()));
        };

        if token.is_expired() {
            return Err(ApiError::Unauthorized("invalid api secret".to_string()));
        }

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
        api_secret: TokenSecret,
        rights: Rights,
        duration: Option<Duration>,
    ) -> Result<ApiToken, ApiError> {
        info!("create_api_token");

        let token = self.database.find_api_token_by_secret(api_secret).await;

        let Some(token) = token else {
            return Err(ApiError::Unauthorized("invalid secret".to_string()))
        };

        if token.is_expired() {
            return Err(ApiError::Unauthorized("invalid api secret".to_string()));
        }

        let new_secret = loop {
            let secret = TokenSecret::new_random_api_secret();

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

        let our_rights = &token.rights;
        let new_rights = &new_token.rights;
        let has_equal_rights_or_lesser = (our_rights.token.flags | new_rights.token.flags)
            == our_rights.token.flags
            && (our_rights.user.flags | new_rights.user.flags) == our_rights.user.flags
            && (our_rights.server.flags | new_rights.server.flags) == our_rights.server.flags;

        if !has_equal_rights_or_lesser {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        self.database.add_api_token(new_token.clone()).await;

        Ok(new_token)
    }
}
