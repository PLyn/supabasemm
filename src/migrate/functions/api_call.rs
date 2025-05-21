use crate::shared::models::Project;
use leptos::prelude::*;


#[server]
pub async fn api_call(url: String) -> Result<Vec<Project>, ServerFnError> {
    // Imports
    use crate::server::api::get_api_call;
    use crate::server::api::handle_response_error;
    use leptos_axum::extract;
    use tower_sessions::Session;

    let session: Session = extract().await?;
    let constructed_url = format!("https://api.supabase.com/v1{}", url);
    let api_response = get_api_call(session, constructed_url).await?;

    if api_response.status().is_success() {
        match api_response.json::<Vec<Project>>().await {
            Ok(projects) => {
                eprintln!("Successfully parsed projects.{:?}", projects);
                Ok(projects)
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {:?}", e);
                Err(ServerFnError::ServerError(format!("Error parsing JSON: {:?}", e)))
            }
        }
    } else {
        Err(handle_response_error(api_response).await)
    }
}