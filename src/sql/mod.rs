use sqlx::{Connection, SqliteConnection};

use crate::config::db::MetadataDataBase;

#[derive(Debug)]
pub struct SqlState {
    sqlite_connection: Option<SqliteConnection>,
}

impl SqlState {
    /// Осуществляет слединение с базой данных.
    /// Если SQL уже подключён, он отключается от базы данных и подключается к указанной базе данных.
    pub async fn connect(&mut self, select_db: MetadataDataBase) -> anyhow::Result<()> {
        if let Some(_) = &self.sqlite_connection {
            self.clouse().await?;
        }

        let connection =
            sqlx::SqliteConnection::connect(&select_db.path().to_string_lossy()).await?;
        log::info!("Connected database: {}", select_db.name());
        self.sqlite_connection = Some(connection);
        Ok(())
    }

    /// Осуществляет выход и закрытие базы данных.
    pub async fn clouse(&mut self) -> anyhow::Result<()> {
        if let Some(c) = std::mem::take(&mut self.sqlite_connection) {
            c.close().await?;
            log::info!("Closing the database connection.");
        }

        Ok(())
    }
}
