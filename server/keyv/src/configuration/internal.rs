use chacha20::ChaCha20;
use chacha20::cipher::crypto_common::rand_core::OsRng;
use chacha20poly1305::aead::Nonce;
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, ChaChaPoly1305, KeyInit};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Clone)]
pub struct InternalConfig {
    pub cipher: Arc<ChaChaPoly1305<ChaCha20>>,

    pub nonce: Arc<Nonce<ChaChaPoly1305<ChaCha20>>>,
}

impl Default for InternalConfig {
    fn default() -> Self {
        let mut key = [0u8; 32];
        rand::fill(key.as_mut_slice());
        let key: ChaChaPoly1305<ChaCha20> =
            ChaChaPoly1305::new(&chacha20poly1305::Key::clone_from_slice(&key[..32]));

        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        Self {
            cipher: Arc::new(key),
            nonce: Arc::new(nonce),
        }
    }
}

impl Debug for InternalConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("sensitive data")
    }
}
