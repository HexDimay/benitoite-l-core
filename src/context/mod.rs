use crate::{
    config::db::{ConfigDataBase, MetadataDataBase},
    sql::SqlState,
};

/// The `Context' is the central structure for interacting with the backend of the main functionality.
#[derive(Debug)]
pub struct Context {
    pub config_db: ConfigDataBase,
    sql_state: SqlState,
}

impl Context {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            config_db: ConfigDataBase::new().await?,
            sql_state: SqlState::new(),
        })
    }

    /// Создание новой файловой базы данных с обновлением конфига.
    pub async fn create_database(&mut self, name_database: &str) -> anyhow::Result<()> {
        self.sql_state.create_database(name_database).await?;
        self.config_db.scan_databases().await?;

        Ok(())
    }

    /// Создание новой файловой базы данных с обновлением конфига.
    /// С автоматическим подключением.
    pub async fn create_database_and_autoconnect(&mut self, name_database: &str) -> anyhow::Result<()> {
        self.create_database(name_database).await?;
        self.connect_database(name_database).await?;
        Ok(())
    }

    /// Осуществляет подключение к базе данных по имени.
    pub async fn connect_database(&mut self, name_database: &str) -> anyhow::Result<()> {
        self.config_db.select_database(name_database).await?;

        match self.config_db.get_current_database() {
            Some(meta_db) => self.sql_state.connect(meta_db).await?,
            None => {
                #[cfg(feature = "log")]
                log::error!("Failed to connect to the database: {}", name_database)
            }
        }

        Ok(())
    }

    pub fn get_list_databases(&self) -> &[MetadataDataBase] {
        self.config_db.get_list_databases()
    }
}
