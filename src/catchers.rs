use rocket::{serde::json::Json, Request};
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    message: String,
    errors: Option<Value>,
}

impl ErrorResponse {
    pub fn new(message: &str, fields: Option<Value>) -> Json<ErrorResponse> {
        Json(ErrorResponse {
            message: message.to_string(),
            errors: fields,
        })
    }
}

#[catch(403)]
pub fn forbidden() -> Json<ErrorResponse> {
    ErrorResponse::new("You are not allowed to use this server", None)
}

#[catch(422)]
pub fn unprocessable(req: &Request) -> Json<ErrorResponse> {
    // println!("{:#?}", req);
    let errors: Option<Value> = req.local_cache(|| None).clone();
    ErrorResponse::new("Sorry but some data is wrong", errors)
}
