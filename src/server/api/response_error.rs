use leptos::prelude::ServerFnError;
use reqwest::Response; // Make sure ServerFnError is in scope

pub async fn handle_response_error(api_response: Response) -> ServerFnError {
    let status_code = api_response.status().as_u16();
    let error_text = api_response
        .text()
        .await
        .unwrap_or_else(|e| format!("Error reading response body: {}", e)); // More descriptive fallback

    eprintln!(
        "HTTP request failed with status {}: {}",
        status_code, error_text
    );

    ServerFnError::ServerError(format!(
        "HTTP request failed with status {}: {}",
        status_code, error_text
    ))
}
