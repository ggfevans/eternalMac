use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Server,
    Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub role: Role,
    pub server: Option<ServerConfig>,
    pub client: Option<ClientConfig>,
    pub session: SessionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host_label: String,
    pub default_session: String,
    pub boot_sessions: Vec<String>,
    pub tailscale_dns: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub paired_server: String,
    pub pinned: Vec<String>,
    #[serde(default)]
    pub sync_pairs: Vec<SyncPairConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPairConfig {
    pub name: String,
    pub local: String,
    pub remote: String,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub auto_attach: bool,
}
