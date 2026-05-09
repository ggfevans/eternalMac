use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use crate::model::config::Config;
use crate::model::state::State;

#[derive(Debug, Clone)]
pub struct Store {
    base: PathBuf,
}

impl Store {
    pub fn new(base: PathBuf) -> Self {
        Self { base }
    }

    pub fn save_config(&self, config: &Config) -> Result<()> {
        fs::create_dir_all(&self.base)?;
        let serialized = toml::to_string_pretty(config)?;
        fs::write(self.base.join("config.toml"), serialized)?;
        Ok(())
    }

    pub fn load_config(&self) -> Result<Config> {
        let raw = fs::read_to_string(self.base.join("config.toml"))?;
        Ok(toml::from_str(&raw)?)
    }

    pub fn save_state(&self, state: &State) -> Result<()> {
        fs::create_dir_all(&self.base)?;
        let serialized = serde_json::to_string_pretty(state)?;
        fs::write(self.base.join("state.json"), serialized)?;
        Ok(())
    }
}
