use app_shared::{once_cell::sync::Lazy, prelude::*, serenity::prelude::Mutex};
use std::ops::{Deref, DerefMut};
use tera::Tera;

static TEMPLATES: Lazy<Mutex<Option<Templates>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Clone)]
pub struct Templates(pub Tera);

#[async_trait]
impl GlobalState for Templates {
    async fn get_static() -> &'static Lazy<Mutex<Option<Self>>> {
        &TEMPLATES
    }
}

impl GlobalStateSet for Templates {}
impl GlobalStateLock for Templates {}

impl Deref for Templates {
    type Target = Tera;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Templates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
