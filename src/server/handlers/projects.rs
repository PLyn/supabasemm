use crate::server::server_models::{AppState, SupabaseProject};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use reqwest::header::{ACCEPT, AUTHORIZATION};
use tower_sessions::Session;

//const CACHE_QUERY: &str = include_str!("cache.sql");

// GET /connect-supabase/projects (protected - requires token)
pub async fn projects_handler(
    State(app_state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    //println!("Embedded SQL query:\n{}", CACHE_QUERY);

    eprintln!("Projects handler called");
    let token_option: Option<String> = session
        .get("supabase_access_token")
        .await
        .expect("Failed to get access token from session");

    if let Some(token) = token_option {
        eprintln!("Access token found in session.");
        let client = reqwest::Client::new();
        let url = format!("{}/projects", app_state.config.mgmt_api_base_url);

        eprintln!("Fetching projects from: {}", url);
        let result = client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/json")
            .send()
            .await;

        match result {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<SupabaseProject>>().await {
                        Ok(projects) => {
                            let project_list_html = projects
                                .iter()
                                .map(|p| format!("<li>ID: {}, Name: {}</li>", p.id, p.name))
                                .collect::<String>();
                            Html(format!("<h1>Your Supabase Projects</h1><ul>{}</ul><p><a href=\"/connect-supabase\">Back to Home</a></p>", project_list_html))
                        }
                        Err(e) => {
                            eprintln!("Failed to parse projects JSON: {}", e);
                            Html(format!("<p>Error parsing project data: {}.</p><p><a href=\"/connect-supabase\">Back to Home</a></p>", e))
                        }
                    }
                } else {
                    let status = response.status();
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Could not read error body".to_string());
                    eprintln!("Failed to fetch projects (HTTP {}): {}", status, error_text);
                    Html(format!("<p>Error fetching projects: HTTP {} - {}.</p><p><a href=\"/connect-supabase\">Back to Home</a></p>", status, error_text))
                }
            }
            Err(e) => {
                eprintln!("Network error while fetching projects: {}", e);
                Html(format!("<p>Network error fetching projects: {}.</p><p><a href=\"/connect-supabase\">Back to Home</a></p>", e))
            }
        }
    } else {
        eprintln!("No access token found in session. Redirecting to login.");
        // Return HTML with a redirect instead of using Redirect
        Html(format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta http-equiv="refresh" content="0;url=/connect-supabase/login">
                <title>Redirecting to login...</title>
            </head>
            <body>
                <p>No access token found. Redirecting to login page...</p>
                <p>If you are not redirected, <a href="/connect-supabase/login">click here</a>.</p>
            </body>
            </html>
            "#
        ))
    }
}
