use crate::migrate::migrate_page::ConfigItems;
use crate::shared::models::ProjectConfig;
use crate::shared::server_functions::mgmt_api_patch;
use leptos::prelude::*;

#[server]
pub async fn migrate_config(
    project_config: Vec<ProjectConfig>,
    dest_project: String
) -> Result<Vec<ProjectConfig>, ServerFnError> {
    use super::json_diff;
    use serde_json::Value;
    use tower_sessions::Session;
    use leptos_axum::extract;

    let session: Session = extract().await?;
    let auth_session_data_option: Option<String> = session.get("Auth").await?;

    let auth_session_data = match auth_session_data_option {
        Some(data) => data,
        None => {
            return Err(ServerFnError::ServerError(
                "No session data found for Auth service".to_string(),
            ))
        }
    };
    eprintln!("auth session data length: {}", auth_session_data.len());

    //let mut response_text: String;
    let mut new_project_config = project_config.clone();
    for service in new_project_config.iter_mut() {
        match service.name.as_str() {
            "Auth" => { 
                let session_key = format!("{:?}", ConfigItems::Auth);
                let response = mgmt_api_patch(format!("/projects/{}/config/auth", dest_project), service.config_json.clone()).await?;
                let auth_session_data: Option<Value> = session.get(session_key.as_str()).await?;

                if let Some(session_config) = auth_session_data {
                    let dest_value: Value = serde_json::from_str(&response)?;
                    let project_config_entry = json_diff(session_key.clone(), session_config.clone(), dest_value).await?;
                    
                    if let Some(new_config_entry) = project_config_entry {
                        if new_config_entry.name == service.name {
                            *service = new_config_entry.clone();
                            eprintln!("Removing auth session data: {:?}", session_config);
                            session.remove::<String>(session_key.as_str()).await.ok();
                        }
                    }     
                }
            }
            _ => {}
        }
    }
    Ok(new_project_config)
}

/* #[cfg(feature = "ssr")]
async fn compare_preview_with_results(session: Session, result: String) -> Vec<DiffEntry> {
    let mut diff_entries: Vec<DiffEntry> = Vec::new();
    let access_token = session.get("supabase_access_token").await;

    let access_token = match access_token {
        Some(token) => token,
        None => {
            return Err(ServerFnError::ServerError(
                "No access token found in session".to_string(),
            ))
        }
    };

    diff_entries
}
 */