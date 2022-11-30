use serenity::{model::prelude::Reaction, prelude::Context};

use crate::bot::commands::feedback::{helpers::is_user_id_mine, update_reactions};
use crate::prelude::*;

#[instrument(skip(ctx))]
pub async fn reaction_remove(ctx: &Context, reaction: &Reaction) {
    debug!("reaction_remove");

    let settings = Settings::clone_state().await.commands.feedback;

    if reaction.channel_id != settings.channel_id
        || is_user_id_mine(reaction.user_id.unwrap()).await
    {
        return;
    }

    update_reactions(ctx, reaction).await;
}
