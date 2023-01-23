use app_macros::config;

#[config]
#[derive(Debug)]
pub struct DbConfig {
    pub connect: String,
}
