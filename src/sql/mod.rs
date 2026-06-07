use sqlx::{Connection, SqliteConnection};

use crate::config::db::MetadataDataBase;

#[derive(Debug)]
pub struct SqlState {
    sqlite_connection: Option<SqliteConnection>,
}

impl SqlState {
    pub async fn connect(&mut self, select_db: MetadataDataBase) -> anyhow::Result<()> {
        if let Some(_) = &self.sqlite_connection {
            self.clouse().await?;
        }

        let connection =
            sqlx::SqliteConnection::connect(&select_db.path().to_string_lossy()).await?;
        self.sqlite_connection = Some(connection);
        Ok(())
    }

    pub async fn clouse(&mut self) -> anyhow::Result<()> {
        if let Some(c) = std::mem::take(&mut self.sqlite_connection) {
            c.close().await?;
            log::info!("");
        }

        Ok(())
    }
}
