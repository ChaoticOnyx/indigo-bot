use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct DonationTierId(pub i64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonationTier {
    pub id: DonationTierId,
    pub name: String,
}

impl PartialEq for DonationTier {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DonationTier {}

impl Hash for DonationTier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
