

use ory_client::apis::{configuration::Configuration, frontend_api::to_session, identity_api::get_identity};
use rocket::{request::{FromRequest, Outcome, Request}, http::{Status}, serde::{Serialize} };





#[derive(Debug)]
pub enum CookieError {
    Missing,
    Invalid
}

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser { 
    type Error = CookieError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
            
        let user_cookie_opt = request
            .cookies()
            .get("ory_session_blissfulborgp2t3dmd959");

        let user_cookie = match user_cookie_opt {
            Some(cookie) => cookie.to_string(),
            None => {
                return Outcome::Failure((Status::Forbidden, CookieError::Missing));
            }
        };
           
        let mut config = Configuration::new();
        config.base_path = "http://localhost:4000".to_string();

        let mut admin_config = Configuration::new();
        admin_config.bearer_access_token =
            Some(std::env::vars().find(|(k, _v)| k == "ORY_TOKEN").unwrap().1);
        admin_config.base_path =
            "https://blissful-borg-p2t3dmd959.projects.oryapis.com".to_string();
        let s = to_session(&config, Some(""), Some(&user_cookie)).await;
        

        let id = match s{
            Ok(session)  => session.identity.id,
            Err(..) => {
                return Outcome::Failure((Status::Forbidden, CookieError::Invalid));
                // "".to_string()
            }
        };
        
        let user = get_identity(&admin_config, &id, Some(vec!["oidc".to_string()])).await.unwrap();
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
            tokens.push(Token {provider: provider_string.to_owned() , value: provider_token.to_owned(), refresh_token: refresh_token.to_owned()})
        };

        return Outcome::Success(AuthUser {id: user.id.to_string(),  email: traits["email"].as_str().unwrap().to_string(), name: traits["name"].as_str().unwrap().to_string(), tokens})


    }
}
