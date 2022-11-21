use crate::prelude::*;
use once_cell::sync::Lazy;
use serenity::{model::user::CurrentUser, prelude::Mutex};

static SESSION: Lazy<Mutex<Option<Session>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Clone)]
pub struct Session {
    pub user: Option<CurrentUser>,
}

#[async_trait]
impl GlobalState for Session {
    #[instrument]
    async fn get_state() -> Session {
        let lock = SESSION.lock().await;

        lock.clone().unwrap()
    }

    #[instrument]
    async fn set_state(session: Session) {
        let mut lock = SESSION.lock().await;

        *lock = Some(session);
    }
}
