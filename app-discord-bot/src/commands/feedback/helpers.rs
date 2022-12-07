use app_shared::{
    prelude::*,
    serenity::{
        builder::CreateEmbed,
        model::prelude::{
            interaction::application_command::{CommandDataOption, CommandDataOptionValue},
            ChannelId, Message,
        },
        prelude::Context,
        utils::Color,
    },
    state::DiscordSession,
};

#[instrument(skip(ctx))]
pub async fn create_and_pin_message(
    ctx: &Context,
    channel_id: &ChannelId,
    message_content: &str,
) -> Message {
    debug!("create_and_pin_message");

    let new_message = channel_id
        .send_message(&ctx.http, |msg| msg.content(message_content))
        .await
        .unwrap();

    new_message.pin(&ctx.http).await.unwrap();

    new_message
}

#[instrument]
pub fn create_feature_embed(
    embed: &mut CreateEmbed,
    title: String,
    description: String,
    vote_up_emoji: String,
    vote_down_emoji: String,
    votes_up: u64,
    votes_down: u64,
) -> &mut CreateEmbed {
    debug!("create_feature_embed");

    embed
        .author(|author| author.name("Фича"))
        .color(Color::PURPLE)
        .title(title)
        .description(description)
        .fields([
            (vote_up_emoji, votes_up.to_string(), true),
            (vote_down_emoji, votes_down.to_string(), true),
        ])
}

pub fn get_value_as_string(option: &CommandDataOption) -> String {
    let value = option.value.as_ref().unwrap();

    value.as_str().unwrap().to_string()
}

pub fn get_attachment_url_from_option(option: &CommandDataOption) -> String {
    let value = option.resolved.as_ref().unwrap();

    if let CommandDataOptionValue::Attachment(attachment) = value {
        attachment.url.clone()
    } else {
        panic!("expected attachment");
    }
}

pub async fn is_user_id_mine(user_id: DiscordUserId) -> bool {
    let session = DiscordSession::clone_state().await;

    if let Some(user) = session.user {
        if user_id == user.id {
            return true;
        }
    }

    false
}
