use super::{auth::AuthUser, DataError::*, GuardError};
use crate::db::{playlist::DBPlaylist, CanBeStored, DB};
use rocket::{
    data::{ByteUnit, FromData, Outcome},
    http::Status,
    Data, Request,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MergeData {
    left: String,
    right: String,
    direction: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, CanBeStored)]
pub struct Merge {
    id: String,
    left: String,
    right: String,
    direction: String,
}

impl Merge {
    pub fn from_data(data: MergeData) -> Self {
        Merge {
            id: uuid::Uuid::new_v4().to_string(),
            left: data.left,
            right: data.right,
            direction: data.direction,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for MergeData {
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

        let d: MergeData = match serde_json::from_str::<MergeData>(str_data.as_str()) {
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

        Outcome::Success(MergeData {
            direction: d.direction,
            left: d.left,
            right: d.right,
        })
    }
}
