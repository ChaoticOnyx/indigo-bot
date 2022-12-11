use app_shared::{
    prelude::*,
    serenity::{model::prelude::Reaction, prelude::Context},
};

use crate::commands::feedback::config::FeedbackConfig;
use crate::commands::feedback::{helpers::is_user_id_mine, update_reactions};

#[instrument(skip(ctx))]
pub async fn reaction_remove(ctx: &Context, reaction: &Reaction) {
    trace!("reaction_remove");

    let config = FeedbackConfig::get().await.unwrap();

    if reaction.channel_id != config.channel_id || is_user_id_mine(reaction.user_id.unwrap()).await
    {
        return;
    }

    update_reactions(ctx, reaction).await;
}
