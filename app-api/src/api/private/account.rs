use crate::api::private::PrivateApi;
use app_shared::chrono::Utc;
use app_shared::models::{AccountId, AccountIntegrations};
use app_shared::{
    models::{Account, AnyUserId, ApiError, Rights, Role, RoleId, Secret},
    prelude::*,
};

impl PrivateApi {
    /// Создаёт новый аккаунт.
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
            .ok_or_else(|| ApiError::Other("Некорректный TFA токен".to_string()))?;

        self.find_account_by_id(AnyUserId::DiscordId(token.discord_user_id))
    }

    /// Находит аккаунт по одному из его ID.
    #[instrument]
    pub fn find_account_by_id(&self, user_id: AnyUserId) -> Result<Account, ApiError> {
        trace!("find_account_by_id");

        self.database
            .find_account(user_id)
            .ok_or_else(|| ApiError::Other("Неверный user_id".to_string()))
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
            .ok_or_else(|| ApiError::Other("Аккаунт не существует".to_string()))
    }

    /// Возвращает все аккаунты с указанной ролью.
    #[instrument]
    pub fn find_accounts_with_role(&self, role_id: RoleId) -> Vec<Account> {
        trace!("find_accounts_with_role");

        self.database.find_accounts_with_role(role_id)
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

    #[instrument]
    pub fn connect_ss14_account(
        &self,
        user_id: AnyUserId,
        ss14_user_id: SS14UserId,
    ) -> Result<(), ApiError> {
        trace!("connect_ss14_account");

        if ss14_user_id.0.trim().is_empty() {
            return Err(ApiError::Other("Пустой ss14_user_id".to_string()));
        }

        let integrations = self.find_integrations_by_account_id(user_id.clone())?;

        if integrations.ss14_guid.is_some() {
            return Err(ApiError::Other("Аккаунт SS14 уже подключен".to_string()));
        }

        self.database
            .connect_account(user_id, AnyUserId::SS14Guid(ss14_user_id));

        Ok(())
    }

    /// Возвращает права аккаунта.
    #[instrument]
    pub fn get_account_rights(&self, account_id: AccountId, roles: Option<Vec<Role>>) -> Rights {
        trace!("get_account_rights");

        let account_roles = match roles {
            Some(roles) => roles,
            None => self.get_account_roles(account_id),
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

    #[instrument]
    pub fn remove_role_from_account(
        &self,
        account_id: AccountId,
        role_id: RoleId,
    ) -> Result<(), ApiError> {
        trace!("remove_role_from_account");

        let account_roles = self.get_account_roles(account_id);

        if !account_roles.iter().any(|role| role.id == role_id) {
            return Err(ApiError::Other(
                "Пользователь не имеет этой роли".to_string(),
            ));
        }

        self.database.remove_account_role(account_id, role_id);

        Ok(())
    }

    /// Меняет имя пользователя (если оно не занято).
    #[instrument]
    pub fn change_username(
        &self,
        user_id: AnyUserId,
        mut new_username: String,
    ) -> Result<(), ApiError> {
        trace!("change_username");

        new_username = new_username.trim().to_string();

        if new_username.is_empty() {
            return Err(ApiError::Other(
                "Имя пользователя не должно быть пустым".to_string(),
            ));
        }

        if new_username.chars().count() > 25 {
            return Err(ApiError::Other(
                "Имя пользователя не должно быть длинее 25 символов!".to_string(),
            ));
        }

        if !self.database.is_username_free(new_username.clone()) {
            return Err(ApiError::Other("Имя пользователя занято".to_string()));
        }

        let account = self.find_account_by_id(user_id)?;
        self.database.change_username(account.id, new_username);

        Ok(())
    }

    /// Меняет аватарку пользователя.
    #[instrument]
    pub fn change_avatar_url(
        &self,
        user_id: AnyUserId,
        mut new_avatar_url: String,
    ) -> Result<(), ApiError> {
        trace!("change_avatar_url");

        if new_avatar_url.chars().count() > 200 {
            return Err(ApiError::Other(
                "Ссылка не должна быть длинее 200 символов!".to_string(),
            ));
        }

        if new_avatar_url.trim().is_empty() {
            new_avatar_url = String::from("/public/images/avatar.png");
        }

        let account = self.find_account_by_id(user_id)?;
        self.database.change_avatar_url(account.id, new_avatar_url);

        Ok(())
    }
}
