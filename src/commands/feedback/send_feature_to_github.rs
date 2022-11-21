use crate::prelude::*;
use serenity::model::{prelude::Message, user::User};

#[instrument]
pub async fn send_feature_to_github(message: &Message, author: &User) {
    info!("send_feature_to_github");

    let settings = Settings::get_state().await;
    let github = Github::get_state().await;
    let embed = message.embeds.first().unwrap().clone();

    let author = format!("{}#{} ({})", author.name, author.discriminator, author.id);
    let content = format!("{}\n\n_Этот иссуй был создан автоматически по [сообщению из дискорда]({}). Автор: {author}._", embed.description.unwrap(), message.link());

    github
        .create_issue(
            settings.commands.feedback.features_repository,
            embed.title.unwrap(),
            content,
            settings.commands.feedback.feature_issue_labels,
        )
        .await;
}
