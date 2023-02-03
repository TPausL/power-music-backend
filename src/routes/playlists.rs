

use rocket::serde::json::Json;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::guards::auth::AuthUser;

#[derive(Serialize, Deserialize, Debug, Default, ToSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub source: String,
    pub link: String,
    pub count: u16,
    pub thumbnail: String,
    pub editable: bool,
    pub hidden: bool,
}

#[async_trait]
pub trait HasPlaylists {
    async fn get_all_playlists(&self) -> Vec<Playlist>;
}

#[utoipa::path(get,operation_id="getUserPlaylists", path="/playlists" ,responses((status = 200, description =  "All playlists from authenticated user across all connected services", body = [Playlist]), (status = 403, description = "Unauthorized", body = ErrorResponse)))]
#[get("/")]
pub async fn get_all(user: AuthUser) -> Json<Vec<Playlist>> {
    Json(user.get_all_playlists().await)
}
