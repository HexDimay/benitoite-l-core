use crate::config::db::{ConfigDataBase, MetadataDataBase};

/// The `Context' is the central structure for interacting with the backend of the main functionality.
#[derive(Debug)]
pub struct Context {
    config_db: ConfigDataBase,
}

impl Context {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            config_db: ConfigDataBase::new().await?,
        })
    }

    pub fn connect_database(&mut self, name_database: &str) {
        
    }

    pub fn get_list_databases(&self) -> &[MetadataDataBase] {
        self.config_db.get_list_databases()
    }
}
