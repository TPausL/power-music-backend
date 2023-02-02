use super::rocket;
use dotenv::dotenv;
use rocket::http::{Cookie, Status};
use rocket::local::asynchronous::Client;

use crate::providers::common::{ProviderData, ProviderUserData};
use crate::routes::{playlists::Playlist, user::User};
#[async_test]
async fn not_found() {
    let client = Client::tracked(rocket().await)
        .await
        .expect("valid rocket instance");
    let res = client.get("/hello").dispatch().await;
    assert_eq!(res.status(), Status::NotFound);
}

#[async_test]
async fn no_auth() {
    let client = Client::tracked(rocket().await)
        .await
        .expect("valid rocket instance");
    let res = client.get("/user").dispatch().await;
    assert_eq!(res.status(), Status::Forbidden);
}

#[async_test]
async fn user() {
    dotenv().ok();
    let token = dotenv::var("ORY_COOKIE").unwrap_or("".to_string());
    let client = Client::tracked(rocket().await)
        .await
        .expect("valid rocket instance");
    let cookie = Cookie::build(
        "ory_session_blissfulborgp2t3dmd959",
        token.as_str().to_owned(),
    )
    .domain("localhost")
    .path("/")
    .finish();
    let res = client.get("/user").cookie(cookie).dispatch().await;
    assert_eq!(
        res.into_json::<User>().await.unwrap(),
        User {
            email: "timoopeters@gmail.com".to_string(),
            id: "fd4fa310-c8ed-48b1-85ac-767f4829c82d".to_string(),
            name: "Timo".to_string(),
            providers: vec![ProviderData {
                name: "spotify".to_string(),
                user_data: ProviderUserData {
                    image: "https://i.scdn.co/image/ab6775700000ee856b697db572b12b13805f226e"
                        .to_string(),
                    name: "Timo".to_string(),
                    email: "timoopeters@gmail.com".to_string(),
                    id: "spotify:user:9dl4j3e3ip4wiazr2xvljimi7".to_string()
                }
            }]
        }
    )
}

#[async_test]
async fn playlist() {
    dotenv().ok();
    let token = dotenv::var("ORY_COOKIE").unwrap_or("".to_string());
    let client = Client::tracked(rocket().await)
        .await
        .expect("valid rocket instance");
    let cookie = Cookie::new(
        "ory_session_blissfulborgp2t3dmd959",
        token.as_str().to_owned(),
    );
    let res = client.get("/playlists").cookie(cookie).dispatch().await;
    let playlist = res
        .into_json::<Vec<Playlist>>()
        .await
        .unwrap()
        .into_iter()
        .find(|x| &x.id.as_str() == &"spotify_spotify:user:9dl4j3e3ip4wiazr2xvljimi7_spotify:playlist:2LhLRnArsSwT3FNPumaXPp")
        .unwrap();
    assert_eq!(
        playlist,
        Playlist {
            id: "spotify_spotify:user:9dl4j3e3ip4wiazr2xvljimi7_spotify:playlist:2LhLRnArsSwT3FNPumaXPp".to_string(),
            title: "PowerMusicTest".to_string(),
            source: "spotify".to_string(),
            link: "https://open.spotify.com/playlist/2LhLRnArsSwT3FNPumaXPp".to_string(),
            count: 0,
            thumbnail: "".to_string(),
            editable: true,
            hidden: false
        }
    )
    //assert!(res.body() as , ));
}
