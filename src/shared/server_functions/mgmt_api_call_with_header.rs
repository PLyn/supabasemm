use leptos::prelude::*;

#[server]
pub async fn mgmt_api_call_with_header(url: String, header: String) -> Result<String, ServerFnError> {
    use leptos_axum::extract;
    use tower_sessions::Session;
    use reqwest::Response;
    use reqwest::header::{AUTHORIZATION, HeaderValue};

    let session: Session = extract().await?;
    let constructed_url = format!("https://api.supabase.com/v1{}", url);
    let api_response: Response;
    let token_option: Option<String> = session.get("supabase_access_token").await?;

    match token_option {
        Some(_token) => {
            let client = reqwest::Client::new();

            api_response = client
                        .get(&constructed_url)
                        .header(AUTHORIZATION, HeaderValue::from_str(&header)?)
                        .send()
                        .await?;
        }
        None => return Err(ServerFnError::ServerError(format!("Access token not found in session")))
    }

    if api_response.status().is_success() {
        match api_response.text().await { 
            Ok(text) => {
                Ok(text)
            }
            Err(e) => {
                Err(ServerFnError::ServerError(format!("Error reading response body as text: {:?}", e)))
            }
        }
    } else {
        let status_code = api_response.status().as_u16();
        let error_text = api_response
            .text()
            .await
            .unwrap_or_else(|e| format!("Error reading response body: {}", e)); 

        Err(ServerFnError::ServerError(format!("HTTP request failed with status {}: {}", status_code, error_text)))
    }
}