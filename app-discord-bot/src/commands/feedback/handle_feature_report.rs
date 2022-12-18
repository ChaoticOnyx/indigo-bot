use std::str::FromStr;

use crate::commands::feedback::config::FeedbackConfig;
use app_api::Api;
use app_macros::tokio_blocking;
use app_shared::{
    models::FeatureVote,
    prelude::*,
    serenity::{
        model::prelude::{
            interaction::{
                application_command::ApplicationCommandInteraction, InteractionResponseType,
            },
            Mention, ReactionType,
        },
        prelude::Context,
    },
};

use super::constants::*;
use crate::commands::feedback::helpers::{create_feature_embed, get_value_as_string};

#[instrument(skip(ctx))]
pub async fn handle_feature_report(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    trace!("handle_feature_report");
    let config = FeedbackConfig::get().unwrap();

    // Shortcuts
    let channel_id = &config.channel_id;
    let user = cmd.user.clone();
    let option = cmd.data.options.first().unwrap();
    let vote_up_emoji = config.vote_up_emoji.clone();
    let vote_down_emoji = config.vote_down_emoji.clone();
    let mut feature_title = String::new();
    let mut feature_description = String::new();

    for option in &option.options {
        match option.name.as_str() {
            TITLE_OPTION_NAME => feature_title = get_value_as_string(option),
            DESCRIPTION_OPTION_NAME => feature_description = get_value_as_string(option),
            _ => (),
        }
    }

    debug!("creating message with voting");
    let feature_message = channel_id
        .send_message(&ctx.http, |message| {
            message
                .allowed_mentions(|mentions| mentions.users(&[user.clone()]))
                .content(Mention::User(user.id))
                .embed(|embed| {
                    create_feature_embed(
                        embed,
                        feature_title,
                        feature_description,
                        vote_up_emoji.clone(),
                        vote_down_emoji.clone(),
                        0,
                        0,
                    )
                })
                .reactions([
                    ReactionType::from_str(&vote_up_emoji).unwrap(),
                    ReactionType::from_str(&vote_down_emoji).unwrap(),
                ])
        })
        .await
        .unwrap();

    debug!("responding to user");
    cmd.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message
                    .allowed_mentions(|mentions| mentions.users(&[user.clone()]))
                    .content(format!(
                        "{}, {} 👍",
                        Mention::User(user.id),
                        feature_message.link()
                    ))
            })
    })
    .await
    .unwrap();

    let feature_vote = FeatureVote {
        author_id: cmd.user.id,
        ..feature_message.into()
    };

    Api::lock(tokio_blocking!(|api| {
        api.private_api.new_feature_vote(feature_vote).await
    }));
}
