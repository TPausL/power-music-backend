#![cfg_attr(feature = "strict", deny(warnings))]

use dotenv::dotenv;
use rocket::State;

use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};

#[cfg(test)]
mod tests;
#[macro_use]
extern crate rocket;
extern crate dotenv;

mod routes;
mod guards;
mod fairings;
mod catchers;
mod providers;


#[launch]
pub async fn rocket() -> _ {
 
    dotenv().ok();
    let namespace = dotenv::var("SURREAL_NAMESPACE").unwrap_or("power_music".to_string());
    let database = dotenv::var("SURREAL_DATABASE").unwrap_or("main".to_string());
    let url = dotenv::var("SURREAL_URL").unwrap_or("memory".to_string());
  
    let db  = Surreal::new::<Ws>(url).await.unwrap();
    let _ = db.signin(Root {username: "root", password:"root"}).await;
    let _login =  db.use_ns(namespace).use_db(database).await;

    rocket::build()
       .manage(db)
        .mount("/", routes![all_options,test, routes::open_api])
        .mount("/user", routes![routes::user::get])
        .register("/", catchers![catchers::forbidden])
        .attach(fairings::cors::CORS)
}


#[get("/")]
async fn test(db: &State<Surreal<Client>>){
    let res = db.query("SELECT * FROM article;").await.unwrap();
    println!("{:#?}", res);
}


/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

