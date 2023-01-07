use power_music_backend;
use rocket;

#[allow(unused_must_use)] 
#[rocket::main]
async fn main() -> () {
    power_music_backend::rocket().launch().await.unwrap_or_else(|_| panic!("rocket server failed to start!"));
}
