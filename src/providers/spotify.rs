
use rspotify::{Token, AuthCodeSpotify, Credentials, clients::*};
use rspotify_macros::scopes;

use crate::guards::auth::AuthUser;

use super::common::{UserData, ProviderUserData};



#[derive(Debug)]
pub struct Spotify<'a> {
    user: &'a AuthUser,
}

impl<'a> Spotify<'a> {
    pub fn new(user: &'a AuthUser) -> Spotify {
        Self { user }
    }
}


#[async_trait]
impl UserData for Spotify<'_> {
    async fn get_user_data(&self) -> ProviderUserData {
        let scopes = scopes!("read-user-email", "user-read-private", "user-top-read", "playlist-read-private",  "playlist-modify-public", "playlist-modify-private", "playlist-read-collaborative");
        let token = &self.user.tokens.iter().find(|&t| t.provider == "spotify").unwrap();
        

            let sp_token = Token {
                access_token: token.value.to_string(),
                refresh_token: Some(token.refresh_token.to_string()),
                scopes:scopes.to_owned(),
                ..Token::default()
            };
            let spt = AuthCodeSpotify::new(Credentials::from_env().unwrap() , rspotify::OAuth::from_env(scopes.to_owned()).unwrap() );

            *spt.token.lock().await.unwrap() = Some(sp_token.clone());

            spt.refresh_token().await.unwrap();
            //*tok = Some(new_token);
            let me = spt.me().await;
            match me {
                Ok(u) =>  {
                    ProviderUserData { image: u.images.unwrap().first().unwrap().url.to_owned() , name: u.display_name.unwrap() , email: u.email.unwrap(), id: u.id.to_string() }
                },
                Err(..) => ProviderUserData::default(),
            }
    } 
}

