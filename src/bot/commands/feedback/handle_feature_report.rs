use std::str::FromStr;

use crate::bot::commands::feedback::helpers::get_value_as_string;
use crate::prelude::*;
use serenity::model::prelude::Mention;
use serenity::model::prelude::{
    interaction::application_command::ApplicationCommandInteraction,
    interaction::InteractionResponseType::ChannelMessageWithSource, ReactionType,
};
use serenity::prelude::Context;

use super::constants::{DESCRIPTION_OPTION_NAME, TITLE_OPTION_NAME};
use super::helpers::create_feature_embed;

#[instrument(skip(ctx))]
pub async fn handle_feature_report(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    info!("handle_feature_report");
    let settings = Settings::clone_state().await;

    // Shortcuts
    let channel_id = &settings.commands.feedback.channel_id;
    let user = cmd.user.clone();
    let option = cmd.data.options.first().unwrap();
    let vote_up_emoji = settings.commands.feedback.vote_up_emoji.clone();
    let vote_down_emoji = settings.commands.feedback.vote_down_emoji.clone();
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
            .kind(ChannelMessageWithSource)
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

    Api::lock(async_closure!(|api| {
        api.new_feature_vote(feature_message.into()).await
    }))
    .await;
}
