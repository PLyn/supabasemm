use crate::shared::models::Project;
use leptos::prelude::*;

#[server]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    // Imports
    use super::mgmt_api_call;

    let text = mgmt_api_call("/projects".to_string()).await?;

    match serde_json::from_str::<Vec<Project>>(&text) {
        Ok(projects) => {
            Ok(projects)
        }
        Err(e) => {
            Err(ServerFnError::ServerError(format!("Error parsing text: {:?}", e)))
        }
    }
}