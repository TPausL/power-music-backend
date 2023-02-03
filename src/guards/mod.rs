pub mod auth;
pub mod merge;

#[derive(Debug)]
pub enum DataError {
    Missing,
    Invalid,
}
#[derive(Debug)]
pub enum CookieError {
    Missing,
    Invalid,
}

#[derive(Debug)]
pub enum GuardError {
    Cookie(CookieError),
    Data(DataError),
}
