use crate::manifest::Manifest;
use app_shared::models::{Rights, Role};
use app_shared::{
    prelude::*,
    serde_json::{self, Value},
};
use std::collections::HashMap;
use std::path::PathBuf;
use tera::try_get_value;

pub fn main_role_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let value = value.clone();
    let roles: Vec<Role> = serde_json::from_value(value)?;
    let roles: Vec<Role> = roles
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.rights.bits(), &b.rights.bits()))
        .sorted_by(|a, b| Ord::cmp(&b.id, &a.id))
        .collect();

    serde_json::to_value(roles.get(0).cloned().unwrap_or_default())
        .map_err(|err| tera::Error::json(err))
}

pub fn rights_to_bits_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let value = value.clone();
    let rights: Rights = serde_json::from_value(value)?;

    serde_json::to_value(rights.bits()).map_err(|err| tera::Error::json(err))
}

pub fn asset_path_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let value = value.clone();
    let path = Manifest::lock(|manifest| {
        let path = try_get_value!("asset_path", "value", PathBuf, value);
        Ok(manifest
            .resolve(&path)
            .map(|path| format!("/public/{}", path.display())))
    });

    path.map(|path| serde_json::to_value(path).unwrap())
}
