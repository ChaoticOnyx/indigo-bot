use crate::api::private::PrivateApi;
use app_shared::chrono::Utc;
use app_shared::models::{AccountId, AccountIntegrations};
use app_shared::{
    models::{Account, AnyUserId, ApiError, Rights, Role, RoleId, Secret},
    prelude::*,
};

impl PrivateApi {
    pub fn create_account(
        &self,
        username: String,
        avatar_url: String,
        discord_user_id: DiscordUserId,
    ) -> Result<AccountId, ApiError> {
        let mut new_username = username.clone();
        let mut counter = 1;

        loop {
            if self.database.is_username_free(new_username.clone()) {
                break;
            } else {
                new_username = format!("{username}{counter}");
                counter += 1;
            }
        }

        Ok(self
            .database
            .add_account(new_username, avatar_url, Utc::now(), &[], discord_user_id))
    }

    /// Находит аккаунт по принадлежащему ему TFA токену.
    #[instrument]
    pub fn find_account_by_tfa_token_secret(
        &mut self,
        secret: Secret,
    ) -> Result<Account, ApiError> {
        trace!("find_account_by_tfa_token_secret");

        let token = self
            .find_tfa_token_by_secret(secret)
            .ok_or(ApiError::Other("Некорректный TFA токен".to_string()))?;

        self.find_account_by_id(AnyUserId::DiscordId(token.discord_user_id))
    }

    /// Находит аккаунт по одному из его ID.
    #[instrument]
    pub fn find_account_by_id(&self, user_id: AnyUserId) -> Result<Account, ApiError> {
        trace!("find_account_by_id");

        self.database
            .find_account(user_id)
            .ok_or(ApiError::Other("Неверный user_id".to_string()))
    }

    /// Возвращает интеграции аккаунтов.
    #[instrument]
    pub fn find_integrations_by_account_id(
        &self,
        user_id: AnyUserId,
    ) -> Result<AccountIntegrations, ApiError> {
        trace!("find_integrations_by_account_id");

        self.database
            .find_account_integrations_by_user_id(user_id)
            .ok_or(ApiError::Other("Аккаунт не существует".to_string()))
    }

    /// Создаёт связь между BYOND аккаунтом и внутренним аккаунтом.
    #[instrument]
    pub fn connect_byond_account(
        &self,
        user_id: AnyUserId,
        ckey: ByondUserId,
    ) -> Result<(), ApiError> {
        trace!("connect_byond_account");

        if ckey.0.trim().is_empty() {
            return Err(ApiError::Other("Пустой ckey".to_string()));
        }

        let integrations = self.find_integrations_by_account_id(user_id.clone())?;

        if integrations.byond_ckey.is_some() {
            return Err(ApiError::Other("Аккаунт BYOND уже подключен".to_string()));
        }

        self.database
            .connect_account(user_id, AnyUserId::ByondCkey(ckey));

        Ok(())
    }

    /// Возвращает права аккаунта.
    #[instrument]
    pub fn get_account_rights(&self, account_id: AccountId, roles: Option<Vec<Role>>) -> Rights {
        trace!("get_account_rights");

        let account_roles = match roles {
            Some(roles) => roles,
            None => self.get_account_roles(account_id.clone()),
        };

        Role::sum_roles_rights(account_roles)
    }

    /// Возвращает список ролей аккаунта.
    #[instrument]
    pub fn get_account_roles(&self, account_id: AccountId) -> Vec<Role> {
        trace!("get_account_roles");

        self.database.get_account_roles(account_id)
    }

    /// Добавляет роль к аккаунту.
    #[instrument]
    pub fn add_role_to_account(
        &self,
        account_id: AccountId,
        role_id: RoleId,
    ) -> Result<(), ApiError> {
        trace!("add_role_to_account");

        let account_roles = self.get_account_roles(account_id);

        if account_roles.iter().any(|role| role.id == role_id) {
            return Err(ApiError::Other(
                "Пользователь уже имеет эту роль".to_string(),
            ));
        }

        self.database.add_account_role(account_id, role_id);

        Ok(())
    }
}
