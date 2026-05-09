use anyhow::Result;

use crate::app::paths::Paths;
use crate::config::store::Store;
use crate::model::config::{Config, Role, ServerConfig, SessionConfig};
use crate::model::state::State;
use crate::platform::launchd::{write_plist, Definition};
use crate::process::runner::Runner;
use crate::tooling::brew::{install_cask_args, install_formula_args};
use crate::tooling::tailscale::{parse_status_json, status_args};
use crate::tooling::tmux::new_session_args;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerSetupSummary {
    pub dns_name: String,
    pub default_session: String,
}

pub fn apply_server_setup<R: Runner>(
    paths: &Paths,
    store: &Store,
    runner: &R,
    host_label: String,
) -> Result<ServerSetupSummary> {
    let formulae = vec!["et".into(), "tmux".into(), "mutagen".into()];
    if let Some(args) = install_formula_args(&formulae) {
        runner.run("brew", &args)?;
    }

    let cask_args = install_cask_args("tailscale-app");
    runner.run("brew", &cask_args)?;

    let tailscale_args = status_args();
    let tailscale_status = runner.run("tailscale", &tailscale_args)?;
    let parsed_status = parse_status_json(&tailscale_status.stdout)?;
    let dns_name = parsed_status
        .dns_name
        .unwrap_or_else(|| format!("{host_label}.unknown.ts.net"));

    let default_session = "default".to_string();
    let config = Config {
        role: Role::Server,
        server: Some(ServerConfig {
            host_label,
            default_session: default_session.clone(),
            boot_sessions: vec![default_session.clone()],
            tailscale_dns: Some(dns_name.clone()),
        }),
        client: None,
        session: SessionConfig { auto_attach: true },
    };
    store.save_config(&config)?;

    store.save_state(&State {
        role: Role::Server,
        tailscale_ok: true,
        server_reachable: true,
        healthy: true,
        summary: "server ready".into(),
        tailscale_dns: Some(dns_name.clone()),
        daemon_healthy: true,
        daemon_heartbeat_unix: 0,
        default_session_present: true,
        known_sessions: vec![default_session.clone()],
        syncs: vec![],
    })?;

    write_plist(
        &paths.server_plist,
        &Definition {
            label: "com.eternalmac.server".into(),
            program_arguments: vec![
                "/opt/homebrew/bin/eternalMac".into(),
                "daemon".into(),
                "server".into(),
            ],
            run_at_load: true,
            keep_alive: true,
        },
    )?;

    let launchctl_args = vec![
        "load".into(),
        "-w".into(),
        paths.server_plist.display().to_string(),
    ];
    runner.run("launchctl", &launchctl_args)?;

    let tmux_args = new_session_args(&default_session);
    runner.run("tmux", &tmux_args)?;

    Ok(ServerSetupSummary {
        dns_name,
        default_session,
    })
}
