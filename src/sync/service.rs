#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair {
    pub name: String,
    pub local: String,
    pub remote: String,
    pub mode: String,
}

pub fn build_pair(name: &str, local: &str, remote: &str) -> Pair {
    Pair {
        name: name.into(),
        local: local.into(),
        remote: remote.into(),
        mode: "last-write-wins".into(),
    }
}
