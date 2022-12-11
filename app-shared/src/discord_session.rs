use crate::prelude::*;
use once_cell::sync::Lazy;
use serenity::{model::user::CurrentUser, prelude::Mutex};

static SESSION: Lazy<Mutex<Option<DiscordSession>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Clone)]
pub struct DiscordSession {
    pub user: Option<CurrentUser>,
}

#[async_trait]
impl GlobalState for DiscordSession {
    async fn get_static() -> &'static Lazy<Mutex<Option<Self>>> {
        &SESSION
    }
}

impl GlobalStateSet for DiscordSession {}
impl GlobalStateClone for DiscordSession {}
