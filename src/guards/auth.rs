use dotenv::dotenv;
use ory_client::apis::{
    configuration::Configuration, frontend_api::to_session, identity_api::get_identity,
};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    serde::Serialize,
};

use crate::{
    db::{CanBeStored, InDB},
    providers::{
        common::{HasProviders, ProviderClient, ProviderData, UserData},
        spotify::Spotify,
    },
    routes::playlists::{HasPlaylists, Playlist},
};

use super::{CookieError, GuardError};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
pub struct Token {
    pub provider: String,
    pub value: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
pub struct AuthUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub tokens: Vec<Token>,
}

impl AuthUser {
    pub async fn mine<T: CanBeStored>(&self, what: impl InDB) -> bool {
        match what.get_from_db::<T>().await {
            Ok(d) => d.get_id().contains(&self.id),
            Err(_) => false,
        }
    }
}

#[async_trait]
impl HasProviders for AuthUser {
    async fn get_provider_clients(&self) -> Vec<ProviderClient> {
        let mut provs: Vec<ProviderClient> = Vec::new();
        for tok in &self.tokens {
            match tok.provider.as_str() {
                "spotify" => provs.push(ProviderClient::Spotify(Spotify::new(self).await)),
                "google" => (),
                _ => (),
            }
        }
        provs
    }
    async fn get_provider_data(&self) -> Vec<ProviderData> {
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

#[async_trait]
impl HasPlaylists for AuthUser {
    async fn get_all_playlists(&self) -> Vec<Playlist> {
        let provs = self.get_provider_clients().await;
        let mut lists: Vec<Playlist> = Vec::new();
        for p in &provs {
            lists.extend(match p {
                ProviderClient::Spotify(sp) => sp.get_all_playlists().await,
            })
        }
        lists
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = GuardError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        dotenv().ok();
        let cookie_name = dotenv::var("ORY_COOKIE_NAME")
            .unwrap_or("ory_session_blissfulborgp2t3dmd959".to_string());
        let user_cookie_opt = request.cookies().get(&cookie_name);
        let user_cookie = match user_cookie_opt {
            Some(cookie) => cookie.to_string(),
            None => {
                return Outcome::Failure((
                    Status::Forbidden,
                    GuardError::Cookie(CookieError::Missing),
                ));
            }
        };

        let session_url =
            dotenv::var("ORY_SESSION_URL").unwrap_or("http://localhost:4000".to_string());
        let mut config = Configuration::new();
        config.base_path = session_url;

        let admin_url = dotenv::var("ORY_ADMIN_URL").unwrap_or("http://localhost:4000".to_string());
        let mut admin_config = Configuration::new();
        admin_config.bearer_access_token =
            Some(std::env::vars().find(|(k, _v)| k == "ORY_TOKEN").unwrap().1);
        admin_config.base_path = admin_url;
        let s = to_session(&config, Some(""), Some(&user_cookie)).await;

        let id = match s {
            Ok(session) => session.identity.id,
            Err(..) => {
                return Outcome::Failure((
                    Status::Forbidden,
                    GuardError::Cookie(CookieError::Invalid),
                ));
                // "".to_string()
            }
        };

        let user = get_identity(&admin_config, &id, Some(vec!["oidc".to_string()]))
            .await
            .unwrap();
        // let providers: Vec<serde_json::value::Value>;
        let traits = user.traits.unwrap();
        let credentials = user.credentials.unwrap();
        let providers = credentials["oidc"].config.as_ref().unwrap()["providers"]
            .as_array()
            .unwrap();
        let mut tokens = Vec::new();

        for prov in providers {
            let provider_string = &prov["provider"].as_str().unwrap().to_owned();
            let provider_token = &prov["initial_access_token"].as_str().unwrap().to_owned();
            let refresh_token = &prov["initial_refresh_token"].as_str().unwrap().to_owned();
            tokens.push(Token {
                provider: provider_string.to_owned(),
                value: provider_token.to_owned(),
                refresh_token: refresh_token.to_owned(),
            })
        }

        return Outcome::Success(AuthUser {
            id: user.id,
            email: traits["email"].as_str().unwrap().to_string(),
            name: traits["name"].as_str().unwrap().to_string(),
            tokens,
        });
    }
}
