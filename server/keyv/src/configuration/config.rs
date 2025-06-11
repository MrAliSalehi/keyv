use crate::configuration::internal::InternalConfig;
use crate::configuration::{
    conn::ConnectionConfig, resource::ResourcesConfig, storage::StorageConfig,
};
use eyre::{OptionExt, eyre};
use tokio::io::AsyncWriteExt;

pub(super) const BASE_DIR: &str = "keyv";
pub(super) const CONFIG_NAME: &str = "config.yaml";
pub(super) const DATA_DIR: &str = "aol";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Config {
    pub log_level: String,
    pub connection: ConnectionConfig,
    pub resources: ResourcesConfig,
    pub storage: StorageConfig,
    #[serde(skip)]
    pub _internal: InternalConfig,
}

impl Config {
    pub async fn load_or_create() -> eyre::Result<Self> {
        let config_dir = dirs::config_dir().ok_or_eyre("failed to find config dir")?;
        if !config_dir.exists() {
            return Err(eyre!("config dir doesnt exist"));
        }
        let c_p = config_dir.join(BASE_DIR).join(CONFIG_NAME);
        let config = if c_p.exists() {
            serde_yaml::from_slice::<Self>(&tokio::fs::read(&c_p).await?)
                .map_err(|e| eyre!("failed to parse config file: {e:?}"))?
        } else {
            println!("creating new config");
            let c = Self::default();
            let content = serde_yaml::to_string(&c)
                .map_err(|e| eyre!("failed to store the config file: {e:?}"))?;
            tokio::fs::create_dir_all(config_dir.join(BASE_DIR)).await?;
            tokio::fs::File::create(c_p)
                .await?
                .write_all(content.as_bytes())
                .await?;
            c
        };
        Ok(config)
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: "info".to_owned(),
            connection: Default::default(),
            resources: Default::default(),
            storage: Default::default(),
            _internal: Default::default(),
        }
    }
}
