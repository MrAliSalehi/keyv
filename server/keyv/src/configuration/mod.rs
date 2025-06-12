use crate::configuration::config::Config;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::Arc;

pub mod config;
pub mod conn;
pub mod internal;
pub mod resource;
pub mod storage;

#[derive(Clone)]
pub struct ConfigRef(pub Arc<Config>);

impl ConfigRef {
    pub fn new(config: Config) -> Self {
        Self(Arc::new(config))
    }
}

impl Deref for ConfigRef {
    type Target = Arc<Config>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Debug for ConfigRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
