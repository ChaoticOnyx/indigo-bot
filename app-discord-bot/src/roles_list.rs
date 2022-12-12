use crate::discord_config::DiscordConfig;
use app_macros::config;
use app_shared::{
    futures_util::StreamExt,
    prelude::*,
    serenity::model::prelude::{ChannelId, GuildId, Member, MessageId, Ready, RoleId, User},
    serenity::prelude::{Context, Mentionable},
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RolesList {
    pub id: RoleId,
    pub title: String,
    pub pin: bool,
    pub message_id: Option<MessageId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RolesListChannel {
    pub id: ChannelId,
    pub roles: Vec<RolesList>,
}

#[config]
#[derive(Debug)]
struct RolesListConfig {
    pub channels: Vec<RolesListChannel>,
}

#[instrument(skip(ctx))]
pub async fn ready(ctx: &Context, ready: &Ready) {
    info!("ready");

    let mut config = RolesListConfig::get().await.unwrap();
    let bot_cfg = DiscordConfig::get().await.unwrap();
    let guild_id: GuildId = bot_cfg.guild_id;
    let mut cached: BTreeMap<RoleId, Vec<User>> = BTreeMap::new();

    for channel in &mut config.channels {
        for role in &mut channel.roles {
            let users = cached
                .entry(role.id)
                .or_insert(get_users_with_role(ctx, guild_id, role.id).await);

            role.message_id = match role.message_id {
                None => Some(create_message(ctx, channel.id, users, role).await),
                Some(_) => Some(update_message(ctx, channel.id, users, role).await),
            }
        }
    }

    RolesListConfig::save(config).await;
}

#[instrument(skip(ctx))]
pub async fn guild_member_update(ctx: &Context, old_if_available: &Option<Member>, new: &Member) {
    trace!("guild_member_update");

    let mut roles_to_update: BTreeSet<RoleId> = BTreeSet::new();

    if let Some(old) = old_if_available {
        let mut set = old.roles.iter().cloned().collect();
        roles_to_update.append(&mut set);
    }

    roles_to_update.append(&mut new.roles.iter().cloned().collect());

    let mut config = RolesListConfig::get().await.unwrap();
    let bot_cfg = DiscordConfig::get().await.unwrap();
    let guild_id: GuildId = bot_cfg.guild_id;
    let mut cached: BTreeMap<RoleId, Vec<User>> = BTreeMap::new();

    for channel in &mut config.channels {
        for role in &mut channel.roles {
            if !roles_to_update.contains(&role.id) {
                continue;
            }

            let users = cached
                .entry(role.id)
                .or_insert(get_users_with_role(ctx, guild_id, role.id).await);

            role.message_id = match role.message_id {
                None => Some(create_message(ctx, channel.id, users, role).await),
                Some(_) => Some(update_message(ctx, channel.id, users, role).await),
            }
        }
    }

    RolesListConfig::save(config).await;
}

#[instrument(skip(ctx))]
async fn get_users_with_role(ctx: &Context, guild_id: GuildId, role_id: RoleId) -> Vec<User> {
    trace!("get_users_with_role");

    let mut users_with_role = Vec::new();
    let mut members = guild_id.members_iter(ctx).boxed();

    while let Some(member) = members.next().await {
        let member = match member {
            Err(err) => {
                error!("{err}");
                continue;
            }
            Ok(member) => member,
        };

        if member.roles.iter().any(|role| *role == role_id) {
            users_with_role.push(member.user)
        }
    }

    users_with_role
}

async fn update_message(
    ctx: &Context,
    channel_id: ChannelId,
    users: &[User],
    cfg: &RolesList,
) -> MessageId {
    let message = channel_id.message(ctx, cfg.message_id.unwrap()).await.ok();
    let mut message = match message {
        None => return create_message(ctx, channel_id, users, cfg).await,
        Some(message) => message,
    };

    let users_string: String = users
        .iter()
        .map(|user| user.mention().to_string())
        .join("\n");

    let message_content = format!("{}\n{}", cfg.title, users_string);

    message
        .edit(ctx, |message| message.content(message_content))
        .await
        .unwrap();

    message.id
}

async fn create_message(
    ctx: &Context,
    channel_id: ChannelId,
    users: &[User],
    cfg: &RolesList,
) -> MessageId {
    let mut message = channel_id
        .send_message(ctx, |message| message.content("-"))
        .await
        .unwrap();

    let users_string: String = users
        .iter()
        .map(|user| user.mention().to_string())
        .join("\n");

    let message_content = format!("{}\n{}", cfg.title, users_string);

    message
        .edit(ctx, |message| message.content(message_content))
        .await
        .unwrap();

    if cfg.pin {
        message.pin(ctx).await.unwrap();
    }

    message.id
}
