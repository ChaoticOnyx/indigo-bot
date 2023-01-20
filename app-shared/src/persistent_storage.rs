use std::collections::BTreeMap;

use app_macros::global;
use serde::{de::DeserializeOwned, Serialize};

use crate::prelude::*;

#[derive(Debug, Default)]
#[global(lock, set)]
pub struct PersistentStorage {
    cache: BTreeMap<String, serde_json::Value>,
}

impl PersistentStorage {
    #[instrument(skip(self, key, data))]
    pub fn save<T>(&mut self, key: impl ToString, data: T)
    where
        T: Serialize,
    {
        trace!("save");

        let key = key.to_string();
        let value = serde_json::to_value(data).unwrap();

        self.cache
            .entry(key)
            .and_modify(|entry| *entry = value.clone())
            .or_insert_with(|| value);

        self.flush();
    }

    #[instrument(skip(self, key))]
    pub fn load<T>(&self, key: impl ToString) -> Option<T>
    where
        T: DeserializeOwned,
    {
        trace!("load");

        let key = key.to_string();
        self.cache
            .get(&key)
            .map(|value| serde_json::from_value::<T>(value.clone()).unwrap())
    }

    #[instrument(skip(self))]
    fn flush(&self) {
        trace!("flush");

        let content = serde_json::to_vec(&self.cache).unwrap();
        std::fs::write(".cache.json", content).unwrap();
    }

    #[instrument]
    pub fn from_file() -> Option<Self> {
        trace!("from_file");

        let content = std::fs::read(".cache.json").ok()?;
        let cache = serde_json::from_slice::<BTreeMap<String, serde_json::Value>>(&content).ok()?;

        Some(Self { cache })
    }
}
