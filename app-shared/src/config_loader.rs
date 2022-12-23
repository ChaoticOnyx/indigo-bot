use crate::config::ConfigType;
use app_macros::global;
use std::any::Any;
use std::collections::BTreeMap;
use std::fs;

use crate::prelude::*;

#[derive(Debug)]
#[global(lock, set)]
pub struct ConfigLoader {
    configs: BTreeMap<ConfigType, serde_yaml::Value>,
    cache: BTreeMap<ConfigType, Box<dyn Any + Sync + Send>>,
    files: BTreeMap<ConfigType, String>,
}

impl ConfigLoader {
    #[instrument]
    pub fn new(path: &str) -> Self {
        info!("new");

        let read_dir = fs::read_dir(path).unwrap();
        let mut configs = BTreeMap::new();
        let mut files = BTreeMap::new();

        for entry in read_dir {
            let Ok(entry) = entry else {
                continue;
            };

            let path = entry.path();
            let Some(extension) = path.extension() else {
                continue
            };

            if extension != "yml" {
                continue;
            }

            debug!("loading file {}", path.display());
            let content = fs::read(&path).unwrap();
            let value = match serde_yaml::from_slice::<serde_yaml::Value>(&content) {
                Err(err) => {
                    error!("error while reading '{}': '{err}'", path.display());
                    continue;
                }
                Ok(value) => value,
            };

            let config_type = value.get("type");

            let Some(config_type) = config_type else {
                error!("file '{}' does not have id field", path.display());
                continue;
            };

            let Some(config_type) = config_type.as_str() else {
                error!("file '{}' has non-string id field", path.display());
                continue;
            };

            let config_type = ConfigType(config_type.to_string());

            if configs.contains_key(&config_type) {
                error!("file '{}' has duplicated id", path.display());
                continue;
            }

            configs.insert(config_type.clone(), value);
            files.insert(config_type, path.to_string_lossy().to_string());
        }

        let cache = BTreeMap::new();

        Self {
            configs,
            cache,
            files,
        }
    }

    #[instrument]
    fn get_from_cache<T>(&self, config_type: &ConfigType) -> Option<T>
    where
        T: Config,
    {
        trace!("get_from_cache");

        let cached_value = self.cache.get(config_type)?;
        let value = cached_value.downcast_ref::<T>().cloned();

        match value {
            None => {
                error!("trying to return invalid type");
                None
            }
            Some(value) => {
                debug!("returning config from cache");
                Some(value)
            }
        }
    }

    #[instrument]
    pub fn find_config<T>(&mut self, config_type: ConfigType) -> Option<T>
    where
        T: Config,
    {
        trace!("get_config");

        if let Some(value) = self.get_from_cache(&config_type) {
            return Some(value);
        }

        let Some(config) = self.configs.get(&config_type) else {
            warn!("config '{}' not found", config_type);
            return None;
        };

        match serde_yaml::from_value::<T>(config.clone()) {
            Err(err) => {
                error!("error while parsing config '{config_type}': '{err}'");
                None
            }
            Ok(value) => {
                let value = Box::new(value);
                self.cache.insert(config_type, value.clone());

                Some(*value)
            }
        }
    }

    pub fn save_config<T>(&mut self, config: T) -> T
    where
        T: Config + Sync + Send + 'static,
    {
        let config_type = config.__type();
        let path = self.files.get(&config_type).unwrap();

        *self.configs.get_mut(&config_type).unwrap() = serde_yaml::to_value(&config).unwrap();
        *self.cache.get_mut(&config_type).unwrap() = Box::new(config.clone());

        fs::write(path, serde_yaml::to_string(&config).unwrap()).unwrap();

        config
    }
}
