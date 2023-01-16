use crate::guards::auth::AuthUser;




// #[utoipa::path(get, path="/playlists" ,responses((status = 200, description =  "All playlists from authenticated user across all connected services", body = User), (status = 403, description = "Unauthorized")))]
#[get("/")]
pub async fn get_all(_user: AuthUser) {

}
