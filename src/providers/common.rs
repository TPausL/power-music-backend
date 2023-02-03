use serde::{Deserialize, Serialize};
use utoipa::ToSchema;



use super::spotify::Spotify;


pub enum ProviderClient {
    Spotify(Spotify),
}

#[async_trait]
pub trait HasProviders {
    async fn get_provider_clients(&self) -> Vec<ProviderClient>;
    async fn get_provider_data(&self) -> Vec<ProviderData>;
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
