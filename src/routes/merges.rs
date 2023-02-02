use rocket::{
    data::{ByteUnit, FromData, Outcome},
    http::Status,
    request::Request,
    Data,
};
use serde::Deserialize;
use serde_json;
use crate::db::CheckDB;
use crate::guards::auth::AuthUser;
use crate::routes::playlists::HasPlaylists;
use super::{DataError, playlists::DBPlaylist};
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Merge {
    left: String,
    right: String,
    direction: String,
}

#[allow(dead_code)]
pub struct MergeDataError {
    general: Option<String>,
    left: Option<String>,
    right: Option<String>,
    direction: Option<String>,
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Merge {
    type Error = DataError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let str_data = match data.open(ByteUnit::GB).into_string().await {
            Ok(str) => str,
            Err(e) => {
                println!("{e:#?}");
                req.local_cache(|| "No Data was provided!");
                return Outcome::Failure((
                    Status::UnprocessableEntity,
                    DataError::Missing("no data".to_string()),
                ));
            }
        };

        let d: Merge = match serde_json::from_str::<Merge>(str_data.as_str()) {
            Ok(d) => d,
            Err(e) => {
                req.local_cache(|| Some(e.to_string()));
                return Outcome::Failure((
                    Status::UnprocessableEntity,
                    DataError::Missing("some data is missing".to_string()),
                ))
            }
        };
        

        let user = req.guard::<AuthUser>().await.unwrap();
        

        match d.left.is_in_db::<DBPlaylist>().await {
            Ok(list) => {
                match user.get_all_playlists().await.into_iter().find(|p| p.id == list.id) {
                    Some(_) => (),
                    None => {
                req.local_cache(|| Some("Left playlist doesn't exist!".to_string()));
                return Outcome::Failure((
                    Status::UnprocessableEntity,
                    DataError::Missing("some data is missing".to_string()),
                ))
            }
                }
            },
            Err(_e) => {
                req.local_cache(|| Some("left playlist doesn't exits".to_string()));
                return Outcome::Failure((
                    Status::UnprocessableEntity,
                    DataError::Missing("some data is missing".to_string()),
                ))
            }
        }
        rocket::outcome::Outcome::Failure((
            Status::UnprocessableEntity,
            DataError::InvalidItem("left is not a string".to_string()),
        ))
    }
}

#[post("/", format = "json", data = "<_merge_data>")]
pub async fn create(_user: AuthUser, _merge_data: Merge) {
    todo!()
}
