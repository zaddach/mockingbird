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
    /// Include paths and a list of files they contain.
    /// Include paths are relative to the config file path.
    /// Include files may be nested paths inside the include directory.
    #[serde(default)]
    pub includes: HashMap<String, Vec<String>>,
}

impl Config {
    /// Load configuration from a YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path)?;
        let config: Config = serde_yml::from_reader(file)?;
        Ok(config)
    }
}