use anyhow::Context;
use async_std::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const PATH_CONFIG: &'static str = "./config_list_database.cfg";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDataBase {
    current_db: Option<MetadataDataBase>,
    list_db: Vec<MetadataDataBase>,
}

impl ConfigDataBase {
    pub async fn new() -> anyhow::Result<Self> {
        #[cfg(feature = "log")]
        log::info!("Init config for ListDataBase");

        match async_std::fs::read_to_string(PATH_CONFIG).await {
            Ok(content) => Ok(serde_json::from_str(&content)?),
            Err(e) if e.kind() == async_std::io::ErrorKind::NotFound => {
                #[cfg(feature = "log")]
                log::warn!("File of cfg is not found");

                let mut config = Self {
                    current_db: None,
                    list_db: Vec::new(),
                };
                async_std::fs::write(PATH_CONFIG, serde_json::to_string(&config)?)
                    .await
                    .context("Failed to write default config")?;

                #[cfg(feature = "log")]
                {
                    log::info!("Creating config for ListDataBase");
                    log::info!("Auto scan db.");
                }

                config.scan_databases().await?;

                Ok(config)
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Getting the current database is given _if it has been selected_.
    pub fn get_current_database(&self) -> Option<&MetadataDataBase> {
        self.current_db.as_ref()
    }

    /// Get the entire list of available databases.
    pub fn get_list_databases(&self) -> &[MetadataDataBase] {
        &self.list_db
    }

    /// Selecting the current database.
    pub async fn select_database(&mut self, name: &str) -> anyhow::Result<()> {
        let path = self
            .list_db
            .iter()
            .find(|&db| db.name == name)
            .ok_or_else(|| anyhow::anyhow!("Database '{}' not found", name))?;
        self.current_db = Some(path.clone());
        self.save_data().await?;
        Ok(())
    }

    /// Scans all database files in the root directory,
    /// and then saves the data to a config file.
    pub async fn scan_databases(&mut self) -> anyhow::Result<()> {
        self.list_db.clear();

        let mut entries = async_std::fs::read_dir("./").await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_file() && entry.path().extension() == Some("db".as_ref())
            {
                let name = entry.file_name().into_string().unwrap_or_default();

                #[cfg(feature = "log")]
                log::info!("Database found: {}", name.clone());

                self.list_db.push(MetadataDataBase::new(
                    name,
                    PathBuf::from(entry.path().as_os_str()),
                ));
            }
        }

        #[cfg(feature = "log")]
        if self.list_db.is_empty() {
            log::info!("No databases were found.");
        }

        self.save_data().await?;
        Ok(())
    }

    /// It should be called whenever the config file is changed, in order to save data for future sessions.
    async fn save_data(&mut self) -> anyhow::Result<()> {
        async_std::fs::write(PATH_CONFIG, serde_json::to_string(self)?).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataDataBase {
    name: String,
    path_db: PathBuf,
}

impl MetadataDataBase {
    pub fn new(name: String, path_db: PathBuf) -> Self {
        Self { name, path_db }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn path(&self) -> &Path {
        &self.path_db
    }
}

#[cfg(test)]
mod test {
    use crate::config::db::ConfigDataBase;

    #[test]
    fn test_create_list_config_db() {
        unsafe { std::env::set_var("RUST_LOG", "info") };
        env_logger::init();
        let res = async_std::task::block_on(async { ConfigDataBase::new().await });

        assert!(res.is_ok());
    }
}
