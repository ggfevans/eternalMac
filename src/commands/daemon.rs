use std::thread;
use std::time::Duration;

use anyhow::Result;

use crate::app::context::AppContext;

const DAEMON_INTERVAL: Duration = Duration::from_secs(30);

pub fn run_server() -> Result<()> {
    let context = AppContext::from_env()?;
    loop {
        crate::daemon::server::run_once(&context.paths, &context.store, &context.runner)?;
        thread::sleep(DAEMON_INTERVAL);
    }
}

pub fn run_client() -> Result<()> {
    let context = AppContext::from_env()?;
    loop {
        crate::daemon::client::run_once(&context.paths, &context.store, &context.runner)?;
        thread::sleep(DAEMON_INTERVAL);
    }
}
