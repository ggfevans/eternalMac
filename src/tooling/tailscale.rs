use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variant {
    Standalone,
    AppStore,
    OpenSource,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub backend_state: String,
    pub dns_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawSelf {
    #[serde(rename = "DNSName")]
    dns_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawStatus {
    #[serde(rename = "BackendState")]
    backend_state: String,
    #[serde(rename = "Self")]
    me: Option<RawSelf>,
}

pub fn detect_variant(installed_apps: &[String]) -> Variant {
    if installed_apps
        .iter()
        .any(|path| path.ends_with("/Applications/Tailscale.app"))
    {
        return Variant::Standalone;
    }

    if installed_apps
        .iter()
        .any(|path| path.contains("Tailscale (App Store)"))
    {
        return Variant::AppStore;
    }

    if installed_apps
        .iter()
        .any(|path| path.contains("tailscaled"))
    {
        return Variant::OpenSource;
    }

    Variant::Unknown
}

pub fn parse_status_json(raw: &str) -> Result<Status> {
    let parsed: RawStatus = serde_json::from_str(raw)?;
    Ok(Status {
        backend_state: parsed.backend_state,
        dns_name: parsed.me.and_then(|me| me.dns_name),
    })
}

pub fn status_args() -> Vec<String> {
    vec!["status".into(), "--json".into()]
}

pub fn login_args() -> Vec<String> {
    vec!["login".into()]
}
