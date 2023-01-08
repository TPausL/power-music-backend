use serde::Serialize;
use utoipa::ToSchema;


#[derive(Serialize, Debug,Default, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ProviderUserData {
    pub image: String,
    pub name: String,
    pub email: String,
    pub id: String,
}

#[derive(Serialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ProviderData {
    pub name: String,
    pub user_data: ProviderUserData,
}

#[async_trait]
pub trait UserData {
    async fn get_user_data(&self) -> ProviderUserData;
}
