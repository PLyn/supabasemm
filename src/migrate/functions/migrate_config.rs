//use crate::migrate::migrate_page::ConfigItems;
use crate::shared::models::ProjectConfig;
use crate::shared::server_functions::mgmt_api_patch;
use leptos::prelude::*;
//#[cfg(feature = "ssr")]
//use tower_sessions::Session;

#[server]
pub async fn migrate_config(
    project_config: Vec<ProjectConfig>,
    dest_project: String
) -> Result<String, ServerFnError> {
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

    for service in project_config {
        match service.name.as_str() {
            "Auth" => { 
                let response = mgmt_api_patch(format!("/projects/{}/config/auth", dest_project), service.config_json).await;
                match response {
                    Ok(_) => { 
                        //let access_token = session.get(format!("{:?}", ConfigItems::Auth).as_str()).await?;
                        //response_text = text
                    },
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return Err(e);
                    }
                }
            }
            _ => return Ok("Migration completed successfully".to_string())
        }
    }
    Ok("Migration completed successfully".to_string())
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