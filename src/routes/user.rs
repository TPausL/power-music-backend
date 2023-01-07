use rocket::serde::json::Json;
use serde::Serialize;

use crate::guards::auth::AuthUser;
use crate::providers::common::{ProviderData, UserData};
use crate::providers::spotify::Spotify;
   


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub providers: Vec<ProviderData>,
}

#[async_trait]
trait UserProviders {
    async fn fetch_provider_data(&self) -> Vec<ProviderData>;
}

#[async_trait]
impl UserProviders for AuthUser {
    async fn fetch_provider_data(&self) -> Vec<ProviderData> {
        let mut provs = Vec::new();
        for t in &self.tokens {
            match t.provider.as_str() {
                "spotify" => {provs.push(ProviderData{ name: String::from("spotify"), user_data: Spotify::new(&self).get_user_data().await});},
                _ => {},
            }
        }
        provs
    }
}


#[get("/")]
pub async fn get(user: AuthUser) -> Json<User> {
    let u = &user;
    Json(User {
        providers: u.fetch_provider_data().await,
        name: user.name,
        email: user.email,
        id: user.id,
    })

}


