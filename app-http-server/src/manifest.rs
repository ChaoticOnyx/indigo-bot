use app_macros::global;
use app_shared::{prelude::*, serde_json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use tera::try_get_value;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileEntry {
    pub file: PathBuf,
}

#[derive(Debug, Clone)]
#[global(set, lock)]
pub struct Manifest {
    map: BTreeMap<PathBuf, FileEntry>,
}

impl Manifest {
    pub fn new() -> Self {
        let content = fs::read("./public/manifest.json");

        let Ok(content) = content else {
            error!("error occured while reading manifest.json: '{}', try to recompile web assets", content.unwrap_err());
            std::process::exit(1);
        };

        let map = serde_json::from_slice(&content).unwrap();

        Self { map }
    }

    pub fn reload(&mut self) {
        let mut new = Self::new();

        std::mem::swap(&mut self.map, &mut new.map);
    }

    pub fn resolve(&self, file_path: &Path) -> Option<PathBuf> {
        self.map.get(file_path).map(|entry| entry.file.clone())
    }

    pub fn asset_path(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
        let value = value.clone();
        let path = Manifest::lock(|manifest| {
            let path = try_get_value!("asset_path", "value", PathBuf, value);
            Ok(manifest
                .resolve(&path)
                .map(|path| format!("/public/{}", path.display())))
        });

        path.map(|path| serde_json::to_value(path).unwrap())
    }
}
