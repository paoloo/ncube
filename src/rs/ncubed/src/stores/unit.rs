use async_trait::async_trait;

use ncube_data::{Download, Media, Source, Unit};
use rusqlite::params;
use serde_rusqlite::from_rows;
use tracing::instrument;

use crate::db::{http, sqlite, Database};
use crate::errors::StoreError;

pub(crate) fn unit_store(wrapped_db: Database) -> Box<dyn UnitStore + Send + Sync> {
    match wrapped_db {
        Database::Sqlite(db) => Box::new(UnitStoreSqlite { db }),
        Database::Http(client) => Box::new(UnitStoreHttp { client }),
    }
}

#[async_trait]
pub(crate) trait UnitStore {
    async fn list(&self, page: usize, page_size: usize) -> Result<Vec<Unit>, StoreError>;
}

#[derive(Debug)]
pub struct UnitStoreSqlite {
    db: Box<sqlite::Database>,
}

#[async_trait]
impl UnitStore for UnitStoreSqlite {
    #[instrument]
    async fn list(&self, page: usize, page_size: usize) -> Result<Vec<Unit>, StoreError> {
        let conn = self.db.connection().await?;
        let mut stmt = conn.prepare_cached(include_str!("../sql/unit/paginate.sql"))?;
        let mut stmt2 = conn.prepare_cached(include_str!("../sql/unit/list-media.sql"))?;
        let mut stmt3 = conn.prepare_cached(include_str!("../sql/unit/list-downloads.sql"))?;
        let mut stmt4 = conn.prepare_cached(include_str!("../sql/unit/list-sources.sql"))?;

        let offset = page * page_size;
        let mut units: Vec<Unit> = vec![];

        for unit in from_rows::<Unit>(stmt.query(params![offset as i32, page_size as i32])?) {
            let mut unit = unit?;
            let mut medias: Vec<Media> = vec![];
            let mut downloads: Vec<Download> = vec![];
            let mut sources: Vec<Source> = vec![];

            for media in from_rows::<Media>(stmt2.query(params![unit.id])?) {
                medias.push(media?);
            }

            for download in from_rows::<Download>(stmt3.query(params![unit.id])?) {
                downloads.push(download?);
            }

            for source in from_rows::<Source>(stmt4.query(params![unit.id])?) {
                sources.push(source?);
            }

            unit.media = medias;
            unit.downloads = downloads;
            unit.sources = sources;

            units.push(unit);
        }

        Ok(units)
    }
}

#[derive(Debug)]
pub struct UnitStoreHttp {
    client: Box<http::Database>,
}

#[async_trait]
impl UnitStore for UnitStoreHttp {
    #[instrument]
    async fn list(&self, _page: usize, _page_size: usize) -> Result<Vec<Unit>, StoreError> {
        todo!()
    }
}