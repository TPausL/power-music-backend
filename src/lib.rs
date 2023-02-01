#![cfg_attr(feature = "strict", deny(warnings))]

use dotenv::dotenv;

#[doc(inline)]
pub use std;

#[cfg(test)]
mod tests;
#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate lazy_static;

mod catchers;
mod db;
mod fairings;
mod guards;
mod providers;
mod routes;

#[launch]
pub async fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![all_options, test, routes::open_api])
        .mount("/user", routes![routes::user::get])
        .mount("/playlists", routes![routes::playlists::get_all])
        .mount("/merges", routes![routes::merges::create])
        .register("/", catchers![catchers::forbidden, catchers::unprocessable])
        .attach(fairings::cors::CORS)
}

#[get("/")]
async fn test() {}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}
