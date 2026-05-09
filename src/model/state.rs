use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub role: String,
    pub tailscale_ok: bool,
    pub server_reachable: bool,
    pub healthy: bool,
    pub summary: String,
}
