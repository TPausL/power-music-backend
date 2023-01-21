use std::{future::IntoFuture, pin::Pin};

use rocket::serde::json::Json;
use rspotify::{model::SimplifiedPlaylist, prelude::OAuthClient};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    db::DB,
    guards::auth::AuthUser,
    providers::{
        common::{HasProviders, Provider},
        spotify::Spotify,
    },
};

#[derive(Debug, Deserialize, Serialize)]
pub struct DBPlaylist {
    id: String,
    hidden: bool,
}
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
            let db = DB.get().await;
            let id = "spotify_".to_owned() + self.id.as_str() + "_" + l.id.to_string().as_str();
            let statement: Pin<
                Box<
                    dyn std::future::Future<Output = Result<DBPlaylist, surrealdb::Error>>
                        + std::marker::Send
                        + Sync,
                >,
            > = db.select(("playlist", id.to_owned())).into_future();

            let hidden = match statement.await {
                Ok(db_playlist) => db_playlist.hidden,
                Err(_) => {
                    let statement: Pin<
                        Box<
                            dyn std::future::Future<Output = Result<DBPlaylist, surrealdb::Error>>
                                + std::marker::Send
                                + Sync,
                        >,
                    > = db
                        .create(("playlist", id.to_owned()))
                        .content(DBPlaylist { id, hidden: false })
                        .into_future();
                    let _ = statement.await;
                    false
                }
            };
            lists.push(Playlist {
                hidden,
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
