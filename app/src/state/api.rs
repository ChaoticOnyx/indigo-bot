use crate::api::Api;
use once_cell::sync::Lazy;
use serenity::{async_trait, prelude::Mutex};

use super::GlobalState;
use crate::prelude::*;

static API: Lazy<Mutex<Option<Api>>> = Lazy::new(|| Mutex::new(None));

#[async_trait]
impl GlobalState for Api {
    async fn get_static() -> &'static Lazy<Mutex<Option<Self>>> {
        &API
    }
}

impl GlobalStateSet for Api {}
impl GlobalStateLock for Api {}
