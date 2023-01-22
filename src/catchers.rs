use rocket::{serde::json::Json, Request};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    message: String,
}

#[catch(403)]
pub fn forbidden(_req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        message: "You are not allowed to access this server!".to_string(),
    })
}
