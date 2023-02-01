use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::guards::auth::AuthUser;
use crate::providers::common::{ProviderData, UserData};
use crate::providers::spotify::Spotify;

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
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
                "spotify" => {
                    provs.push(ProviderData {
                        name: String::from("spotify"),
                        user_data: Spotify::new(self).await.get_user_data().await,
                    });
                }
                _ => {}
            }
        }
        provs
    }
}

#[utoipa::path(get,operation_id="getAuthUser", path="/user" ,responses((status = 200, description =  "Current user data", body = User), (status = 403, description = "Unauthorized", body = ErrorResponse)))]
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
