use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context, Result};

use crate::app::paths::Paths;
use crate::config::store::Store;
use crate::model::state::State;
use crate::process::runner::{Output, Runner};
use crate::tooling::tailscale::{parse_status_json, status_args};
use crate::tooling::tmux::{list_sessions_args, parse_sessions};

#[derive(Debug, Clone)]
pub struct ServerInput {
    pub default_session: String,
    pub existing_sessions: Vec<String>,
    pub tailscale_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerState {
    pub actions: Vec<String>,
    pub healthy: bool,
}

pub fn reconcile_server(input: ServerInput) -> ServerState {
    if !input.tailscale_ready {
        return ServerState {
            actions: vec![],
            healthy: false,
        };
    }
    if input
        .existing_sessions
        .iter()
        .any(|name| name == &input.default_session)
    {
        return ServerState {
            actions: vec![],
            healthy: true,
        };
    }
    ServerState {
        actions: vec![format!("tmux:new-session:{}", input.default_session)],
        healthy: true,
    }
}

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

fn current_unix_seconds() -> Result<i64> {
    let seconds = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    i64::try_from(seconds).context("unix timestamp overflow converting u64 to i64")
}

pub fn run_once<R: Runner>(_paths: &Paths, store: &Store, runner: &R) -> Result<()> {
    let config = store.load_config()?;
    let server = config
        .server
        .as_ref()
        .context("server daemon requires server config")?;

    let tailscale_output = run_checked(runner, "tailscale", &status_args())?;
    let tailscale_status = parse_status_json(&tailscale_output.stdout)?;
    let tailscale_ok = tailscale_status.backend_state == "Running";

    let tmux_output = run_checked(runner, "tmux", &list_sessions_args())?;
    let sessions = parse_sessions(&tmux_output.stdout);
    let default_session_present = sessions
        .iter()
        .any(|session_name| session_name == &server.default_session);
    let healthy = tailscale_ok && default_session_present;

    store.save_state(&State {
        role: config.role,
        tailscale_ok,
        server_reachable: true,
        healthy,
        summary: if healthy {
            "server daemon healthy".into()
        } else {
            "server daemon degraded".into()
        },
        tailscale_dns: tailscale_status
            .dns_name
            .or_else(|| server.tailscale_dns.clone()),
        daemon_healthy: true,
        daemon_heartbeat_unix: current_unix_seconds()?,
        default_session_present,
        known_sessions: sessions,
        syncs: vec![],
    })?;

    Ok(())
}
