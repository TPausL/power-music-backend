use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;


#[test]
fn not_found(){
    
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let res = client.get("/").dispatch();
    assert_eq!(res.status(), Status::NotFound);
}

#[test]
fn no_auth(){
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let res = client.get("/user").dispatch();
    assert_eq!(res.status(), Status::Forbidden);
}
