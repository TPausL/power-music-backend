use rocket::{
    data::{ByteUnit, FromData, Outcome},
    http::Status,
    request::Request,
    Data,
};
use serde::Deserialize;
use serde_json;

use crate::guards::auth::AuthUser;

use super::DataError;
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct MergeData {
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
impl<'r> FromData<'r> for MergeData {
    type Error = DataError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let str_data = match data.open(ByteUnit::GB).into_string().await {
            Ok(str) => str,
            Err(_) => {
                req.local_cache(|| MergeDataError {
                    general: Some("No data was provided!".to_string()),
                    direction: None,
                    left: None,
                    right: None,
                });
                return Outcome::Failure((
                    Status::UnprocessableEntity,
                    DataError::Missing("no data".to_string()),
                ));
            }
        };

        let d: MergeData = match serde_json::from_str::<MergeData>(str_data.as_str()) {
            Ok(d) => d,
            Err(_e) => {
                return Outcome::Failure((
                    Status::UnprocessableEntity,
                    DataError::Missing("some data is missing".to_string()),
                ))
            }
        };
        println!("{:#?}", d);
        rocket::outcome::Outcome::Failure((
            Status::UnprocessableEntity,
            DataError::InvalidItem("left is not a string".to_string()),
        ))
    }
}

#[post("/", format = "json", data = "<_merge_data>")]
pub async fn create(_user: AuthUser, _merge_data: MergeData) {
    todo!()
}
