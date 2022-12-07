use app_shared::{
    prelude::*,
    serenity::{model::prelude::Reaction, prelude::Context},
    state::Settings,
};

use crate::commands::feedback::{helpers::is_user_id_mine, update_reactions};

#[instrument(skip(ctx))]
pub async fn reaction_add(ctx: &Context, reaction: &Reaction) {
    debug!("reaction_remove");

    let settings = Settings::clone_state().await.commands.feedback;

    if reaction.channel_id != settings.channel_id
        || is_user_id_mine(reaction.user_id.unwrap()).await
    {
        return;
    }

    update_reactions(ctx, reaction).await;
}
