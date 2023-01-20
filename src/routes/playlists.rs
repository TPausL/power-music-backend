use rocket::serde::json::Json;
use rspotify::{model::SimplifiedPlaylist, prelude::OAuthClient};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    guards::auth::AuthUser,
    providers::{
        common::{HasProviders, Provider},
        spotify::Spotify,
    },
};

#[derive(Serialize, Debug, Default, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct Playlist {
    id: String,
    title: String,
    source: String,
    link: String,
    count: u16,
    thumbnail: String,
    editable: bool,
}

#[async_trait]
pub trait HasPlaylists {
    async fn get_all_playlists(&self) -> Vec<Playlist>;
}

#[async_trait]
impl HasPlaylists for Spotify {
    async fn get_all_playlists(&self) -> Vec<Playlist> {
        let mut res_lists: Vec<SimplifiedPlaylist> = Vec::new();
        let mut offset = Some(0);
        loop {
            let res = self
                .client
                .current_user_playlists_manual(Some(50), offset)
                .await
                .unwrap();
            let mut items = res.items.to_owned();
            res_lists.append(&mut items);
            offset = Some(offset.unwrap() + 50);
            if !(res.offset < res.total) {
                break;
            }
        }

        let mut lists = Vec::new();
        for l in &res_lists {
            let can_edit = l.collaborative || l.owner.id.to_string() == self.id;
            lists.push(Playlist {
                id: l.id.to_string(),
                title: l.name.to_string(),
                link: l.external_urls.get("spotify").unwrap().to_string(),
                source: "spotify".to_string(),
                count: l.tracks.total as u16,
                thumbnail: match l.images.get(0) {
                    Some(i) => i.url.to_string(),
                    _ => "".to_string(),
                },
                editable: can_edit,
            })
        }

        lists
    }
}

#[utoipa::path(get,operation_id="getUserPlaylists", path="/playlists" ,responses((status = 200, description =  "All playlists from authenticated user across all connected services", body = [Playlist]), (status = 403, description = "Unauthorized")))]
#[get("/")]
pub async fn get_all(user: AuthUser) -> Json<Vec<Playlist>> {
    let provs = user.get_providers().await;
    let mut lists: Vec<Playlist> = Vec::new();
    for p in &provs {
        lists.extend(match p {
            Provider::Spotify(sp) => sp.get_all_playlists().await,
        })
    }
    Json(lists)
}
