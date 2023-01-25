use crate::Api;
use app_shared::{models::JournalEntryCursor, prelude::*, Database};

impl Api {
    pub fn get_journal_entries(&self, current: JournalEntryCursor) -> Option<JournalEntryCursor> {
        trace!("get_journal_entries");

        Database::lock(|database| database.get_journal_entries(current))
    }
}
