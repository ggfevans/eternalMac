use std::cell::RefCell;
use std::collections::BTreeMap;

use anyhow::Result;
use eternalmac::app::paths::Paths;
use eternalmac::config::store::Store;
use eternalmac::model::config::{
    ClientConfig, Config, Role, ServerConfig, SessionConfig, SyncPairConfig,
};
use eternalmac::process::runner::{Output, Runner};
use eternalmac::tooling::mutagen::list_args;
use eternalmac::tooling::tailscale::status_args;
use eternalmac::tooling::tmux::list_sessions_args;

#[derive(Debug, Default)]
struct FakeRunner {
    responses: BTreeMap<(String, Vec<String>), Output>,
    calls: RefCell<Vec<(String, Vec<String>)>>,
}

impl FakeRunner {
    fn with_response(mut self, program: &str, args: Vec<String>, output: Output) -> Self {
        self.responses.insert((program.to_string(), args), output);
        self
    }
}

impl Runner for FakeRunner {
    fn run(&self, program: &str, args: &[String]) -> Result<Output> {
        self.calls
            .borrow_mut()
            .push((program.to_string(), args.to_vec()));

        if let Some(output) = self
            .responses
            .get(&(program.to_string(), args.to_vec()))
            .cloned()
        {
            return Ok(output);
        }

        Ok(Output {
            stdout: String::new(),
            stderr: String::new(),
            success: true,
        })
    }
}

#[test]
fn server_run_once_refreshes_state_and_marks_daemon_healthy() {
    let tempdir = tempfile::tempdir().unwrap();
    let paths = Paths::new(tempdir.path().to_path_buf());
    let store = Store::new(paths.clone());
    let runner = FakeRunner::default()
        .with_response(
            "tailscale",
            status_args(),
            Output {
                stdout:
                    r#"{"BackendState":"Running","Self":{"DNSName":"mac-mini.example.ts.net"}}"#
                        .into(),
                stderr: String::new(),
                success: true,
            },
        )
        .with_response(
            "tmux",
            list_sessions_args(),
            Output {
                stdout: "default\npairing\n".into(),
                stderr: String::new(),
                success: true,
            },
        );

    store
        .save_config(&Config {
            role: Role::Server,
            server: Some(ServerConfig {
                host_label: "mac-mini".into(),
                default_session: "default".into(),
                boot_sessions: vec!["default".into()],
                tailscale_dns: Some("mac-mini.example.ts.net".into()),
            }),
            client: None,
            session: SessionConfig { auto_attach: true },
        })
        .unwrap();

    eternalmac::daemon::server::run_once(&paths, &store, &runner).unwrap();

    let state = store.load_state().unwrap();
    assert!(matches!(state.role, Role::Server));
    assert!(state.daemon_healthy);
    assert!(state.daemon_heartbeat_unix > 0);
    assert!(state.default_session_present);
    assert_eq!(state.known_sessions, vec!["default", "pairing"]);
    assert!(state.syncs.is_empty());

    let calls = runner.calls.borrow();
    assert!(calls
        .iter()
        .any(|(program, args)| program == "tailscale" && args == &status_args()));
    assert!(calls
        .iter()
        .any(|(program, args)| program == "tmux" && args == &list_sessions_args()));
}

#[test]
fn client_run_once_refreshes_sync_state_from_config_and_mutagen_listing() {
    let tempdir = tempfile::tempdir().unwrap();
    let paths = Paths::new(tempdir.path().to_path_buf());
    let store = Store::new(paths.clone());
    let runner = FakeRunner::default()
        .with_response(
            "tailscale",
            status_args(),
            Output {
                stdout:
                    r#"{"BackendState":"Running","Self":{"DNSName":"mac-mini.example.ts.net"}}"#
                        .into(),
                stderr: String::new(),
                success: true,
            },
        )
        .with_response(
            "mutagen",
            list_args(),
            Output {
                stdout: "Name: project\nStatus: Watching for changes\n".into(),
                stderr: String::new(),
                success: true,
            },
        );

    store
        .save_config(&Config {
            role: Role::Client,
            server: None,
            client: Some(ClientConfig {
                paired_server: "mac-mini.example.ts.net".into(),
                pinned: vec![],
                sync_pairs: vec![
                    SyncPairConfig {
                        name: "project".into(),
                        local: "/Users/me/project".into(),
                        remote: "mac-mini.example.ts.net:~/project".into(),
                        mode: "two-way-resolved".into(),
                    },
                    SyncPairConfig {
                        name: "docs".into(),
                        local: "/Users/me/docs".into(),
                        remote: "mac-mini.example.ts.net:~/docs".into(),
                        mode: "two-way-resolved".into(),
                    },
                ],
            }),
            session: SessionConfig { auto_attach: true },
        })
        .unwrap();

    eternalmac::daemon::client::run_once(&paths, &store, &runner).unwrap();

    let state = store.load_state().unwrap();
    assert!(matches!(state.role, Role::Client));
    assert!(state.daemon_healthy);
    assert!(state.daemon_heartbeat_unix > 0);
    assert_eq!(state.syncs.len(), 2);
    assert_eq!(state.syncs[0].name, "project");
    assert_eq!(state.syncs[0].local, "/Users/me/project");
    assert_eq!(state.syncs[0].remote, "mac-mini.example.ts.net:~/project");
    assert_eq!(state.syncs[0].mode, "two-way-resolved");
    assert_eq!(state.syncs[0].status, "active");
    assert_eq!(state.syncs[1].name, "docs");
    assert_eq!(state.syncs[1].status, "missing");

    let calls = runner.calls.borrow();
    assert!(calls
        .iter()
        .any(|(program, args)| program == "tailscale" && args == &status_args()));
    assert!(calls
        .iter()
        .any(|(program, args)| program == "mutagen" && args == &list_args()));
}
