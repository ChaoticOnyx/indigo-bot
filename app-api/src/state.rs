use crate::Api;
use app_shared::{once_cell::sync::Lazy, prelude::*, tokio::sync::Mutex};

static API: Lazy<Mutex<Option<Api>>> = Lazy::new(|| Mutex::new(None));

#[async_trait]
impl GlobalState for Api {
    async fn get_static() -> &'static Lazy<Mutex<Option<Self>>> {
        &API
    }
}

impl GlobalStateSet for Api {}
impl GlobalStateLock for Api {}
