use hashbrown::HashMap;
use std::sync::Arc;
use keyv_core::engine_specifications::*;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;



#[derive(Clone)]
pub struct Engine<'a> {
    data: Arc<RwLock<HashMap<Key, Val<'a>>>>,
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    pub async fn get(&'a self, key: Key) -> Option<&'a [u8]> {
        let x = self.data.read().await;
        x.get(&key).map(|e| *e)
    }

    pub async fn set(&self, key: Key, value: Val<'a>) {
        let mut lock = self.data.write().await;
        lock.insert(key, value);
        drop(lock)
    }

    pub fn spawn(&self) -> JoinHandle<()> {
        tokio::spawn(async move {})
    }
}
