use std::path::PathBuf;

use config::{Config, FileFormat};

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct HttpSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct LogSettings {
    pub directive: String,
    pub directory: PathBuf,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct Settings {
    pub image_directory: PathBuf,
    pub log: LogSettings,
    pub http: HttpSettings,
}

const PREFIX: &str = "IMAGE_RESIZE_API";
const SETTINGS_FILE_NAME: &str = "settings";

pub fn initialize() -> Settings {
    let mut config = Config::default();

    let base_settings = format!("{}.yaml", SETTINGS_FILE_NAME);
    config
        .merge(config::File::new(&base_settings, FileFormat::Yaml).required(false))
        .expect("Unable to read base settings.");

    if let Ok(env) = std::env::var(format!("{}_ENVIRONMENT", PREFIX)) {
        if !env.is_empty() {
            let env_settings = format!("{}.{}.yaml", SETTINGS_FILE_NAME, env);
            config
                .merge(config::File::new(&env_settings, FileFormat::Yaml).required(false))
                .expect("Unable to read environment settings.");
        }
    }

    config
        .merge(config::Environment::with_prefix(PREFIX).separator("_"))
        .expect("Unable to read settings from environment variables.");

    config.try_into().expect("Unable to convert settings.")
}
