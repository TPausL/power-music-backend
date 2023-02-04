

use rspotify::{clients::*, model::SimplifiedPlaylist, AuthCodeSpotify, Credentials, Token};
use rspotify_macros::scopes;

use crate::{
    db::{playlist::DBPlaylist, CanBeStored},
    guards::auth::AuthUser,
    routes::playlists::{HasPlaylists, Playlist},
};

use rspotify::clients::OAuthClient;

use super::common::{ProviderUserData, UserData};

#[derive(Debug)]
pub struct Spotify {
    //user: &'a AuthUser,
    pub client: AuthCodeSpotify,
    pub id: String,
    pub user_id: String,
}

impl Spotify {
    pub async fn new(user: &AuthUser) -> Spotify {
        let scopes = scopes!(
            "read-user-email",
            "user-read-private",
            "user-top-read",
            "playlist-read-private",
            "playlist-modify-public",
            "playlist-modify-private",
            "playlist-read-collaborative"
        );
        let token = user
            .tokens
            .iter()
            .find(|&t| t.provider == "spotify")
            .unwrap();

        let sp_token = Token {
            access_token: token.value.to_string(),
            refresh_token: Some(token.refresh_token.to_string()),
            scopes: scopes.to_owned(),
            ..Token::default()
        };
        let spt = AuthCodeSpotify::new(
            Credentials::from_env().unwrap(),
            rspotify::OAuth::from_env(scopes.to_owned()).unwrap(),
        );

        *spt.token.lock().await.unwrap() = Some(sp_token.clone());

        spt.refresh_token().await.unwrap();
        Self {
            //user,
            id: spt.me().await.unwrap().id.to_string(),
            client: spt,
            user_id: user.id.to_owned(),
        }
    }
}

#[async_trait]
impl UserData for Spotify {
    async fn get_user_data(&self) -> ProviderUserData {
        //*tok = Some(new_token);
        let me = self.client.me().await;
        match me {
            Ok(u) => ProviderUserData {
                image: u.images.unwrap().first().unwrap().url.to_owned(),
                name: u.display_name.unwrap(),
                email: u.email.unwrap(),
                id: u.id.to_string(),
            },
            Err(..) => ProviderUserData::default(),
        }
    }
}

#[async_trait]
impl HasPlaylists for Spotify {
    async fn get_all_playlists(&self) -> Vec<Playlist> {
        let mut res_lists: Vec<SimplifiedPlaylist> = Vec::new();
        let mut offset = Some(0);
        loop {
            println!("requesttt");
            let req = self
                .client
                .current_user_playlists_manual(Some(50), offset)
                .await;

            let res = match req {
                Err(e) => {
                    println!("{e:#?}");
                    break;
                }
                Ok(d) => d,
            };

            let mut items = res.items.to_owned();
            res_lists.append(&mut items);
            offset = Some(offset.unwrap() + 50);
            if res.offset >= res.total {
                println!("break");
                break;
            }
        }

        let mut lists = Vec::new();
        for l in &res_lists {
            let can_edit = l.collaborative || l.owner.id.to_string() == self.id;
            let id =
                "spotify_".to_owned() + self.user_id.as_str() + "_" + l.id.to_string().as_str();
            let mut list = DBPlaylist {
                hidden: false,
                id: id.to_owned(),
            };
            let hidden = match list.fetch().await {
                Ok(p) => p.hidden,
                Err(_) => match list.store().await {
                    Ok(p) => p.hidden,
                    Err(e) => {
                        dbg!(e);
                        continue;
                    }
                },
            };
            lists.push(Playlist {
                hidden,
                id,
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
