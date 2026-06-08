use crate::{config::db::{ConfigDataBase, MetadataDataBase}, sql::SqlState};

/// The `Context' is the central structure for interacting with the backend of the main functionality.
#[derive(Debug)]
pub struct Context {
    config_db: ConfigDataBase,
    sql_state: SqlState,
}

impl Context {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            config_db: ConfigDataBase::new().await?,
            sql_state: SqlState::new(),
        })
    }

    /// Осуществляет подключение к базе данных.
    pub async fn connect_database(&mut self, name_database: &str) -> anyhow::Result<()> {
        self.config_db.select_database(name_database).await?;

        match self.config_db.get_current_database() {
            Some(meta_db) => self.sql_state.connect(meta_db).await?,
            None => log::error!("Failed to connect to the database: {}", name_database),
        }

        Ok(())
    }

    pub fn get_list_databases(&self) -> &[MetadataDataBase] {
        self.config_db.get_list_databases()
    }
}
