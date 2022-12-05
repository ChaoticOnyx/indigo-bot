use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct ServiceId(pub String);

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ServiceError {
    Any(String),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::Any(msg) => f.write_str(msg),
        }
    }
}
