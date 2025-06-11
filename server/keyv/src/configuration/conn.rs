use std::net::Ipv4Addr;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ConnectionConfig {
    pub port: usize,
    pub listen_addr: String,
    pub master_pwd: Arc<String>,
    pub max_incoming_bytes: usize,
}
impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            port: 1032,
            listen_addr: Ipv4Addr::LOCALHOST.to_string(),
            master_pwd: Arc::new("toor".to_string()),
            max_incoming_bytes: 1048576, // ~1MB
        }
    }
}
