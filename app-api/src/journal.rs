use app_macros::global;
use app_shared::{
    chrono::Utc,
    models::{ActionType, Actor, JournalEntry},
    prelude::*,
    Database,
};

#[derive(Debug, Default)]
#[global(set, lock)]
pub struct Journal;

impl Journal {
    #[instrument]
    pub fn log(&self, object: Actor, subject: Option<Actor>, action: ActionType) -> JournalEntry {
        trace!("log");

        Database::lock(|database| database.add_journal_entry(object, Utc::now(), subject, action))
    }
}
