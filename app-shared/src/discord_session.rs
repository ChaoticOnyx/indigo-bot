use app_macros::global;
use serenity::model::user::CurrentUser;

#[derive(Debug, Clone)]
#[global(set, clone)]
pub struct DiscordSession {
    pub user: Option<CurrentUser>,
}
