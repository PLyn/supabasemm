use crate::shared::models::ProjectConfig;
use crate::shared::server_functions::mgmt_api_patch;
use leptos::prelude::*;

#[server]
pub async fn migrate_config(
    project_config: Vec<ProjectConfig>,
    dest_project: String
) -> Result<String, ServerFnError> {
    for service in project_config {
        eprintln!("{:?}", service.name);
        match service.name.as_str() {
            "Auth" => { 
                let response = mgmt_api_patch(format!("/projects/{}/config/auth", dest_project), service.config_json).await;
                match response {
                    Ok(_) => return Ok("Migration completed successfully".to_string()),
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