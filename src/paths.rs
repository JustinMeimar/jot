use crate::error::{JotError, Result};
use std::path::{Path, PathBuf};

pub fn jot_home() -> Result<PathBuf> {
    let home = std::env::var("HOME").map_err(|_| JotError::NoHome)?;
    Ok(PathBuf::from(home).join(".jot"))
}

pub fn config_path(home: &Path) -> PathBuf {
    home.join("config.toml")
}

pub fn variant_dir(home: &Path, key: &str) -> PathBuf {
    home.join(key)
}

pub fn template_path(home: &Path, key: &str) -> PathBuf {
    home.join("templates").join(format!("{key}.template"))
}
