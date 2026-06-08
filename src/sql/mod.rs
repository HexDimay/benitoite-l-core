use sqlx::{Connection, SqliteConnection};

use crate::config::db::MetadataDataBase;

#[derive(Debug)]
pub struct SqlState {
    sqlite_connection: Option<SqliteConnection>,
}

impl SqlState {
    pub fn new() -> Self {
        Self {
            sqlite_connection: None,
        }
    }

    /// Создание файла базы данных в корневой директории и обновление конфига.
    pub async fn create_database(&self, config: &mut crate::config::db::ConfigDataBase, name_db: &str) -> anyhow::Result<()> {
        async_std::fs::write(format!("./{}.db", name_db), "").await?;
        config.scan_databases().await?;

        Ok(())
    }

    /// Создание файла базы данных в корневой директории и автоматический выбор базы данных (подключение) с обновлением конфигурации.
    pub async fn create_database_and_connect(&mut self, config: &mut crate::config::db::ConfigDataBase, name_db: &str) -> anyhow::Result<()> {
        self.create_database(config, name_db).await?;

        config.select_database(name_db).await?;

        match config.get_current_database() {
            Some(meta_db) => self.connect(meta_db).await?,
            None => {
                #[cfg(feature = "log")]
                log::error!("Failed to connect to the database: {}", name_db)
            }
        }

        Ok(())
    }

    /// Осуществляет соединение с базой данных.
    /// Если SQL уже подключён, он отключается от базы данных и подключается к указанной базе данных.
    pub async fn connect(&mut self, select_db: &MetadataDataBase) -> anyhow::Result<()> {
        if let Some(_) = &self.sqlite_connection {
            self.clouse().await?;
        }

        let connection =
            sqlx::SqliteConnection::connect(&select_db.path().to_string_lossy()).await?;

        #[cfg(feature = "log")]
        log::info!("Connected database: {}", select_db.name());

        self.sqlite_connection = Some(connection);

        Ok(())
    }

    /// Осуществляет выход и закрытие базы данных.
    pub async fn clouse(&mut self) -> anyhow::Result<()> {
        if let Some(c) = std::mem::take(&mut self.sqlite_connection) {
            c.close().await?;

            #[cfg(feature = "log")]
            log::info!("Closing the database connection.");
        }

        Ok(())
    }
}
