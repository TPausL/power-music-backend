use rocket::{Request, serde::json::Json};
use serde::Serialize;


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
 pub struct ErrorResponse {
   message: String 
}

#[catch(403)]
pub fn forbidden(_req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {message: "You are not allowed to access this server!".to_string()})
}
