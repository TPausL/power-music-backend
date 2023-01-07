#![cfg_attr(feature = "strict", deny(warnings))]

use dotenv::dotenv;
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
pub fn rocket() -> _ {
 
    dotenv().ok();
    rocket::build()
        .mount("/", routes![all_options])
        .mount("/user", routes![routes::user::get])
        .register("/", catchers![catchers::forbidden])
        .attach(fairings::cors::CORS)
}



/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

