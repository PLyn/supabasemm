use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::Response;
use tower_sessions::session::Error as SessionError;
use tower_sessions::Session; // Import the specific error type

pub async fn api_request(session: Session, url: String) -> Result<Response, String> {
    let token_result: Result<Option<String>, SessionError> =
        session.get("supabase_access_token").await;

    let token_option: Option<String> = match token_result {
        Ok(option) => option, // If session.get was successful, we get the Option<String>
        Err(e) => {
            return Err(format!("Failed to get access token from storage: {}", e));
        }
    };

    if let Some(token) = token_option {
        eprintln!("Access token found in session.");
        let client = reqwest::Client::new();

        eprintln!("Fetching projects from: {}", url);
        let result = client
            .get(&url)
            // Use the unwrapped 'token' (which is a String) directly
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/json")
            .send()
            .await;

        match result {
            Ok(response) => Ok(response),
            Err(e) => Err(format!("API request failed: {}", e)),
        }
    } else {
        Err("Access token not found in session".to_string())
    }
}
