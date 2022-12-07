use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RightsScope<T, B>
where
    T: Sized + Ord,
    B: Sized + Ord,
{
    Everything(B),
    Some(BTreeMap<T, B>),
    None,
}

impl<T, B> Default for RightsScope<T, B>
where
    T: Sized + Ord,
    B: Sized + Ord,
{
    fn default() -> Self {
        RightsScope::None
    }
}
