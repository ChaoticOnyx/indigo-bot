use super::constants::*;
use app_api::Api;
use app_shared::{
    models::BugReport,
    prelude::*,
    serenity::{
        model::prelude::{
            interaction::{
                application_command::ApplicationCommandInteraction, InteractionResponseType,
            },
            Mention,
        },
        prelude::Context,
    },
};

use crate::commands::feedback::helpers::{get_attachment_url_from_option, get_value_as_string};

#[instrument(skip(ctx))]
pub async fn handle_bug_report(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    trace!("handle_bug_report");

    let option = cmd.data.options.first().unwrap();
    let author_id = cmd.user.id;
    let mut bug_title = String::new();
    let mut bug_description = String::new();
    let mut bug_os: Option<String> = None;
    let mut bug_gpu: Option<String> = None;
    let mut bug_logs_attachment: Option<String> = None;
    let mut bug_screenshot_attachment: Option<String> = None;

    for option in &option.options {
        match option.name.as_str() {
            TITLE_OPTION_NAME => bug_title = get_value_as_string(option),
            DESCRIPTION_OPTION_NAME => bug_description = get_value_as_string(option),
            OS_OPTION_NAME => bug_os = Some(get_value_as_string(option)),
            GPU_OPTION_NAME => bug_gpu = Some(get_value_as_string(option)),
            LOGS_OPTION_NAME => bug_logs_attachment = Some(get_attachment_url_from_option(option)),
            SCREENSHOT_OPTION_NAME => {
                bug_screenshot_attachment = Some(get_attachment_url_from_option(option))
            }
            _ => (),
        }
    }

    let mut body = String::new();

    body += &format!("{bug_description}\n\n");

    if let Some(os) = bug_os {
        body += &format!("**ОС:** {os}\n");
    }

    if let Some(gpu) = bug_gpu {
        body += &format!("**Видеокарта:** {gpu}\n");
    }

    if let Some(logs_url) = bug_logs_attachment {
        body += &format!("**[Логи]({logs_url})**\n");
    }

    if let Some(screenshot_url) = bug_screenshot_attachment {
        body += &format!("**[Скриншот]({screenshot_url})**\n");
    }

    let author = format!(
        "{}#{} ({})",
        cmd.user.name, cmd.user.discriminator, author_id
    );

    body += &format!(
        "_Этот иссуй был создан автоматически по сообщению из дискорда. Автор: {author}._"
    );

    let issue_id = Api::lock_async(move |api| {
        let issue_id = api.create_bug_issue(bug_title, body);

        let bugreport = BugReport::new(author_id, issue_id);
        api.add_bug_report(bugreport);
        issue_id
    })
    .await
    .unwrap();

    debug!("responding to user");
    cmd.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message
                    .allowed_mentions(|mentions| mentions.users(&[cmd.user.clone()]))
                    .content(format!(
                        "{}, ваш багрепорт с номером **#{}** создан",
                        Mention::User(cmd.user.id),
                        issue_id
                    ))
            })
    })
    .await
    .unwrap();
}
