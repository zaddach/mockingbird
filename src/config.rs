use std::{collections::HashMap, path::Path};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api: HashMap<String, Vec<String>>,
}

impl Config {
    /// Load configuration from a YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path)?;
        let config: Config = serde_yml::from_reader(file)?;
        Ok(config)
    }
}