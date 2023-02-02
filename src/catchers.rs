use rocket::{serde::json::Json, request::Request};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    message: String,
    details: Option<String>,
}

impl ErrorResponse {
    pub fn new(message: String, details: Option<String>) -> Json<ErrorResponse> {
        Json(ErrorResponse {
            message,
            details,
        })
    }
}

#[catch(403)]
pub fn forbidden() -> Json<ErrorResponse> {
    ErrorResponse::new("You are not allowed to use this server".to_string(), None)
}

#[catch(422)]
pub fn unprocessable(req: &Request) -> Json<ErrorResponse> {
    // println!("{:#?}", req);
    let errors: String = req.local_cache(|| None).clone().unwrap();
    ErrorResponse::new("Sorry but some data is wrong".to_string(), Some(errors))
}
