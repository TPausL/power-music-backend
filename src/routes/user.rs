use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::guards::auth::AuthUser;
use crate::providers::common::{HasProviders, ProviderData};

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub providers: Vec<ProviderData>,
}

#[utoipa::path(get,operation_id="getAuthUser", path="/user" ,responses((status = 200, description =  "Current user data", body = User), (status = 403, description = "Unauthorized", body = ErrorResponse)))]
#[get("/")]
pub async fn get(user: AuthUser) -> Json<User> {
    let u = &user;
    Json(User {
        providers: u.get_provider_data().await,
        name: user.name,
        email: user.email,
        id: user.id,
    })
}
