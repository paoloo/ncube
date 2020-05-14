use async_trait::async_trait;
use ncube_data::{Collection, ConfigSetting, NcubeConfig};
use rusqlite::{self, params, NO_PARAMS};
use serde_rusqlite::{self, from_rows};
use std::fmt::Debug;

use crate::db::sqlite;
use crate::errors::StoreError;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

#[async_trait]
pub(crate) trait ConfigStore {
    type Database;

    async fn init(&mut self, db: &Self::Database) -> Result<(), StoreError>;
    async fn upgrade(&mut self, db: &Self::Database) -> Result<(), StoreError>;
    async fn list_collections(
        &mut self,
        db: &Self::Database,
    ) -> Result<Vec<Collection>, StoreError>;
    async fn is_bootstrapped(&mut self, db: &Self::Database) -> Result<bool, StoreError>;
    async fn show(&mut self, db: &Self::Database) -> Result<NcubeConfig, StoreError>;
    async fn insert(
        &mut self,
        db: &Self::Database,
        name: &str,
        value: &str,
    ) -> Result<(), StoreError>;
}

#[derive(Debug)]
pub struct ConfigSqliteStore;

#[async_trait]
impl ConfigStore for ConfigSqliteStore {
    type Database = sqlite::Database;

    #[tracing::instrument]
    async fn init(&mut self, db: &Self::Database) -> Result<(), StoreError> {
        let mut conn = db.connection().await?;
        conn.pragma_update(None, "foreign_keys", &"ON")?;
        // FIXME: Should I enable this?
        // conn.pragma_update(None, "journal_mode", &"WAL")?;
        Ok(())
    }

    #[tracing::instrument]
    async fn upgrade(&mut self, db: &Self::Database) -> Result<(), StoreError> {
        let mut conn = db.connection().await?;
        // The actual sqlite connection is hidden inside a deadpool Object
        // inside a ClientWrapper. We deref those two levels to make refinery
        // happy.
        embedded::migrations::runner().run(&mut **conn)?;
        Ok(())
    }

    #[tracing::instrument]
    async fn list_collections(
        &mut self,
        db: &Self::Database,
    ) -> Result<Vec<Collection>, StoreError> {
        let conn = db.connection().await?;
        let mut stmt = conn.prepare(include_str!("sql/config/list_collections.sql"))?;

        let collections_iter = from_rows::<Collection>(stmt.query(NO_PARAMS)?);

        let mut collections: Vec<Collection> = vec![];
        for collection in collections_iter {
            collections.push(collection?);
        }

        Ok(collections)
    }

    #[tracing::instrument]
    async fn is_bootstrapped(&mut self, db: &Self::Database) -> Result<bool, StoreError> {
        let conn = db.connection().await?;
        let result: i32 = conn.query_row(
            include_str!("sql/config/is_bootstrapped.sql"),
            NO_PARAMS,
            |row| row.get(0),
        )?;

        if result == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    #[tracing::instrument]
    async fn show(&mut self, db: &Self::Database) -> Result<NcubeConfig, StoreError> {
        let conn = db.connection().await?;
        let mut stmt = conn.prepare(include_str!("sql/config/show.sql"))?;

        let config_iter = from_rows::<ConfigSetting>(stmt.query(NO_PARAMS)?);

        let mut ncube_config: NcubeConfig = vec![];
        for setting in config_iter {
            ncube_config.push(setting?);
        }

        Ok(ncube_config)
    }

    #[tracing::instrument]
    async fn insert(
        &mut self,
        db: &Self::Database,
        name: &str,
        value: &str,
    ) -> Result<(), StoreError> {
        let conn = db.connection().await?;
        let setting_id: i32 = conn.query_row(
            include_str!("sql/config/setting_exists.sql"),
            params![&name],
            |row| row.get(0),
        )?;

        conn.execute(
            include_str!("sql/config/upsert.sql"),
            params![&setting_id, &value],
        )?;

        Ok(())
    }
}
