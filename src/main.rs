#[macro_use] extern create rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!";
}

#[launch]
fn rocket -> _ {
    rocket::build().mount("/", routes![index])
}
