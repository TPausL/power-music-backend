use async_once::AsyncOnce;
use dotenv::dotenv;
use lazy_static::lazy_static;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

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
