use leptos::prelude::*;
#[server]
pub async fn mgmt_api_post_deploy(
    project_ref: String,
    slug: String,
    file_bytes: String, // Now contains TS/JS source code
    entrypoint_path: String,
    import_map_path: Option<String>,
) -> Result<String, ServerFnError> {
    use leptos_axum::extract;
    use reqwest::header::AUTHORIZATION;
    use reqwest::multipart;
    use reqwest::Response;
    use serde_json::json;
    use tower_sessions::Session;

    eprintln!("=== DEBUG: mgmt_api_post_deploy (Corrected) ===");
    eprintln!("Project Ref: {}", project_ref);
    eprintln!("Slug: {}", slug);
    eprintln!("File size: {} bytes", file_bytes.len());
    eprintln!("Entrypoint: {}", entrypoint_path);

    let session: Session = extract().await?;

    // 1. Correct Endpoint: Use the `/deploy` endpoint.
    let constructed_url = format!(
        "https://api.supabase.com/v1/projects/{}/functions/deploy",
        project_ref
    );
    eprintln!("Full URL: {}", constructed_url);

    let api_response: Response;
    let token_option: Option<String> = session.get("supabase_access_token").await?;

    match token_option {
        Some(token) => {
            // 2. Build Metadata: Create the JSON object for the `metadata` form part.
            let metadata = json!({
                "entrypoint_path": entrypoint_path,
                "import_map_path": import_map_path,
                "static_patterns": [] // Required by the API, even if empty
            });
            let metadata_part =
                multipart::Part::text(metadata.to_string()).mime_str("application/json")?;

            // 3. Build File Part: Create the `file` part for the TS/JS source code
            // Determine the file extension based on the entrypoint_path
            let file_extension = if entrypoint_path.ends_with(".ts") {
                "ts"
            } else {
                "js"
            };

            // Use appropriate MIME type for TypeScript or JavaScript
            let mime_type = if file_extension == "ts" {
                "application/typescript"
            } else {
                "application/javascript"
            };

            let file_part = multipart::Part::text(file_bytes)
                .file_name(format!("{}.{}", slug, file_extension))
                .mime_str(mime_type)?;

            // 4. Build Multipart Form: Combine parts into a multipart form.
            let form = multipart::Form::new()
                .part("metadata", metadata_part)
                .part("file", file_part);

            let client = reqwest::Client::new();

            // Add `slug` as a query parameter
            api_response = client
                .post(&constructed_url)
                .query(&[("slug", &slug)])
                .header(AUTHORIZATION, format!("Bearer {}", token))
                // The Content-Type is now set automatically by .multipart()
                .multipart(form)
                .send()
                .await?;

            eprintln!("Response received: Status = {}", api_response.status());
        }
        None => {
            return Err(ServerFnError::ServerError(
                "Access token not found in session".to_string(),
            ))
        }
    }

    if api_response.status().is_success() {
        match api_response.text().await {
            Ok(text) => {
                eprintln!("Success response: {}", text);
                Ok(text)
            }
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
        eprintln!(
            "Error response: Status = {}, Body = {}",
            status_code, error_text
        );
        Err(ServerFnError::ServerError(format!(
            "HTTP request failed with status {}: {}",
            status_code, error_text
        )))
    }
}
