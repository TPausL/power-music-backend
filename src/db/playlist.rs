
use serde::{Deserialize, Serialize};

use super::{CanBeStored, DB};

#[derive(Debug, Deserialize, Serialize)]
pub struct DBPlaylist {
    pub id: String,
    pub hidden: bool,
}

#[async_trait]
impl CanBeStored for DBPlaylist {
    const TABLE_NAME: &'static str = "playlist";

    async fn get_containing(_what: String) -> surrealdb::Result<Vec<Self>> {
        let db = DB.get().await;
        let res = db
            .query(format!("select * from {}", Self::TABLE_NAME))
            .await?;
        dbg!(res);
        todo!()
    }
    fn get_id(&self) -> String {
        return self.id.to_owned();
    }
    async fn get(&self) -> surrealdb::Result<Box<Self>> {
        todo!()
    }
    async fn store(&self) -> surrealdb::Result<Box<Self>> {
        todo!()
    }
    async fn update(&self) -> surrealdb::Result<Box<Self>> {
        todo!()
    }
    async fn delete(&self) -> surrealdb::Result<Box<Self>> {
        todo!()
    }
}
