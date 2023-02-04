
use rocket::serde::json::Json;


use crate::db::CanBeStored;
use crate::guards::auth::AuthUser;
use crate::guards::merge::{Merge, MergeData};

use super::Response;
#[post("/", format = "json", data = "<merge>")]
pub async fn create(_user: AuthUser, merge: MergeData) -> Json<Response<Merge>> {
    Response::new(
        "Succesfully created merge".to_string(),
        Merge::from_data(merge).store().await.unwrap(),
    )
}
