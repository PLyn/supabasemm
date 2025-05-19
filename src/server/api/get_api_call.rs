use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::Response;
use tower_sessions::Session;

use super::request_error::ApiRequestError;

pub async fn get_api_call(session: Session, url: String) -> Result<Response, ApiRequestError> {
    let token_option: Option<String> = session // <--- Variable name and type corrected
        .get("supabase_access_token")
        .await
        .map_err(ApiRequestError::SessionError)?;

    if let Some(token) = token_option {
        eprintln!("Access token found in session.");
        let client = reqwest::Client::new();

        eprintln!("Fetching projects from: {}", url);
        let result = client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/json")
            .send()
            .await;

        result.map_err(ApiRequestError::ReqwestError)
    } else {
        Err(ApiRequestError::TokenNotFound)
    }
}
