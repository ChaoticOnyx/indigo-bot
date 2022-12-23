use crate::api::private::PrivateApi;
use app_shared::{
    models::{Account, AnyUserId, ApiError, Rights, Role, RoleId, Secret},
    prelude::*,
};

impl PrivateApi {
    /// Находит аккаунт по принадлежащему ему TFA токену.
    #[instrument]
    pub fn find_account_by_tfa_token_secret(&self, secret: Secret) -> Option<Account> {
        trace!("find_account_by_tfa_token_secret");

        let token = self.find_tfa_token_by_secret(secret);

        let Some(token) = token else {
            return None;
        };

        self.find_account_by_id(AnyUserId::DiscordId(token.discord_user_id))
    }

    /// Находит аккаунт по одному из его ID.
    #[instrument]
    pub fn find_account_by_id(&self, user_id: AnyUserId) -> Option<Account> {
        trace!("find_account_by_id");

        self.database.find_account(user_id)
    }

    /// Создаёт связь между BYOND аккаунтом и внутренним аккаунтом.
    #[instrument]
    pub fn connect_byond_account(
        &self,
        user_id: AnyUserId,
        ckey: ByondUserId,
    ) -> Result<(), ApiError> {
        trace!("connect_byond_account");

        let Some(account) = self.find_account_by_id(user_id.clone()) else {
            return Err(ApiError::Other("account not found".to_string()))
        };

        if ckey.0.trim().is_empty() {
            return Err(ApiError::Other("ckey is empty".to_string()));
        }

        if account.byond_ckey.is_some() {
            warn!("byond account is already connected");

            return Err(ApiError::Other(
                "byond account is already connected".to_string(),
            ));
        }

        self.database
            .connect_account(user_id, AnyUserId::ByondCkey(ckey));

        Ok(())
    }

    /// Возвращает права аккаунта.
    #[instrument]
    pub fn get_account_rights(&self, user_id: AnyUserId, roles: Option<Vec<Role>>) -> Rights {
        trace!("get_account_rights");

        let account_roles = match roles {
            Some(roles) => roles,
            None => self.get_account_roles(user_id.clone()),
        };

        Role::sum_roles_rights(account_roles)
    }

    /// Возвращает список ролей аккаунта.
    #[instrument]
    pub fn get_account_roles(&self, user_id: AnyUserId) -> Vec<Role> {
        trace!("get_account_roles");

        self.database.get_account_roles(user_id)
    }

    /// Добавляет роль к аккаунту.
    #[instrument]
    pub fn add_role_to_account(&self, user_id: AnyUserId, role_id: RoleId) -> Result<(), ApiError> {
        trace!("add_role_to_account");

        let Some(account) = self.find_account_by_id(user_id) else {
            return Err(ApiError::Other("invalid user_id".to_string()))
        };

        let account_roles = self.get_account_roles(AnyUserId::AccountId(account.id));

        if account_roles.iter().any(|role| role.id == role_id) {
            return Err(ApiError::Other("user already has this role".to_string()));
        }

        self.database
            .add_account_role(AnyUserId::AccountId(account.id), role_id);

        Ok(())
    }
}
