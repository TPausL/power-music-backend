pub mod playlist;
use async_once::AsyncOnce;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[async_trait]
pub trait InDB {
    async fn get_from_db<T: CanBeStored>(&self) -> surrealdb::Result<T>;
}

#[async_trait]
pub trait CanBeStored: Send + Sync + for<'a> Deserialize<'a> {
    const TABLE_NAME: &'static str;
    async fn get_containing(what: String) -> surrealdb::Result<Vec<Self>>;
    async fn get(&self) -> surrealdb::Result<Box<Self>>;
    async fn store(&self) -> surrealdb::Result<Box<Self>>;
    async fn update(&self) -> surrealdb::Result<Box<Self>>;
    async fn delete(&self) -> surrealdb::Result<Box<Self>>;
    fn get_id(&self) -> String;
}

#[async_trait]
impl InDB for String {
    async fn get_from_db<T: CanBeStored>(&self) -> surrealdb::Result<T> {
        let db = DB.get().await;
        let res: T = db.select((T::TABLE_NAME, self)).await?;
        Ok(res)
    }
}

lazy_static! {
    pub static ref DB: AsyncOnce<Surreal<Client>> = AsyncOnce::new(async {
        dotenv().ok();
        let namespace = dotenv::var("SURREAL_NAMESPACE").unwrap_or("power_music".to_string());
        let database = dotenv::var("SURREAL_DATABASE").unwrap_or("main".to_string());
        let url = dotenv::var("SURREAL_URL").unwrap_or("memory".to_string());

        let db = Surreal::new::<Ws>(url).await.unwrap();
        let _ = db
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await;
        let _login = db.use_ns(namespace).use_db(database).await;
        db
    });
}
