use app_macros::config;
use app_shared::serenity::model::id::GuildId;

#[config]
#[derive(Debug)]
pub struct DiscordConfig {
    pub guild_id: GuildId,
    pub token: String,
}
