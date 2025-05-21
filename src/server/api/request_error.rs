use reqwest::Error as ReqwestError;
use thiserror::Error;
use tower_sessions::session::Error as SessionError; 

#[derive(Debug, Error)]
pub enum ApiRequestError {
    #[error("Failed to get access token from session: {0}")]
    SessionError(#[from] SessionError),

    #[error("Access token not found in session")]
    TokenNotFound,

    #[error("HTTP request failed: {0}")]
    ReqwestError(#[from] ReqwestError), 
}
