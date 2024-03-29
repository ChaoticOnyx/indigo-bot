use crate::{
    commands::{self, feedback::COMMAND_NAME},
    roles_bind, roles_list,
};
use app_shared::{
    prelude::*,
    serenity::{
        model::prelude::{
            interaction::Interaction, ChannelId, GuildId, Member, Message, MessageId, Reaction,
            Ready,
        },
        prelude::*,
    },
    DiscordConfig, DiscordSession,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    #[instrument(skip(self, ctx))]
    async fn guild_member_update(
        &self,
        ctx: Context,
        old_if_available: Option<Member>,
        new: Member,
    ) {
        trace!("guild_member_update");

        let guild_id = new.guild_id;
        let config = DiscordConfig::get().unwrap();

        if config.guild_id != guild_id {
            return;
        }

        roles_list::guild_member_update(&ctx, &old_if_available, &new).await;
        roles_bind::guild_member_update(&ctx, &old_if_available, &new).await;
    }

    #[instrument(skip(self, ctx))]
    async fn message(&self, ctx: Context, new_message: Message) {
        trace!("message");

        let guild_id = new_message.guild_id;
        let config = DiscordConfig::get().unwrap();

        if guild_id.is_some() && config.guild_id != guild_id.unwrap() {
            return;
        }

        commands::tfa::handlers::message(&ctx, &new_message).await;
    }

    #[instrument(skip(self, ctx))]
    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        trace!("message_delete");

        let config = DiscordConfig::get().unwrap();

        if guild_id.is_some() && config.guild_id != guild_id.unwrap() {
            return;
        }

        commands::feedback::handlers::message_delete(
            &ctx,
            channel_id,
            deleted_message_id,
            guild_id,
        )
        .await;
    }

    #[instrument(skip(self, ctx))]
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        trace!("reaction_remove");

        let config = DiscordConfig::get().unwrap();

        if reaction.guild_id.is_some() && config.guild_id != reaction.guild_id.unwrap() {
            return;
        }

        commands::feedback::handlers::reaction_add(&ctx, &reaction).await;
    }

    #[instrument(skip(self, ctx))]
    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        trace!("reaction_remove");

        let config = DiscordConfig::get().unwrap();

        if reaction.guild_id.is_some() && config.guild_id != reaction.guild_id.unwrap() {
            return;
        }

        commands::feedback::handlers::reaction_remove(&ctx, &reaction).await;
    }

    #[instrument(skip(self, ctx))]
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("ready");

        DiscordSession::set_state(DiscordSession {
            user: Some(ready.user.clone()),
        });

        let config = DiscordConfig::get().unwrap();
        let guild = config.guild_id;

        info!("registering application commands");
        let commands = guild
            .set_application_commands(&ctx.http, |commands| {
                commands.create_application_command(commands::feedback::register)
            })
            .await
            .unwrap();

        info!(
            "registered commands: {}",
            commands
                .iter()
                .map(|cmd| cmd.name.clone())
                .collect::<String>()
        );

        commands::feedback::handlers::ready(&ctx, &ready).await;
        roles_list::ready(&ctx, &ready).await;
        roles_bind::ready(&ctx, &ready).await;
    }

    #[instrument(skip(self, ctx))]
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        trace!("interaction_create");

        let Interaction::ApplicationCommand(cmd) = interaction else {
            return;
        };

        let config = DiscordConfig::get().unwrap();

        if cmd.guild_id.is_some() && config.guild_id != cmd.guild_id.unwrap() {
            return;
        }

        #[allow(clippy::single_match)]
        match cmd.data.name.as_str() {
            COMMAND_NAME => commands::feedback::run(&ctx, &cmd).await,
            _ => (),
        };
    }
}
