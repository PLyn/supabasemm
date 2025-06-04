use crate::shared::models::ProjectConfig;
use crate::shared::server_functions::mgmt_api_get;
use crate::migrate::migrate_page::ConfigItems;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use serde_json::Value;

#[server]
pub async fn generate_preview(
    source_project: String,
    dest_project: String,
) -> Result<Vec<ProjectConfig>, ServerFnError> {
    use super::json_diff;
    use tower_sessions::Session;
    use leptos_axum::extract;

    let session: Session = extract().await?;

    let mut project_config: Vec<ProjectConfig> = Vec::new();
    let mut config_json: Vec<(String, String, String)> = Vec::new();

    let source_config = mgmt_api_get(format!("/projects/{}/config/auth", source_project)).await?;
    let dest_config = mgmt_api_get(format!("/projects/{}/config/auth", dest_project)).await?;
    config_json.push((format!("{:?}", ConfigItems::Auth), source_config, dest_config));

    let source_config = mgmt_api_get(format!("/projects/{}/postgrest", source_project)).await?;
    let dest_config = mgmt_api_get(format!("/projects/{}/postgrest", dest_project)).await?;
    config_json.push((format!("{:?}", ConfigItems::Postgrest), source_config, dest_config));

    let source_config = mgmt_api_get(format!("/projects/{}/functions", source_project)).await?;
    let dest_config = mgmt_api_get(format!("/projects/{}/functions", dest_project)).await?;
    config_json.push((format!("{:?}", ConfigItems::EdgeFunctions), source_config, dest_config));

    let source_config = mgmt_api_get(format!("/projects/{}/secrets", source_project)).await?;
    let dest_config = mgmt_api_get(format!("/projects/{}/secrets", dest_project)).await?;
    config_json.push((format!("{:?}", ConfigItems::Secrets), source_config, dest_config));

    let source_config = mgmt_api_get(format!("/projects/{}/config/database/postgres", source_project)).await?;
    let dest_config = mgmt_api_get(format!("/projects/{}/config/database/postgres",dest_project)).await?;
    config_json.push((format!("{:?}", ConfigItems::Postgres), source_config, dest_config));

    for (config_type, source_json, dest_json) in config_json {
        let source_value: Value = serde_json::from_str(&source_json)?;
        let dest_value: Value = serde_json::from_str(&dest_json)?;

        let new_project_config = json_diff(config_type, source_value, dest_value, project_config).await?;
        
        if let Some(last_config) = new_project_config.last() {
            let last_config_json = &last_config;
            if let Err(e) = session.insert(last_config_json.name.as_str(), last_config.config_json.clone()).await {
                eprintln!("Failed to insert preview results into session: {:?}", e);
            } 
        }
        project_config = new_project_config;
    }
    Ok(project_config)
}
