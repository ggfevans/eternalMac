use eternalmac::config::store::Store;
use eternalmac::model::config::{ClientConfig, Config, Role, ServerConfig, SessionConfig};

#[test]
fn config_round_trip_preserves_role_and_default_session() {
    let tempdir = tempfile::tempdir().unwrap();
    let store = Store::new(tempdir.path().to_path_buf());

    let config = Config {
        role: Role::Server,
        server: ServerConfig {
            host_label: "mac-mini".into(),
            default_session: "default".into(),
            boot_sessions: vec!["default".into()],
        },
        client: ClientConfig {
            paired_server: String::new(),
            pinned: vec![],
        },
        session: SessionConfig { auto_attach: true },
    };

    store.save_config(&config).unwrap();
    let loaded = store.load_config().unwrap();

    assert!(matches!(loaded.role, Role::Server));
    assert_eq!(loaded.server.default_session, "default");
}
