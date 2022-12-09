use app_macros::validate_api_secret;
use app_shared::models::Role;
use app_shared::{
    models::{Account, AnyUserId, RoleId, Secret, UserRights},
    prelude::*,
};

use crate::{Api, ApiError};

impl Api {
    #[instrument]
    pub async fn find_account_by_tfa_token_secret(&self, secret: Secret) -> Option<Account> {
        trace!("find_account_by_tfa_token_secret");

        let token = self.tokens_storage.find_by_secret(secret);

        let Some(token) = token else {
            return None;
        };

        self.find_account_by_id(AnyUserId::DiscordId(token.user.id))
            .await
    }

    #[instrument]
    pub async fn find_account_by_id(&self, user_id: AnyUserId) -> Option<Account> {
        trace!("find_account_by_id");

        self.database.find_account(user_id).await
    }

    #[instrument]
    pub async fn connect_byond_account_by_2fa(
        &self,
        api_secret: Secret,
        tfa_secret: Secret,
        ckey: ByondUserId,
    ) -> Result<(), ApiError> {
        trace!("connect_byond_account_by_2fa");

        let account = self.find_account_by_tfa_token_secret(tfa_secret).await;

        let Some(account) = account else {
            return Err(ApiError::Other("account not found".to_string()))
        };

        self.connect_byond_account(api_secret, AnyUserId::AccountId(account.id), ckey)
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
        trace!("connect_byond_account");

        let token = validate_api_secret!(api_secret);

        if !token
            .rights
            .user
            .contains(UserRights::ADD_CONNECTED_ACCOUNTS)
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
    pub async fn add_role_to_account(
        &self,
        api_secret: Secret,
        user_id: AnyUserId,
        role_id: RoleId,
    ) -> Result<(), ApiError> {
        trace!("add_role_to_account");

        let token = validate_api_secret!(api_secret);
        let role = self.database.find_role_by_id(role_id).await;

        if !token.rights.user.contains(UserRights::ADD_ROLES) {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        let Some(role) = role else {
            return Err(ApiError::Other("invalid role_id".to_string()))
        };

        if (!token.is_service && token.rights < role.rights)
            || (token.is_service && token.rights <= role.rights)
        {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        let account_roles = self.database.get_account_roles(user_id.clone()).await;

        if account_roles.iter().any(|role| role.id == role_id) {
            return Err(ApiError::Other("user already has this role".to_string()));
        }

        let account_rights = Role::sum_roles_rights(account_roles);

        if (!token.is_service && token.rights < account_rights)
            || (token.is_service && token.rights <= account_rights)
        {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        self.database.add_account_role(user_id, role_id).await;

        Ok(())
    }
}
