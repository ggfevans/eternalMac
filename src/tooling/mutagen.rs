pub fn build_create_args(name: &str, local: &str, remote: &str) -> Vec<String> {
    vec![
        "sync".into(),
        "create".into(),
        "--name".into(),
        name.into(),
        "--sync-mode".into(),
        "two-way-resolved".into(),
        local.into(),
        remote.into(),
    ]
}
