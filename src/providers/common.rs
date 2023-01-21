use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::guards::auth::AuthUser;

use super::spotify::Spotify;
use rspotify::clients::OAuthClient;

pub enum Provider {
    Spotify(Spotify),
}

#[async_trait]
pub trait HasProviders {
    async fn get_providers(&self) -> Vec<Provider>;
}

#[async_trait]
impl HasProviders for AuthUser {
    async fn get_providers(&self) -> Vec<Provider> {
        let mut provs: Vec<Provider> = Vec::new();
        for tok in &self.tokens {
            match tok.provider.as_str() {
                "spotify" => provs.push(Provider::Spotify(Spotify::new(self).await)),
                "google" => (),
                _ => (),
            }
        }
        provs
    }
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct ProviderUserData {
    pub image: String,
    pub name: String,
    pub email: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct ProviderData {
    pub name: String,
    pub user_data: ProviderUserData,
}

#[async_trait]
pub trait UserData {
    async fn get_user_data(&self) -> ProviderUserData;
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
