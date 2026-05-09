pub const SYNC_MODE_TWO_WAY_RESOLVED: &str = "two-way-resolved";

pub fn build_create_args(name: &str, local: &str, remote: &str) -> Vec<String> {
    vec![
        "sync".into(),
        "create".into(),
        "--name".into(),
        name.into(),
        "--sync-mode".into(),
        SYNC_MODE_TWO_WAY_RESOLVED.into(),
        local.into(),
        remote.into(),
    ]
}

pub fn list_args() -> Vec<String> {
    vec!["sync".into(), "list".into()]
}
