use std::cell::RefCell;

use eternalmac::app::paths::Paths;
use eternalmac::commands::attach::{resolve_session_name, run_with};
use eternalmac::config::store::Store;
use eternalmac::model::config::{ClientConfig, Config, Role, SessionConfig};
use eternalmac::process::runner::{Output, Runner};
use eternalmac::tooling::et::build_attach_args;

struct FakeRunner {
    calls: RefCell<Vec<(String, Vec<String>)>>,
    output: Output,
}

impl FakeRunner {
    fn success() -> Self {
        Self {
            calls: RefCell::new(vec![]),
            output: Output {
                stdout: String::new(),
                stderr: String::new(),
                success: true,
            },
        }
    }

    fn failure(stderr: &str) -> Self {
        Self {
            calls: RefCell::new(vec![]),
            output: Output {
                stdout: String::new(),
                stderr: stderr.into(),
                success: false,
            },
        }
    }
}

impl Runner for FakeRunner {
    fn run(&self, program: &str, args: &[String]) -> anyhow::Result<Output> {
        self.calls
            .borrow_mut()
            .push((program.to_string(), args.to_vec()));
        Ok(self.output.clone())
    }
}

fn save_client_config(store: &Store, paired_server: &str) {
    store
        .save_config(&Config {
            role: Role::Client,
            server: None,
            client: Some(ClientConfig {
                paired_server: paired_server.into(),
                pinned: vec![],
                sync_pairs: vec![],
            }),
            session: SessionConfig { auto_attach: true },
        })
        .unwrap();
}

#[test]
fn attach_args_target_named_tmux_session() {
    assert_eq!(
        build_attach_args("mac-mini", "default"),
        vec!["mac-mini", "-c", "tmux attach -t 'default'"]
    );
}

#[test]
fn attach_args_quote_session_names_with_spaces() {
    assert_eq!(
        build_attach_args("mac-mini", "pair programming"),
        vec!["mac-mini", "-c", "tmux attach -t 'pair programming'"]
    );
}

#[test]
fn attach_args_escape_single_quotes_in_session_names() {
    assert_eq!(
        build_attach_args("mac-mini", "dhruvil's session"),
        vec!["mac-mini", "-c", "tmux attach -t 'dhruvil'\\''s session'"]
    );
}

#[test]
fn session_selection_falls_back_to_default_for_missing_or_blank_values() {
    assert_eq!(resolve_session_name(None), "default");
    assert_eq!(resolve_session_name(Some("  ")), "default");
    assert_eq!(resolve_session_name(Some(" pair ")), "pair");
}

#[test]
fn attach_run_uses_client_server_and_default_session() {
    let tempdir = tempfile::tempdir().unwrap();
    let paths = Paths::new(tempdir.path().to_path_buf());
    let store = Store::new(paths);
    save_client_config(&store, "mac-mini");
    let runner = FakeRunner::success();

    run_with(&store, &runner, None).unwrap();

    let calls = runner.calls.borrow();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, "et");
    assert_eq!(calls[0].1, build_attach_args("mac-mini", "default"));
}

#[test]
fn attach_run_returns_clear_error_on_non_zero_exit() {
    let tempdir = tempfile::tempdir().unwrap();
    let paths = Paths::new(tempdir.path().to_path_buf());
    let store = Store::new(paths);
    save_client_config(&store, "mac-mini");
    let runner = FakeRunner::failure("unable to connect");

    let error = run_with(&store, &runner, Some("pair")).unwrap_err();
    let message = error.to_string();

    assert!(message.contains("command failed: et"));
    assert!(message.contains("stderr: unable to connect"));
}
