use crate::config::db::MetadataDataBase;

#[derive(Debug)]
pub(crate) struct SqlState {
    sqlite_connection: Option<sqlx::sqlite::SqlitePool>,
}

impl SqlState {
    pub(crate) fn new() -> Self {
        Self {
            sqlite_connection: None,
        }
    }

    /// Создание файла базы данных в корневой директории и обновление конфига.
    pub(crate) async fn create_database(&self, name_db: &str) -> anyhow::Result<()> {
        async_std::fs::write(format!("./{}.db", name_db), "").await?;
        Ok(())
    }

    /// Осуществляет соединение с базой данных.
    /// Если SQL уже подключён, он отключается от базы данных и подключается к указанной базе данных.
    pub(crate) async fn connect(&mut self, select_db: &MetadataDataBase) -> anyhow::Result<()> {
        if let Some(_) = &self.sqlite_connection {
            self.close().await?;
        }

        let connection =
            sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&select_db.sql_path())
                .await?;

        #[cfg(feature = "log")]
        log::info!("Connected database: {}", select_db.name());

        self.sqlite_connection = Some(connection);

        Ok(())
    }

    /// Осуществляет выход и закрытие базы данных.
    pub(crate) async fn close(&mut self) -> anyhow::Result<()> {
        if let Some(c) = std::mem::take(&mut self.sqlite_connection) {
            c.close().await;

            #[cfg(feature = "log")]
            log::info!("Closing the database connection.");
        }

        Ok(())
    }
}
