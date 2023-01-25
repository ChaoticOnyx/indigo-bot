use app_shared::{models::Role, prelude::*, Database};

use crate::Api;

impl Api {
    #[instrument]
    pub fn get_roles(&self) -> Vec<Role> {
        trace!("get_roles");

        Database::lock(|database| database.get_roles())
    }
}
