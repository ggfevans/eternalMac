use crate::tooling::mutagen::SYNC_MODE_TWO_WAY_RESOLVED;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair {
    pub name: String,
    pub local: String,
    pub remote: String,
    pub mode: String,
}

pub fn normalize_mode(mode: Option<&str>) -> String {
    let Some(raw_mode) = mode else {
        return SYNC_MODE_TWO_WAY_RESOLVED.into();
    };

    let normalized = raw_mode.trim().to_ascii_lowercase().replace('_', "-");
    if normalized == SYNC_MODE_TWO_WAY_RESOLVED {
        return normalized;
    }

    SYNC_MODE_TWO_WAY_RESOLVED.into()
}

pub fn build_pair(name: &str, local: &str, remote: &str, mode: Option<&str>) -> Pair {
    Pair {
        name: name.into(),
        local: local.into(),
        remote: remote.into(),
        mode: normalize_mode(mode),
    }
}
