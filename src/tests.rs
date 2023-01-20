use super::rocket;
use rocket::http::Status;
use rocket::local::asynchronous::Client;

#[async_test]
async fn not_found() {
    let client = Client::tracked(rocket().await)
        .await
        .expect("valid rocket instance");
    let res = client.get("/hello").dispatch().await;
    assert_eq!(res.status(), Status::NotFound);
}

#[async_test]
async fn no_auth() {
    let client = Client::tracked(rocket().await)
        .await
        .expect("valid rocket instance");
    let res = client.get("/user").dispatch().await;
    assert_eq!(res.status(), Status::Forbidden);
}
