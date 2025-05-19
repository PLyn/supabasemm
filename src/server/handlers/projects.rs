use crate::server::api::call_api;
use crate::server::server_models::AppState;
use crate::shared::models::Project;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use reqwest::StatusCode;
use tower_sessions::Session;
//const CACHE_QUERY: &str = include_str!("cache.sql");

pub async fn projects_handler(
    State(app_state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    //println!("Embedded SQL query:\n{}", CACHE_QUERY);

    let url = format!("{}/projects", app_state.config.mgmt_api_base_url);
    let api_response = call_api(session, url).await;

    if let Ok(response) = api_response {
        if response.status().is_success() {
            match response.json::<Vec<Project>>().await {
                Ok(projects) => {
                    let project_list_html = projects
                        .iter()
                        .map(|p| format!("<li>ID: {}, Name: {}</li>", p.id, p.name))
                        .collect::<String>();
                    (StatusCode::OK, Html(format!("<h1>Your Supabase Projects</h1><ul>{}</ul><p><a href=\"/connect-supabase\">Back to Home</a></p>", project_list_html))).into_response()
                }
                Err(e) => {
                    eprintln!("Failed to parse projects JSON: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("<p>Error parsing project data: {}.</p><p><a href=\"/connect-supabase\">Back to Home</a></p>", e))).into_response()
                }
            }
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Could not read error body".to_string());
            eprintln!("Failed to fetch projects (HTTP {}): {}", status, error_text);
            (status, Html(format!("<p>Error fetching projects: HTTP {} - {}.</p><p><a href=\"/connect-supabase\">Back to Home</a></p>", status, error_text))).into_response()
        }
    } else {
        let e = api_response.unwrap_err(); // We know it's Err here
        eprintln!("Error from api_request: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("<p>Error accessing Supabase API: {}.</p><p><a href=\"/connect-supabase\">Back to Home</a></p>", e))).into_response()
    }
}
