use std::{collections::HashMap, path::{Path, PathBuf}};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub type_aliases: HashMap<String, String>,
    /// Paths for template files.
    /// Paths are relative to the config file.
    pub templates: Vec<String>,
    pub output_dir: Option<PathBuf>,
    /// Path for include files.
    /// Paths are relative to the config file.
    #[serde(default)]
    pub includes: Vec<String>,
}

impl Config {
    /// Load configuration from a YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path)?;
        let config: Config = serde_yml::from_reader(file)?;
        Ok(config)
    }
}