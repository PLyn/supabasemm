use crate::migrate::migrate_page::CONFIG_ITEM_COUNT;
use crate::shared::models::ProjectConfig;
use crate::shared::server_functions::check_auth_status;
use leptos::prelude::*;

#[server]
pub async fn generate_preview(
    source_project: String,
    dest_project: String,
    config_items_rw: [RwSignal<bool>; CONFIG_ITEM_COUNT],
) -> Result<Vec<ProjectConfig>, ServerFnError> {
    use crate::migrate::migrate_page::ConfigItems;
    use crate::shared::server_functions::mgmt_api_get;

    //server only imports
    use super::json_diff;
    use leptos_axum::extract;
    use serde_json::Value;
    use tower_sessions::Session;

    let session: Session = extract().await?;
    let is_auth = check_auth_status(session.clone()).await?;
    if !is_auth {
        return Ok(Vec::new());
    }

    let mut project_config: Vec<ProjectConfig> = Vec::new();
    let mut config_json: Vec<(String, String, String)> = Vec::new();

    if config_items_rw[ConfigItems::Auth as usize].get() == true {
        let source_config =
            mgmt_api_get(format!("/projects/{}/config/auth", source_project)).await?;
        let dest_config = mgmt_api_get(format!("/projects/{}/config/auth", dest_project)).await?;
        config_json.push((
            format!("{:?}", ConfigItems::Auth),
            source_config,
            dest_config,
        ));
    }

    if config_items_rw[ConfigItems::Postgrest as usize].get() == true {
        let source_config = mgmt_api_get(format!("/projects/{}/postgrest", source_project)).await?;
        let dest_config = mgmt_api_get(format!("/projects/{}/postgrest", dest_project)).await?;
        config_json.push((
            format!("{:?}", ConfigItems::Postgrest),
            source_config,
            dest_config,
        ));
    }

    if config_items_rw[ConfigItems::EdgeFunctions as usize].get() == true {
        let source_config = mgmt_api_get(format!("/projects/{}/functions", source_project)).await?;
        let dest_config = mgmt_api_get(format!("/projects/{}/functions", dest_project)).await?;
        config_json.push((
            format!("{:?}", ConfigItems::EdgeFunctions),
            source_config,
            dest_config,
        ));
    }

    if config_items_rw[ConfigItems::Secrets as usize].get() == true {
        let source_config = mgmt_api_get(format!("/projects/{}/secrets", source_project)).await?;
        let dest_config = mgmt_api_get(format!("/projects/{}/secrets", dest_project)).await?;
        config_json.push((
            format!("{:?}", ConfigItems::Secrets),
            source_config,
            dest_config,
        ));
    }

    if config_items_rw[ConfigItems::Postgres as usize].get() == true {
        let source_config = mgmt_api_get(format!(
            "/projects/{}/config/database/postgres",
            source_project
        ))
        .await?;
        let dest_config = mgmt_api_get(format!(
            "/projects/{}/config/database/postgres",
            dest_project
        ))
        .await?;

        eprintln!("postgres src: {} || dest: {}", source_config, dest_config);
        config_json.push((
            format!("{:?}", ConfigItems::Postgres),
            source_config,
            dest_config,
        ));
    }

    for (service, source_json, dest_json) in config_json {
        let source: Value = serde_json::from_str(&source_json)?;
        let dest: Value = serde_json::from_str(&dest_json)?;

        let project_config_entry = json_diff(service.clone(), source.clone(), dest).await?;

        if let Some(config_entry) = project_config_entry {
            project_config.push(config_entry);
        }

        if let Err(e) = session.insert(service.as_str(), source).await {
            eprintln!("Failed to insert preview results into session: {:?}", e);
        }
    }
    Ok(project_config)
}
