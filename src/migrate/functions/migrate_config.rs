use crate::shared::models::ProjectConfig;
use leptos::prelude::*;

#[server]
pub async fn migrate_config(
    project_config: Vec<ProjectConfig>,
    dest_project: String,
) -> Result<Vec<ProjectConfig>, ServerFnError> {
    use crate::migrate::migrate_page::ConfigItems;
    use crate::shared::models::AuthConfigStruct;
    use crate::shared::server_functions::mgmt_api_patch;

    //server only imports
    use super::json_diff;
    use leptos_axum::extract;
    use serde_json::Value;
    use tower_sessions::Session;

    eprintln!("Start migrate");
    let session: Session = extract().await?;

    let mut new_project_config = project_config.clone();
    for service in new_project_config.iter_mut() {
        match service.name.as_str() {
            "Auth" => {
                let session_key = format!("{:?}", ConfigItems::Auth);
                let auth_config: Option<String> = session.get(session_key.as_str()).await?;

                if let Some(source_json) = auth_config {
                    let config: AuthConfigStruct = serde_json::from_str(&source_json.clone())?;
                    let patched_config: AuthConfigStruct = config.remove_smtp_fields_if_disabled();
                    let patch_json = serde_json::to_string(&patched_config)?;
                    eprintln!("Patch JSON: {}", patch_json.clone());
                    let response = mgmt_api_patch(
                        format!("/projects/{}/config/auth", dest_project),
                        patch_json.clone(),
                    )
                    .await?;

                    let source_value: Value = serde_json::from_str(&source_json)?;
                    let dest_value: Value = serde_json::from_str(&response)?;
                    let project_config_entry =
                        json_diff(session_key.clone(), source_value, dest_value).await?;

                    if let Some(new_config_entry) = project_config_entry {
                        if new_config_entry.name == service.name {
                            *service = new_config_entry;
                            eprintln!("Removing auth session data");
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
