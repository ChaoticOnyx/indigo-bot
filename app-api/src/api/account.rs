use crate::{Api, Journal};
use app_macros::validate_api_secret;
use app_shared::{
    chrono::Utc,
    models::ApiCaller,
    models::{
        Account, AccountId, AccountIntegrations, ActionType, Actor, AnyUserId, ApiError,
        DonationTier, Rights, Role, RoleId, Secret, UserRights,
    },
    prelude::*,
    Database,
};

impl Api {
    /// Создаёт новый аккаунт.
    pub fn create_account(
        &self,
        caller: ApiCaller,
        username: String,
        avatar_url: String,
        discord_user_id: DiscordUserId,
    ) -> Result<AccountId, ApiError> {
        let mut new_username = username.clone();
        let mut counter = 1;
        let mut actor = Actor::System;

        if let ApiCaller::Token(secret) = caller {
            let token = validate_api_secret!(secret);

            if !token.rights.user.contains(UserRights::CREATE_ACCOUNTS) {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            }

            actor = if let Some(account_id) = token.creator {
                Actor::User(account_id)
            } else {
                Actor::System
            };
        }

        loop {
            if Database::lock(|database| database.is_username_free(new_username.clone())) {
                break;
            } else {
                new_username = format!("{username}{counter}");
                counter += 1;
            }
        }

        let account_id = Database::lock(|database| {
            database.add_account(new_username, avatar_url, Utc::now(), &[], discord_user_id)
        });

        Journal::lock(|journal| {
            journal.log(
                actor,
                Some(Actor::User(account_id)),
                ActionType::AccountCreated,
            )
        });

        Ok(account_id)
    }

    #[instrument]
    pub fn get_accounts(&self) -> Vec<Account> {
        trace!("get_accounts");

        Database::lock(|database| database.get_accounts())
    }

    #[instrument]
    pub fn get_donation_tiers(&self) -> Vec<DonationTier> {
        trace!("get_donation_tiers");

        Database::lock(|database| database.get_donation_tiers())
    }

    /// Подключает BYOND аккаунт с помощью 2FA.
    #[instrument]
    pub fn connect_byond_account_by_2fa(
        &mut self,
        caller: ApiCaller,
        tfa_secret: Secret,
        ckey: ByondUserId,
    ) -> Result<(), ApiError> {
        trace!("connect_byond_account_by_2fa");

        let mut actor = Actor::System;

        if let ApiCaller::Token(secret) = caller {
            let token = validate_api_secret!(secret);

            if !token
                .rights
                .user
                .contains(UserRights::ADD_CONNECTED_ACCOUNTS)
            {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            }

            actor = if let Some(account_id) = token.creator {
                Actor::User(account_id)
            } else {
                Actor::System
            };
        }

        let account = self.find_account_by_tfa_token_secret(tfa_secret)?;
        self.connect_byond_account(AnyUserId::AccountId(account.id), ckey.clone())?;

        Journal::lock(|journal| {
            journal.log(
                actor,
                Some(Actor::User(account.id)),
                ActionType::ByondConnected { ckey },
            )
        });

        Ok(())
    }

    /// Подключает SS14 аккаунт с помощью 2FA.
    #[instrument]
    pub fn connect_ss14_account_by_2fa(
        &mut self,
        caller: ApiCaller,
        tfa_secret: Secret,
        user_id: SS14UserId,
    ) -> Result<(), ApiError> {
        trace!("connect_ss14_account_by_2fa");

        let mut actor = Actor::System;

        if let ApiCaller::Token(secret) = caller {
            let token = validate_api_secret!(secret);

            if !token
                .rights
                .user
                .contains(UserRights::ADD_CONNECTED_ACCOUNTS)
            {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            }

            actor = if let Some(account_id) = token.creator {
                Actor::User(account_id)
            } else {
                Actor::System
            };
        }

        let account = self.find_account_by_tfa_token_secret(tfa_secret)?;
        self.connect_ss14_account(AnyUserId::AccountId(account.id), user_id.clone())?;

        Journal::lock(|journal| {
            journal.log(
                actor,
                Some(Actor::User(account.id)),
                ActionType::SS14Connected { ss14_guid: user_id },
            )
        });

        Ok(())
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

        Database::lock(|database| {
            database
                .find_account(user_id)
                .ok_or_else(|| ApiError::Other("Неверный user_id".to_string()))
        })
    }

    /// Возвращает интеграции аккаунтов.
    #[instrument]
    pub fn find_integrations_by_account_id(
        &self,
        user_id: AnyUserId,
    ) -> Result<AccountIntegrations, ApiError> {
        trace!("find_integrations_by_account_id");

        Database::lock(|database| {
            database
                .find_account_integrations_by_user_id(user_id)
                .ok_or_else(|| ApiError::Other("Аккаунт не существует".to_string()))
        })
    }

    /// Возвращает все аккаунты с указанной ролью.
    #[instrument]
    pub fn find_accounts_with_role(&self, role_id: RoleId) -> Vec<Account> {
        trace!("find_accounts_with_role");

        Database::lock(|database| database.find_accounts_with_role(role_id))
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

        Database::lock(|database| database.connect_account(user_id, AnyUserId::ByondCkey(ckey)));

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

        Database::lock(|database| {
            database.connect_account(user_id, AnyUserId::SS14Guid(ss14_user_id))
        });

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

        Database::lock(|database| database.get_account_roles(account_id))
    }

    /// Добавляет роль к аккаунту.
    #[instrument]
    pub fn add_role_to_account(
        &self,
        caller: ApiCaller,
        account_id: AccountId,
        role_id: RoleId,
    ) -> Result<(), ApiError> {
        trace!("add_role_to_account");

        let mut actor = Actor::System;
        let role = Database::lock(|database| database.find_role_by_id(role_id));

        if let ApiCaller::Token(secret) = caller {
            let token = validate_api_secret!(secret);

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

            let account_rights = self.get_account_rights(account_id, None);

            if (!token.is_service && token.rights < account_rights)
                || (token.is_service && token.rights <= account_rights)
            {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            }

            actor = if let Some(account_id) = token.creator {
                Actor::User(account_id)
            } else {
                Actor::System
            };
        }

        let account_roles = self.get_account_roles(account_id);

        if account_roles.iter().any(|role| role.id == role_id) {
            return Err(ApiError::Other(
                "Пользователь уже имеет эту роль".to_string(),
            ));
        }

        Database::lock(|database| database.add_account_role(account_id, role_id));

        Journal::lock(|journal| {
            journal.log(
                actor,
                Some(Actor::User(account_id)),
                ActionType::RoleAdded { role_id },
            )
        });

        Ok(())
    }

    /// Удаляет роль с аккаунта.
    #[instrument]
    pub fn remove_role_from_account(
        &self,
        caller: ApiCaller,
        account_id: AccountId,
        role_id: RoleId,
    ) -> Result<(), ApiError> {
        trace!("remove_role_from_account");

        let mut actor = Actor::System;
        let role = Database::lock(|database| database.find_role_by_id(role_id));

        if let ApiCaller::Token(secret) = caller {
            let token = validate_api_secret!(secret);

            if !token.rights.user.contains(UserRights::REMOVE_ROLES) {
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

            let account_rights = self.get_account_rights(account_id, None);

            if (!token.is_service && token.rights < account_rights)
                || (token.is_service && token.rights <= account_rights)
            {
                return Err(ApiError::Forbidden("Недостаточно доступа".to_string()));
            }

            actor = if let Some(account_id) = token.creator {
                Actor::User(account_id)
            } else {
                Actor::System
            };
        }

        let account_roles = self.get_account_roles(account_id);

        if !account_roles.iter().any(|role| role.id == role_id) {
            return Err(ApiError::Other(
                "Пользователь не имеет этой роли".to_string(),
            ));
        }

        Database::lock(|database| database.remove_account_role(account_id, role_id));

        Journal::lock(|journal| {
            journal.log(
                actor,
                Some(Actor::User(account_id)),
                ActionType::RoleRemoved { role_id },
            )
        });

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

        if !Database::lock(|database| database.is_username_free(new_username.clone())) {
            return Err(ApiError::Other("Имя пользователя занято".to_string()));
        }

        let account = self.find_account_by_id(user_id)?;
        Database::lock(|database| database.change_username(account.id, new_username));

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
        Database::lock(|database| database.change_avatar_url(account.id, new_avatar_url));

        Ok(())
    }

    /// Находит аккаунт по секрету сессии.
    #[instrument]
    pub fn find_account_by_session(&self, session_secret: Secret) -> Result<Account, ApiError> {
        trace!("find_account_by_session");

        let session = self
            .find_session_by_secret(session_secret)
            .ok_or_else(|| ApiError::Other("Некорректная сессия".to_string()))?;

        self.find_account_by_id(AnyUserId::AccountId(session.account_id))
    }
}
