use crate::guards::auth::AuthUser;
use crate::guards::merge::Merge;









#[post("/", format = "json", data = "<_merge>")]
pub async fn create(_user: AuthUser, _merge: Merge) {
    todo!()
}
