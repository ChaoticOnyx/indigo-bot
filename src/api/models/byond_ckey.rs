use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct ByondCkey(pub String);
