use crate::cli::RESERVED_NAMES;
use crate::error::{JotError, Result};
use crate::paths;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub jot: BTreeMap<String, Variant>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Variant {
    pub subcommand: String,
    pub description: Option<String>,
}

pub fn load(home: &Path) -> Result<Config> {
    let path = paths::config_path(home);
    if !path.exists() {
        return Ok(Config::default());
    }
    let text = std::fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&text)?;
    for key in config.jot.keys() {
        if RESERVED_NAMES.contains(&key.as_str()) {
            return Err(JotError::ReservedName(key.clone()));
        }
    }
    Ok(config)
}

pub fn save(home: &Path, config: &Config) -> Result<()> {
    let text = toml::to_string_pretty(config)?;
    std::fs::write(paths::config_path(home), text)?;
    Ok(())
}
