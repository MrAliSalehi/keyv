use crate::configuration::config;

#[derive(serde::Serialize, serde::Deserialize,Debug)]
pub struct StorageConfig {
    pub data_path: String,
    pub max_file_size_bytes: usize,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            data_path: dirs::data_dir()
                .unwrap()
                .join(config::BASE_DIR)
                .join(config::DATA_DIR)
                .to_string_lossy()
                .to_string(),
            max_file_size_bytes: 3145728, //~3MB}
        }
    }
}
