mod traits;
mod manage_competitions;
mod secret;
mod base;
mod requests;
mod staging;

pub use traits::*;
pub use manage_competitions::*;
pub use secret::*;
pub use base::*;
pub use requests::*;
pub use staging::*;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub error: String,
    pub error_description: Option<String>,
}

#[derive(Debug)]
pub enum Error {
    ApiError(ApiError),
    ReqwestError(reqwest::Error),
    MissingScope(String),
    Other(String),
}

impl From<ApiError> for Error {
    fn from(value: ApiError) -> Self {
        Error::ApiError(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::ReqwestError(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Other(value)
    }
}
