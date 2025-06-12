use crate::shared::{models::Project, server_functions::check_auth_status};
use leptos::prelude::*;

#[server]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    // Imports
    use super::mgmt_api_get;
    use serde_json::from_str;
    let auth_result = check_auth_status().await;
    if let Ok(is_auth) = auth_result {
        if !is_auth {
            return Ok(Vec::new());
        }
    } else {
        return Err(ServerFnError::ServerError("Error Authenticating".to_string()));   
    }
    

    eprintln!("get projects");
    let text = mgmt_api_get("/projects".to_string()).await?;

    match from_str::<Vec<Project>>(&text) {
        Ok(projects) => Ok(projects),
        Err(e) => Err(ServerFnError::ServerError(format!(
            "Error parsing text: {:?}",
            e
        ))),
    }
}
