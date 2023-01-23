#[allow(clippy::module_inception)]
mod database;
mod db_config;
pub mod tables;

pub use database::Database;
