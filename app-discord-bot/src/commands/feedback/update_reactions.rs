use app_api::Api;
use app_shared::{
    models::FeatureVoteDescriptor,
    prelude::*,
    serenity::{
        model::prelude::{Embed, Reaction},
        prelude::Context,
    },
    state::Settings,
};

use crate::commands::feedback::{helpers::create_feature_embed, send_feature_to_github};

#[instrument(skip(ctx))]
pub async fn update_reactions(ctx: &Context, reaction: &Reaction) {
    trace!("updating reactions");

    let settings = Settings::clone_state().await;

    if reaction.channel_id != settings.commands.feedback.channel_id {
        debug!("reaction from another channel");
        return;
    }

    let descriptor = FeatureVoteDescriptor(reaction.message_id, reaction.channel_id);
    let is_vote_ended = Api::lock(async_closure!(|api| {
        api.is_vote_ended(descriptor).await
    }))
    .await;

    if is_vote_ended {
        debug!("message not found");
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
        Api::lock(async_closure! {
            |api| {
                api.end_feature_vote(descriptor).await;
            }
        })
        .await;

        let feature_vote = Api::lock(async_closure!(|api| {
            api.get_feature_vote(descriptor).await
        }))
        .await
        .unwrap();

        let author = feature_vote.author_id.to_user(&ctx.http).await.ok();

        if let Some(author) = author {
            send_feature_to_github(&message, &author).await;
        } else {
            warn!("user not found {}", feature_vote.author_id);
        }

        Api::lock(async_closure!(|api| {
            api.end_feature_vote(descriptor).await;
        }))
        .await;
    }
}
