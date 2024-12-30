#[derive(Debug)]
pub enum Error {
    AuthenticationError(String),
    ApiError(String),
    NetworkError(String),
    ParseError(String),
    OtherError(String),
}
