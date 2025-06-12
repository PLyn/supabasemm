use crate::migrate::migrate_page::ConfigItems;
use crate::shared::models::AuthConfigStruct;
use crate::shared::models::ProjectConfig;
use crate::shared::server_functions::mgmt_api_get;
use crate::shared::server_functions::mgmt_api_patch;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use super::json_diff;
#[cfg(feature = "ssr")]
use serde_json::Value;
#[cfg(feature = "ssr")]
use tower_sessions::Session;

#[server]
pub async fn migrate_config(
    project_config: Vec<ProjectConfig>,
    dest_id: String,
) -> Result<Vec<ProjectConfig>, ServerFnError> {
    //server only imports
    use crate::shared::server_functions::check_auth_status;
    use leptos_axum::extract;
    use tower_sessions::Session;

    let session: Session = extract().await?;
    let is_auth = check_auth_status(session.clone()).await?;
    if !is_auth {
        return Ok(Vec::new());
    }

    let mut new_project_config = project_config;
    for service in new_project_config.iter_mut() {
        match service.name.as_str() {
            "Auth" => {
                migrate_auth(session.clone(), dest_id.clone(), service).await?;
            }
            _ => {}
        }
    }
    Ok(new_project_config)
}

#[cfg(feature = "ssr")]
async fn migrate_auth(
    session: Session,
    dest_id: String,
    service_config: &mut ProjectConfig,
) -> Result<(), ServerFnError> {
    let session_key = format!("{:?}", ConfigItems::Auth);
    let auth_config: Option<String> = session.get(session_key.as_str()).await?;

    if let Some(source_json) = auth_config {
        let config: AuthConfigStruct = serde_json::from_str(&source_json.clone())?;
        let patched_config: AuthConfigStruct = config.remove_smtp_fields_if_disabled();
        let patch_json = serde_json::to_string(&patched_config)?;

        let _response = mgmt_api_patch(
            format!("/projects/{}/config/auth", dest_id),
            patch_json.clone(),
        )
        .await?;

        let dest_new_json = mgmt_api_get(format!("/projects/{}/config/auth", dest_id)).await?;

        let source_value: Value = serde_json::from_str(&source_json)?;
        let dest_value: Value = serde_json::from_str(&dest_new_json)?;
        let project_config_entry = json_diff(session_key.clone(), source_value, dest_value).await?;

        if let Some(new_config_entry) = project_config_entry {
            if new_config_entry.name == service_config.name {
                *service_config = new_config_entry;
                eprintln!("Removing auth session data");
                session.remove::<String>(session_key.as_str()).await.ok();
            }
        }
    }
    Ok(())
}
