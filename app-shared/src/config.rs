use crate::prelude::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConfigType(pub String);

impl Display for ConfigType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for ConfigType {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConfigType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
pub trait Config: Clone + Serialize + DeserializeOwned + Sync + Send + 'static {
    async fn get() -> Option<Self>;
    async fn save(self) -> Self;
    fn __type(&self) -> ConfigType;
}
