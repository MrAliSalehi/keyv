use tracing::error;

#[derive(serde::Serialize, serde::Deserialize,Debug)]
pub struct ResourcesConfig {
    pub workers_count: usize,
}
impl Default for ResourcesConfig {
    fn default() -> Self {
        Self {
            workers_count: sysinfo::System::physical_core_count().unwrap_or_else(|| {
                error!("could not find the physical cores count, setting workers to 4");
                4
            }),
        }
    }
}
