use leptos::prelude::*;

#[server]
pub async fn mgmt_api_get_bytes(url: String) -> Result<Vec<u8>, ServerFnError> {
    use leptos_axum::extract;
    use reqwest::header::{ACCEPT, AUTHORIZATION};
    use reqwest::Response;
    use tower_sessions::Session;

    let session: Session = extract().await?;
    let constructed_url = format!("https://api.supabase.com/v1{}", url);
    let api_response: Response;
    let token_option: Option<String> = session.get("supabase_access_token").await?;

    match token_option {
        Some(token) => {
            let client = reqwest::Client::new();
            api_response = client
                .get(&constructed_url)
                .header(AUTHORIZATION, format!("Bearer {}", token))
                .header(ACCEPT, "application/vnd.deno.land.eszip")
                .send()
                .await?;
        }
        None => {
            return Err(ServerFnError::ServerError(format!(
                "Access token not found in session"
            )))
        }
    }

    if api_response.status().is_success() {
        match api_response.bytes().await {
            Ok(bytes) => Ok(bytes.to_vec()),
            Err(e) => Err(ServerFnError::ServerError(format!(
                "Error reading response body as bytes: {:?}",
                e
            ))),
        }
    } else {
        let status_code = api_response.status().as_u16();
        let error_text = api_response
            .text()
            .await
            .unwrap_or_else(|e| format!("Error reading response body: {}", e));
        Err(ServerFnError::ServerError(format!(
            "HTTP request failed with status {}: {}",
            status_code, error_text
        )))
    }
}

#[server]
pub async fn mgmt_api_patch_bytes(url: String, body: Vec<u8>) -> Result<String, ServerFnError> {
    use leptos_axum::extract;
    use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
    use reqwest::Response;
    use tower_sessions::Session;

    eprintln!("=== DEBUG: mgmt_api_patch_bytes_debug ===");
    eprintln!("URL: {}", url);
    eprintln!("Body size: {} bytes", body.len());
    eprintln!("First 100 bytes: {:?}", &body[..body.len().min(100)]);

    let session: Session = extract().await?;
    let constructed_url = format!("https://api.supabase.com/v1{}", url);
    let api_response: Response;
    let token_option: Option<String> = session.get("supabase_access_token").await?;

    eprintln!("Full URL: {}", constructed_url);

    match token_option {
        Some(token) => {
            let client = reqwest::Client::new();
            api_response = client
                .patch(&constructed_url)
                .header(AUTHORIZATION, format!("Bearer {}", token))
                .header(ACCEPT, "application/json")
                .header(CONTENT_TYPE, "application/vnd.deno.land.eszip")
                .body(body)
                .send()
                .await?;
            eprintln!("Response received: Status = {}", api_response.status());
        }
        None => {
            return Err(ServerFnError::ServerError(format!(
                "Access token not found in session"
            )))
        }
    }

    if api_response.status().is_success() {
        match api_response.text().await {
            Ok(text) => Ok(text),
            Err(e) => Err(ServerFnError::ServerError(format!(
                "Error reading response body as text: {:?}",
                e
            ))),
        }
    } else {
        let status_code = api_response.status().as_u16();
        let error_text = api_response
            .text()
            .await
            .unwrap_or_else(|e| format!("Error reading response body: {}", e));
        Err(ServerFnError::ServerError(format!(
            "HTTP request failed with status {}: {}",
            status_code, error_text
        )))
    }
}
