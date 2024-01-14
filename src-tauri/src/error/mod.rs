pub mod api;
pub mod bad_request;
pub mod common;

//to be renovated
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    BadRequest(#[from] bad_request::BadRequest),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("{0}")]
    Http(#[from] common::http::HttpError),

    #[error("Sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Jwt error: {0}")]
    Jwt(#[from] common::jwt::JwtError),

    #[error("Parse error: {0}")]
    Parse(#[from] common::parse::ParseError),

    // sqlite
    #[error("Database error: {0}")]
    Database(#[from] common::database::DatabaseError),

    #[error("Query table list failed: {0}")]
    QueryTableListFailed(String),

    // node
    #[error("Command channel send failed: {0}")]
    CommandChannelSendFailed(String),
}
