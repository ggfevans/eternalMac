pub fn new_session_args(name: &str) -> Vec<String> {
    vec!["new-session".into(), "-d".into(), "-s".into(), name.into()]
}

pub fn list_sessions_args() -> Vec<String> {
    vec!["list-sessions".into(), "-F".into(), "#S".into()]
}

pub fn parse_sessions(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}
