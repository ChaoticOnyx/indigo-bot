use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct SS14Guid(pub String);
