use crate::Api;
use app_macros::validate_api_secret;
use app_shared::{
    models::{AnyUserId, ApiError, ApiToken, RoleId, Secret, UserRights},
    prelude::*,
};

impl Api {
    #[instrument]
    pub fn connect_byond_account_by_2fa(
        &self,
        api_secret: Secret,
        tfa_secret: Secret,
        ckey: ByondUserId,
    ) -> Result<(), ApiError> {
        trace!("connect_byond_account_by_2fa");

        let token: ApiToken = validate_api_secret!(api_secret);

        if !token
            .rights
            .user
            .contains(UserRights::ADD_CONNECTED_ACCOUNTS)
        {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        let account = self
            .private_api
            .find_account_by_tfa_token_secret(tfa_secret);

        let Some(account) = account else {
            return Err(ApiError::Other("account not found".to_string()))
        };

        self.private_api
            .connect_byond_account(AnyUserId::AccountId(account.id), ckey)?;

        Ok(())
    }

    #[instrument]
    pub fn add_role_to_account(
        &self,
        api_secret: Secret,
        user_id: AnyUserId,
        role_id: RoleId,
    ) -> Result<(), ApiError> {
        trace!("add_role_to_account");

        let token = validate_api_secret!(api_secret);
        let role = self.private_api.database.find_role_by_id(role_id);

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

        let account_rights = self.private_api.get_account_rights(user_id.clone(), None);

        if (!token.is_service && token.rights < account_rights)
            || (token.is_service && token.rights <= account_rights)
        {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        self.private_api.add_role_to_account(user_id, role_id)?;

        Ok(())
    }
}
