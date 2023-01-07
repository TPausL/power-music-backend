use serde::Serialize;


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
#[derive(Default)]
pub struct ProviderUserData {
    pub image: String,
    pub name: String,
    pub email: String,
    pub id: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
pub struct ProviderData {
    pub name: String,
    pub user_data: ProviderUserData,
}

#[async_trait]
pub trait UserData {
    async fn get_user_data(&self) -> ProviderUserData;
}
