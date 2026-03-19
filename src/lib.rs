use std::{ffi::OsString, path::PathBuf};

pub fn is_absolute_path(path: OsString) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    if path.is_absolute() { Some(path) } else { None }
}

pub fn config_dir() -> Option<PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .and_then(is_absolute_path)
        .or_else(|| std::env::home_dir().map(|h| h.join(".config")))
}

pub fn data_dir() -> Option<PathBuf> {
    std::env::var_os("XDG_DATA_HOME")
        .and_then(is_absolute_path)
        .or_else(|| std::env::home_dir().map(|h| h.join(".local/share")))
}
