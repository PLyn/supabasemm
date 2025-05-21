use leptos::prelude::*;

#[server]
pub async fn api_call(url: String) -> Result<String, ServerFnError> {
    // Imports
    use crate::server::api::get_api_call;
    use crate::server::api::handle_response_error;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use serde_json::Value;
    use reqwest::Response; 

    let session: Session = extract().await?;
    let constructed_url = format!("https://api.supabase.com/v1{}", url);
    let api_response = get_api_call(session, constructed_url).await?;

    if api_response.status().is_success() {
        match api_response.text().await { 
            Ok(json_string) => {
                eprintln!("Successfully fetched API response as string.");
                Ok(json_string)
            }
            Err(e) => {
                eprintln!("Error reading response body as text: {:?}", e);
                Err(ServerFnError::ServerError(format!("Error reading response body as text: {:?}", e)))
            }
        }
    } else {
        Err(handle_response_error(api_response).await)
    }
}