use std::collections::HashMap;

use crate::catchers::ErrorResponse;
use rocket::serde::json::{Json, Value};
use utoipa::OpenApi;

use crate::providers::common::{ProviderData, ProviderUserData};

pub mod merges;
pub mod playlists;
pub mod user;

#[derive(OpenApi)]
#[openapi(
    paths(user::get, playlists::get_all),
    components(schemas(
        user::User,
        ProviderData,
        ProviderUserData,
        playlists::Playlist,
        ErrorResponse
    ))
)]
struct ApiDoc;

#[get("/openapi")]
pub fn open_api() -> Json<HashMap<String, Value>> {
    Json(
        rocket::serde::json::from_str(ApiDoc::openapi().to_pretty_json().unwrap().as_str())
            .unwrap(),
    )
}
