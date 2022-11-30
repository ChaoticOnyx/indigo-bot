use crate::prelude::*;
use serenity::model::{prelude::Message, user::User};

#[instrument]
pub async fn send_feature_to_github(message: &Message, author: &User) {
    info!("send_feature_to_github");

    let embed = message.embeds.first().unwrap().clone();
    let author = format!("{}#{} ({})", author.name, author.discriminator, author.id);
    let content = format!("{}\n\n_Этот иссуй был создан автоматически по [сообщению из дискорда]({}). Автор: {author}._", embed.description.unwrap(), message.link());

    Api::lock(async_closure!(|api| {
        api.create_feature_issue(embed.title.unwrap(), content)
            .await;
    }))
    .await;
}
