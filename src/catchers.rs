use rocket::{request::Request, serde::json::Json};
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
        Json(ErrorResponse { message, details })
    }
}

#[catch(403)]
pub fn forbidden() -> Json<ErrorResponse> {
    ErrorResponse::new("You are not allowed to use this server".to_string(), None)
}

#[catch(422)]
pub fn unprocessable(req: &Request) -> Json<ErrorResponse> {
    let errors: Option<String> = req.local_cache(|| None).clone();
    ErrorResponse::new("Sorry but some data is wrong".to_string(), errors)
}

#[catch(500)]
pub fn server(req: &Request) -> Json<ErrorResponse> {
    let errors: Option<String> = req.local_cache(|| None).clone();
    ErrorResponse::new("Ooops we messed something up!".to_string(), errors)
}
