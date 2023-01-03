#[macro_use]
extern crate rocket;
extern crate dotenv;


use dotenv::dotenv;
use google_youtube3::YouTube;
use ory_client::apis::{configuration::Configuration, frontend_api::to_session, identity_api::get_identity};
use rocket::{request::{FromRequest, Outcome, Request}, http::{Status, Header}, serde::{Serialize, json::Json}, fairing::{Fairing, Info, Kind}, Response };
use rspotify::{scopes,prelude::*, AuthCodeSpotify, Credentials, OAuth};
use rspotify::Token as SpToken;
use hyper_tls::HttpsConnector;
use hyper::client::Client;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::http::Method;



#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
struct BaseUser {
    name: String,
    email: String,
}

trait GetBaseUser {
    fn get_base_user(&self) -> BaseUser;
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
struct Token {
    provider: String,
    value: String,
    refresh_token: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
struct AuthUser {
    name: String,
    email: String,
    tokens: Vec<Token>, 
}

impl GetBaseUser for AuthUser {
    fn get_base_user(&self) -> BaseUser {
        BaseUser { name: self.name.to_owned(), email: self.email.to_owned() }
    }
}

#[derive(Debug)]
enum CookieError {
    Missing,
    Invalid
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

        return Outcome::Success(AuthUser {email: traits["email"].as_str().unwrap().to_string(), name: traits["name"].as_str().unwrap().to_string(), tokens})


    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
#[derive(Default)]
struct ProviderUserData {
    image: String,
    name: String,
    email: String,
    id: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
struct ProviderData {
    name: String,
    user_data: ProviderUserData,
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    providers: Vec<ProviderData>,
}


#[get("/")]
async fn index(user: AuthUser) -> String {
    "Hello world".to_string()
}

 async fn fetch_provider_data(user: &AuthUser, prov: &str) -> ProviderUserData {
    
    let access_token = &user.tokens.iter().find(|&t| t.provider == prov).unwrap().value;
    let refresh_token = &user.tokens.iter().find(|&t| t.provider == prov).unwrap().refresh_token;

    match prov {
        "spotify" => {
            let token = SpToken {
                access_token: access_token.to_string(),
                refresh_token: Some(refresh_token.to_string()),
                ..SpToken::default()
            };
            let spt = AuthCodeSpotify::new(Credentials::from_env().unwrap() , OAuth::from_env(scopes!("read-user-email")).unwrap() );

             *spt.token.lock().await.unwrap() = Some(token.clone());

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
        "google" => {
            let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
            println!("{:#?}", access_token);
            let yt = YouTube::new(client, access_token.to_owned());
            
            let user = yt.channels().list(&vec![String::from("snippet")]).mine(true).doit().await;
            println!("{:#?}", user);
            ProviderUserData::default()
        }
        _ => {
            ProviderUserData::default()
        }
    }

}



#[get("/user")]
async fn get_user(user: AuthUser) -> Json<User> {
    let mut provs = Vec::new();
    for p in &user.tokens {
        provs.push(ProviderData {name: p.provider.to_owned(), user_data: fetch_provider_data(&user, p.provider.as_str()).await })
    }
    Json(User {
        providers: provs,
        name: user.name,
        email: user.email
    })

}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ErrorResponse {
   message: String 
}


#[catch(403)]
fn forbidden(_req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {message: "You are not allowed to access this server!".to_string()})
}

#[launch]
fn rocket() -> _ {
 
    dotenv().ok();
    rocket::build()
        .mount("/", routes![index, get_user, all_options]).register("/", catchers![forbidden]).attach(CORS)
}



/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:3000"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
