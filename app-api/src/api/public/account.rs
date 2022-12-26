use crate::Api;
use app_macros::validate_api_secret;
use app_shared::models::{Account, AccountId};
use app_shared::{
    models::{AnyUserId, ApiError, ApiToken, RoleId, Secret, UserRights},
    prelude::*,
};

impl Api {
    #[instrument]
    pub fn connect_byond_account_by_2fa(
        &mut self,
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
            return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
        }

        let account = self
            .private_api
            .find_account_by_tfa_token_secret(tfa_secret)?;

        self.private_api
            .connect_byond_account(AnyUserId::AccountId(account.id), ckey)?;

        Ok(())
    }

    #[instrument]
    pub fn add_role_to_account(
        &self,
        api_secret: Secret,
        account_id: AccountId,
        role_id: RoleId,
    ) -> Result<(), ApiError> {
        trace!("add_role_to_account");

        let token = validate_api_secret!(api_secret);
        let role = self.private_api.database.find_role_by_id(role_id);

        if !token.rights.user.contains(UserRights::ADD_ROLES) {
            return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
        }

        let Some(role) = role else {
            return Err(ApiError::Other("Некорректный role_id".to_string()))
        };

        if (!token.is_service && token.rights < role.rights)
            || (token.is_service && token.rights <= role.rights)
        {
            return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
        }

        let account_rights = self.private_api.get_account_rights(account_id, None);

        if (!token.is_service && token.rights < account_rights)
            || (token.is_service && token.rights <= account_rights)
        {
            return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
        }

        self.private_api.add_role_to_account(account_id, role_id)?;

        Ok(())
    }

    #[instrument]
    pub fn find_account_by_session(&self, session_secret: Secret) -> Result<Account, ApiError> {
        let session = self
            .private_api
            .find_session_by_secret(session_secret)
            .ok_or(ApiError::Other("Некорректная сессия".to_string()))?;

        self.private_api
            .find_account_by_id(AnyUserId::AccountId(session.account_id))
    }
}
