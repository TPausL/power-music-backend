use crate::db::playlist::DBPlaylist;

use super::{auth::AuthUser, DataError::*, GuardError};
use rocket::{
    data::{ByteUnit, FromData, Outcome},
    http::Status,
    Data, Request,
};
use serde::Deserialize;

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
    type Error = GuardError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let str_data = match data.open(ByteUnit::GB).into_string().await {
            Ok(str) => str,
            Err(e) => {
                println!("{e:#?}");
                req.local_cache(|| "No Data was provided!");
                return Outcome::Failure((Status::UnprocessableEntity, GuardError::Data(Missing)));
            }
        };

        let d: Merge = match serde_json::from_str::<Merge>(str_data.as_str()) {
            Ok(d) => d,
            Err(e) => {
                req.local_cache(|| Some(e.to_string()));
                return Outcome::Failure((Status::UnprocessableEntity, GuardError::Data(Invalid)));
            }
        };

        let user = req.guard::<AuthUser>().await.unwrap();
        if !user.mine::<DBPlaylist>(d.left.to_owned()).await {
            req.local_cache(|| Some("Left playlist doesn't exist!".to_string()));
            return Outcome::Failure((Status::UnprocessableEntity, GuardError::Data(Invalid)));
        }
        if !user.mine::<DBPlaylist>(d.right.to_owned()).await {
            req.local_cache(|| Some("Right playlist doesn't exist!".to_string()));
            return Outcome::Failure((Status::UnprocessableEntity, GuardError::Data(Invalid)));
        }

        if &d.left == &d.right {
            req.local_cache(|| Some("You can't sync with yourself!".to_string()));
            return Outcome::Failure((Status::UnprocessableEntity, GuardError::Data(Invalid)));
        }

        if !vec!["left", "right", "both"].contains(&d.direction.as_str()) {
            req.local_cache(|| Some("Direction must be one of 'left,right,both'".to_string()));
            return Outcome::Failure((Status::UnprocessableEntity, GuardError::Data(Invalid)));
        }

        Outcome::Success(Merge {
            direction: d.direction,
            left: d.left,
            right: d.right,
        })
    }
}
