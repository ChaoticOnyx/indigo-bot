use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct ServiceId(pub String);

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ServiceError {
    Any(String),
}
