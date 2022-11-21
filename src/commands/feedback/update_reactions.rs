use crate::prelude::*;
use serenity::{
    model::prelude::{Embed, Reaction},
    prelude::Context,
};

use super::helpers::create_feature_embed;
use super::send_feature_to_github;

#[instrument(skip(ctx))]
pub async fn update_reactions(ctx: &Context, reaction: &Reaction) {
    debug!("updating reactions");

    let settings = Settings::get_state().await;

    if reaction.channel_id != settings.commands.feedback.channel_id {
        debug!("reaction from another channel");
        return;
    }

    let db = Database::get_state().await;
    if !db
        .has_not_vote_ended_feature_message(reaction.channel_id, reaction.message_id)
        .await
    {
        debug!("feature message not found");
        return;
    }

    let mut message = reaction.message(&ctx.http).await.unwrap();
    let mut votes_up = 0;
    let mut votes_down = 0;
    let vote_up_emoji = settings.commands.feedback.vote_up_emoji.clone();
    let vote_down_emoji = settings.commands.feedback.vote_down_emoji.clone();

    for reaction in &message.reactions {
        let emoji = reaction.reaction_type.to_string();

        debug!(emoji);
        debug!(settings.commands.feedback.vote_up_emoji);
        debug!("{}", emoji == settings.commands.feedback.vote_up_emoji);

        if emoji == settings.commands.feedback.vote_up_emoji {
            votes_up = reaction.count - 1;
        } else if emoji == settings.commands.feedback.vote_down_emoji {
            votes_down = reaction.count - 1;
        }
    }

    let Embed {
        title, description, ..
    } = message.embeds.first().cloned().unwrap();

    debug!("updating embed");
    message
        .edit(&ctx.http, |message| {
            message.embed(|embed| {
                create_feature_embed(
                    embed,
                    title.unwrap(),
                    description.unwrap(),
                    vote_up_emoji,
                    vote_down_emoji,
                    votes_up,
                    votes_down,
                )
            })
        })
        .await
        .unwrap();

    if votes_up.saturating_sub(votes_down) >= settings.commands.feedback.min_feature_up_votes {
        let db = Database::get_state().await;
        let user_id = db
            .get_feature_message_author(message.channel_id, message.id)
            .await;
        let author = user_id.to_user(&ctx.http).await.ok();

        if let Some(author) = author {
            send_feature_to_github(&message, &author).await;
        } else {
            warn!("user not found {}", user_id);
        }

        db.end_vote_feature_message(message.channel_id, message.id)
            .await;
    }
}
