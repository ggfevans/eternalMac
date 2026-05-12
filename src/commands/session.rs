use anyhow::{anyhow, Context, Result};

use crate::app::context::AppContext;
use crate::config::store::Store;
use crate::process::runner::{Output, Runner};
use crate::session::service;
use crate::tooling::tmux::{list_sessions_args, new_session_args, parse_sessions};

fn run_checked<R: Runner>(runner: &R, program: &str, args: &[String]) -> Result<Output> {
    let output = runner.run(program, args)?;
    if output.success {
        return Ok(output);
    }

    let mut message = format!("command failed: {program} {}", args.join(" "));
    if !output.stderr.trim().is_empty() {
        message.push_str(&format!("; stderr: {}", output.stderr.trim()));
    }

    Err(anyhow!(message))
}

pub fn list_with<R: Runner>(runner: &R) -> Result<Vec<String>> {
    let output = run_checked(runner, "tmux", &list_sessions_args())?;
    Ok(parse_sessions(&output.stdout))
}

pub fn create_with<R: Runner>(runner: &R, name: &str) -> Result<()> {
    run_checked(runner, "tmux", &new_session_args(name))?;
    Ok(())
}

pub fn pin_session_with(store: &Store, name: &str) -> Result<Vec<String>> {
    let mut config = store
        .load_config()
        .context("loading client config for session pin")?;
    let client = config.client.as_mut().context(
        "session pin requires client config; run `eternalMac setup client` on this machine first",
    )?;
    client.pinned = service::pin(client.pinned.clone(), name);
    let pinned = client.pinned.clone();
    store
        .save_config(&config)
        .context("saving client config after session pin")?;

    Ok(pinned)
}

pub fn unpin_session_with(store: &Store, name: &str) -> Result<Vec<String>> {
    let mut config = store
        .load_config()
        .context("loading client config for session unpin")?;
    let client = config.client.as_mut().context(
        "session unpin requires client config; run `eternalMac setup client` on this machine first",
    )?;
    client.pinned = service::unpin(client.pinned.clone(), name);
    let pinned = client.pinned.clone();
    store
        .save_config(&config)
        .context("saving client config after session unpin")?;

    Ok(pinned)
}

pub fn list() -> Result<()> {
    let context = AppContext::from_env()?;
    for session_name in list_with(&context.runner)? {
        println!("{session_name}");
    }
    Ok(())
}

pub fn create(name: &str) -> Result<()> {
    let context = AppContext::from_env()?;
    create_with(&context.runner, name)
}

pub fn pin_session(name: &str) -> Result<()> {
    let context = AppContext::from_env()?;
    pin_session_with(&context.store, name)?;
    Ok(())
}

pub fn unpin_session(name: &str) -> Result<()> {
    let context = AppContext::from_env()?;
    unpin_session_with(&context.store, name)?;
    Ok(())
}
