use app_api::Api;
use app_shared::{
    models::FeatureVoteDescriptor,
    prelude::*,
    serenity::{
        model::prelude::{Embed, Reaction},
        prelude::Context,
    },
};

use crate::commands::feedback::config::FeedbackConfig;
use crate::commands::feedback::{helpers::create_feature_embed, send_feature_to_github};

#[instrument(skip(ctx))]
pub async fn update_reactions(ctx: &Context, reaction: &Reaction) {
    trace!("updating reactions");

    let config = FeedbackConfig::get().unwrap();

    if reaction.channel_id != config.channel_id {
        debug!("reaction from another channel");
        return;
    }

    let descriptor = FeatureVoteDescriptor(reaction.message_id, reaction.channel_id);
    let is_vote_ended = Api::lock_async(move |api| api.is_vote_ended(descriptor))
        .await
        .unwrap();

    if is_vote_ended {
        debug!("message not found");
        return;
    }

    let mut message = reaction.message(&ctx.http).await.unwrap();
    let mut votes_up = 0;
    let mut votes_down = 0;

    let vote_up_emoji = config.vote_up_emoji.clone();
    let vote_down_emoji = config.vote_down_emoji.clone();

    for reaction in &message.reactions {
        let emoji = reaction.reaction_type.to_string();

        debug!(emoji);
        debug!(config.vote_up_emoji);
        debug!("{}", emoji == config.vote_up_emoji);

        if emoji == config.vote_up_emoji {
            votes_up = reaction.count - 1;
        } else if emoji == config.vote_down_emoji {
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

    if votes_up.saturating_sub(votes_down) >= config.min_feature_up_votes {
        Api::lock_async(move |api| {
            api.end_feature_vote(descriptor);
        })
        .await
        .unwrap();

        let feature_vote = Api::lock_async(move |api| api.get_feature_vote(descriptor))
            .await
            .unwrap()
            .unwrap();

        let author = feature_vote.author_id.to_user(&ctx.http).await.ok();

        if let Some(author) = author {
            send_feature_to_github(&message, &author).await;
        } else {
            warn!("user not found {}", feature_vote.author_id);
        }

        Api::lock_async(move |api| {
            api.end_feature_vote(descriptor);
        })
        .await
        .unwrap();
    }
}
