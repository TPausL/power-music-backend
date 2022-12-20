use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};

use ory_client::apis::configuration::Configuration;
use ory_client::apis::frontend_api::to_session;

use ory_client::apis::identity_api::get_identity;
pub struct Auth {}

#[rocket::async_trait]
impl Fairing for Auth {
    fn info(&self) -> Info {
        Info {
            name: "Authorize user against ORY",
            kind: Kind::Request,
        }
    }

    // Increment the counter for `GET` and `POST` requests.
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let user_cookie = request
            .cookies()
            .get("ory_session_blissfulborgp2t3dmd959")
            .unwrap()
            .to_string();
        let mut config = Configuration::new();
        config.base_path = "http://localhost:4000".to_string();

        // reqwest::Client::builder().https_only(true).build()?;
        let client = reqwest::Client::builder().https_only(true).build().unwrap();
        let mut admin_config = config.clone();
        admin_config.client = client;
        admin_config.bearer_access_token =
            Some(std::env::vars().find(|(k, _v)| k == "ORY_TOKEN").unwrap().1);
        admin_config.base_path =
            "https://blissful-borg-p2t3dmd959.projects.oryapis.com".to_string();
        let s = to_session(&config, Some(""), Some(&user_cookie)).await;
        let id = s.unwrap().identity.id;

        let user = get_identity(&admin_config, &id, Some(vec!["oidc".to_string()])).await;

        print!("{:#?}", user)
    }
}
