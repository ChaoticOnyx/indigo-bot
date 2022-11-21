use crate::prelude::*;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;

use super::constants::*;

#[instrument]
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Сообщить о баге или предложить фичу")
        .create_option(|option| {
            option
                .name(BUG_OPTION_NAME)
                .name_localized("ru", "баг")
                .description("Сообщить о баге")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name(TITLE_OPTION_NAME)
                        .name_localized("ru", "заголовок")
                        .description("Краткое описание, не более 80 символов")
                        .max_length(80)
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name(DESCRIPTION_OPTION_NAME)
                        .name_localized("ru", "описание")
                        .description("Описание бага, не более 2048 символов")
                        .max_length(2048)
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name(OS_OPTION_NAME)
                        .name_localized("ru", "ос")
                        .description("Ваша операционная система")
                        .kind(CommandOptionType::String)
                        .required(false)
                })
                .create_sub_option(|option| {
                    option
                        .name(GPU_OPTION_NAME)
                        .name_localized("ru", "видеокарта")
                        .description("Название вашей видеокарты и версия драйвера")
                        .kind(CommandOptionType::String)
                        .required(false)
                })
                .create_sub_option(|option| {
                    option
                        .name(LOGS_OPTION_NAME)
                        .name_localized("ru", "логи")
                        .description("Файл с логами client.stderr.log")
                        .kind(CommandOptionType::Attachment)
                        .required(false)
                })
                .create_sub_option(|option| {
                    option
                        .name(SCREENSHOT_OPTION_NAME)
                        .name_localized("ru", "скриншот")
                        .description("Скриншот")
                        .kind(CommandOptionType::Attachment)
                        .required(false)
                })
        })
        .create_option(|option| {
            option
                .name(FEATURE_OPTION_NAME)
                .name_localized("ru", "фича")
                .description("Предложить фичу")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("title")
                        .name_localized("ru", "заголовок")
                        .description("Не более 80 символов")
                        .max_length(80)
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("description")
                        .name_localized("ru", "описание")
                        .description("Описание фичи")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
        })
}
