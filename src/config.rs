use std::{ffi::OsString, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub install_dir: PathBuf,
}

#[derive(Debug)]
pub enum ConfigError {
    ConfigDirNotFound,
    ReadError(std::io::Error),
    ParseError(toml::de::Error),
    WriteError(std::io::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::ConfigDirNotFound => {
                write!(
                    f,
                    "Could not determine config directory (tried XDG_CONFIG_HOME and ~/.config)"
                )
            }
            ConfigError::ReadError(err) => {
                write!(f, "Failed to read config file: {}", err)
            }
            ConfigError::ParseError(err) => {
                write!(f, "Failed to parse config file: {}", err)
            }
            ConfigError::WriteError(err) => {
                write!(f, "Failed to write default config file: {}", err)
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::ReadError(err) => Some(err),
            ConfigError::ParseError(err) => Some(err),
            ConfigError::WriteError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::ReadError(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::ParseError(err)
    }
}

fn is_absolute_path(path: OsString) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    if path.is_absolute() { Some(path) } else { None }
}

fn config_dir() -> Option<PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .and_then(is_absolute_path)
        .or_else(|| std::env::home_dir().map(|h| h.join(".config")))
}

fn data_dir() -> Option<PathBuf> {
    std::env::var_os("XDG_DATA_HOME")
        .and_then(is_absolute_path)
        .or_else(|| std::env::home_dir().map(|h| h.join(".local/share")))
}

impl Configuration {
    fn default() -> Self {
        Configuration {
            install_dir: data_dir().map(|share| share.join("AppImages")).unwrap(),
        }
    }

    fn to_toml(&self) -> Result<String, ConfigError> {
        toml::to_string_pretty(self)
            .map_err(|e| ConfigError::WriteError(std::io::Error::new(std::io::ErrorKind::Other, e)))
    }
}

fn create_default_config(config_path: &PathBuf) -> Result<Configuration, ConfigError> {
    let default_config = Configuration::default();

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(ConfigError::WriteError)?;
    }

    let toml_content = default_config.to_toml()?;
    fs::write(config_path, toml_content).map_err(ConfigError::WriteError)?;

    println!("Created default config file at: {}!", config_path.display());

    Ok(default_config)
}

pub fn load() -> Result<Configuration, ConfigError> {
    let app_name = clap::crate_name!();

    let config_dir = config_dir().ok_or(ConfigError::ConfigDirNotFound)?;
    let config_path = config_dir.join(app_name).join(format!("{}.toml", app_name));

    if !config_path.exists() {
        create_default_config(&config_path)?;
    }

    let contents = fs::read_to_string(&config_path)?;
    let mut configuration: Configuration = toml::from_str(&contents)?;

    let install_dir_str = configuration.install_dir.to_string_lossy();
    let expanded = shellexpand::full(&install_dir_str)
        .map_err(|e| ConfigError::ReadError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    configuration.install_dir = PathBuf::from(expanded.as_ref());

    Ok(configuration)
}
