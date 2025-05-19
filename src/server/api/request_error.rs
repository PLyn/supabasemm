use reqwest::Error as ReqwestError;
use thiserror::Error; // Brings in the #[derive(Error)] macro
use tower_sessions::session::Error as SessionError; // Alias to avoid name collision if needed // Alias

#[derive(Debug, Error)]
pub enum ApiRequestError {
    #[error("Failed to get access token from session: {0}")]
    SessionError(#[from] SessionError), // #[from] automatically creates `From<SessionError>` impl

    #[error("Access token not found in session")]
    TokenNotFound, // A custom logical error

    #[error("HTTP request failed: {0}")]
    ReqwestError(#[from] ReqwestError), // #[from] automatically creates `From<ReqwestError>` impl

                                        // You could even add a variant for non-success HTTP statuses if you wanted
                                        // #[error("API returned non-success status {status}: {text}")]
                                        // ApiStatusError {
                                        //     status: u16,
                                        //     text: String,
                                        // },
}
