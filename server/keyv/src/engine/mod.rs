use hashbrown::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

pub type Key<'a> = &'a [u8];
pub type Val<'a> = &'a [u8];

#[derive(Clone)]
pub struct Engine<'a> {
    data: Arc<RwLock<HashMap<Key<'a>, Val<'a>>>>,
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    pub async fn get(&'a self, key: Key<'a>) -> Option<&'a [u8]> {
        let x = self.data.read().await;
        x.get(key).map(|e| *e)
    }

    pub async fn set(&self, key: Key<'a>, value: Key<'a>) {
        let mut lock = self.data.write().await;
        lock.insert(key, value);
        drop(lock)
    }

    pub fn spawn(&self) -> JoinHandle<()> {
        tokio::spawn(async move {})
    }
}
