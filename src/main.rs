#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;

mod fairings;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!<!!!!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![index])
        .attach(fairings::auth::Auth {})
}
