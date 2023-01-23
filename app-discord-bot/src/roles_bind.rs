use app_api::Api;
use app_macros::config;
use app_shared::{
    models::{Account, AnyUserId, ApiCaller, RoleId},
    prelude::*,
    serenity::{
        model::{id::RoleId as DiscordRoleId, prelude::*},
        prelude::*,
    },
    DiscordConfig,
};
use std::collections::HashMap;

#[config]
#[derive(Debug)]
struct RolesBindConfig {
    pub binds: HashMap<DiscordRoleId, Vec<RoleId>>,
}

impl RolesBindConfig {
    pub fn inverted(self) -> HashMap<RoleId, Vec<DiscordRoleId>> {
        let mut inverted_dependencies: HashMap<RoleId, Vec<DiscordRoleId>> = HashMap::new();

        for (discord_role, account_roles) in self.binds {
            for account_role in account_roles {
                inverted_dependencies
                    .entry(account_role)
                    .and_modify(|vec| vec.push(discord_role))
                    .or_insert_with(|| vec![discord_role]);
            }
        }

        inverted_dependencies
    }
}

#[instrument(skip(ctx))]
pub async fn ready(ctx: &Context, _ready: &Ready) {
    trace!("ready");

    let config = RolesBindConfig::get().unwrap();

    if config.binds.is_empty() {
        return;
    }

    let bot_cfg = DiscordConfig::get().unwrap();
    let guild_id: GuildId = bot_cfg.guild_id;
    let accounts = Api::lock_async(|api| api.get_accounts()).await.unwrap();
    let inverted = config.clone().inverted();

    for account in &accounts {
        update_account_roles(ctx, guild_id, account, &config, &inverted).await;
    }
}

#[instrument(skip(ctx))]
pub async fn guild_member_update(ctx: &Context, old_if_available: &Option<Member>, new: &Member) {
    trace!("guild_member_update");

    let config = RolesBindConfig::get().unwrap();

    if config.binds.is_empty() {
        return;
    }

    let discord_user_id = new.user.id;
    let Ok(account) = Api::lock_async(move |api| {
        api
            .find_account_by_id(AnyUserId::DiscordId(discord_user_id))
    })
    .await
    .unwrap() else {
		return;
	};

    let bot_cfg = DiscordConfig::get().unwrap();
    let guild_id: GuildId = bot_cfg.guild_id;

    let inverted = config.clone().inverted();
    update_account_roles(ctx, guild_id, &account, &config, &inverted).await;
}

async fn update_account_roles(
    ctx: &Context,
    guild_id: GuildId,
    account: &Account,
    config: &RolesBindConfig,
    inverted: &HashMap<RoleId, Vec<DiscordRoleId>>,
) {
    let Some(member) = ctx.cache.member(guild_id, account.integrations.discord_user_id) else {
		return;
	};

    for discord_role in &member.roles {
        let Some(roles_id) = config.binds.get(discord_role).cloned() else {
			continue;
		};

        for role_id in roles_id {
            if account.roles.iter().any(|role| role.id == role_id) {
                continue;
            }

            let account_id = account.id;
            Api::lock_async(move |api| {
                api.add_role_to_account(ApiCaller::System, account_id, role_id)
                    .unwrap();
            })
            .await
            .unwrap();
        }
    }

    for account_role in &account.roles {
        let Some(discord_roles) = inverted.get(&account_role.id) else {
			continue;
		};

        if discord_roles
            .iter()
            .all(|role_id| member.roles.contains(role_id))
        {
            continue;
        }

        let account_id = account.id;
        let role_id = account_role.id;
        Api::lock_async(move |api| {
            api.remove_role_from_account(ApiCaller::System, account_id, role_id)
                .unwrap();
        })
        .await
        .unwrap();
    }
}
