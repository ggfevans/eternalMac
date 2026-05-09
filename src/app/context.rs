use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::app::paths::Paths;
use crate::config::store::Store;
use crate::process::runner::SystemRunner;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub paths: Paths,
    pub store: Store,
    pub runner: SystemRunner,
}

impl AppContext {
    pub fn from_env() -> Result<Self> {
        let home = env::var("HOME").context("HOME is not set")?;
        let paths = Paths::new(PathBuf::from(home));
        let store = Store::new(paths.clone());

        Ok(Self {
            paths,
            store,
            runner: SystemRunner,
        })
    }
}
