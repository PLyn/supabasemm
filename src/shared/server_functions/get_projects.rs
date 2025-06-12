use crate::shared::models::Project;
use leptos::prelude::*;

#[server]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    // Imports
    use super::mgmt_api_get;
    use crate::shared::server_functions::check_auth_status;
    use leptos_axum::extract;
    use serde_json::from_str;
    use tower_sessions::Session;

    let session: Session = extract().await?;
    let is_auth = check_auth_status(session).await?;
    if !is_auth {
        return Ok(Vec::new());
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
