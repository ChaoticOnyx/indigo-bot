use serenity::{model::prelude::Reaction, prelude::Context};

use crate::commands::feedback::{helpers::is_users_id_mine, update_reactions};
pub use crate::prelude::*;

#[instrument(skip(ctx))]
pub async fn reaction_add(ctx: &Context, reaction: &Reaction) {
    debug!("reaction_remove");

    let settings = Settings::get_state().await.commands.feedback;

    if reaction.channel_id != settings.channel_id
        || is_users_id_mine(reaction.user_id.unwrap(), None).await
    {
        return;
    }

    update_reactions(ctx, reaction).await;
}
