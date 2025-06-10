use crate::shared::models::ProjectConfig;
use leptos::prelude::*;

#[server]
pub async fn generate_preview(
    source_project: String,
    dest_project: String,
) -> Result<Vec<ProjectConfig>, ServerFnError> {
    use crate::shared::server_functions::mgmt_api_get;
    use crate::migrate::migrate_page::ConfigItems;

    //server only imports
    use super::json_diff;
    use serde_json::Value;
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

        let project_config_entry = json_diff(config_type.clone(), source_value.clone(), dest_value).await?;
        
        if let Some(config_entry) = project_config_entry {
            project_config.push(config_entry);
        }

        if let Err(e) = session.insert(config_type.as_str(), source_value).await {
            eprintln!("Failed to insert preview results into session: {:?}", e);
        }
    }
    Ok(project_config)
}
