use crate::configuration::internal::InternalConfig;
use keyv_core::instructions::init::InitResult;
use chacha20poly1305::aead::Aead;
use std::net::SocketAddr;

pub struct AuthKey {
    auth_key: Vec<u8>,
}

impl AuthKey {
    pub fn from_ip(ic: &InternalConfig, s: &SocketAddr) -> Self {
        let ip = s.to_string();
        let auth_key = ic.cipher.encrypt(&ic.nonce, ip.as_bytes()).unwrap();

        Self { auth_key }
    }
    pub fn into_init_result(self) -> InitResult {
        InitResult {
            auth_key: self.auth_key,
        }
    }
}
