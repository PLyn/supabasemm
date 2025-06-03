use crate::shared::models::{DiffEntry, ProjectConfig};
use crate::shared::server_functions::mgmt_api_call;
use leptos::prelude::*;

#[server]
pub async fn generate_preview(
    source_project_rw: String,
    dest_project_rw: String,
) -> Result<Vec<ProjectConfig>, ServerFnError> {
    use json_structural_diff::JsonDiff;
    use serde_json::Value;

    let mut project_config: Vec<ProjectConfig> = Vec::new();
    let mut config_json: Vec<(String, String, String)> = Vec::new();

    let source_config = mgmt_api_call(format!("/projects/{}/config/auth", source_project_rw)).await?;
    let dest_config = mgmt_api_call(format!("/projects/{}/config/auth", dest_project_rw)).await?;
    config_json.push(("Auth".to_string(), source_config, dest_config));

    let source_config = mgmt_api_call(format!("/projects/{}/postgrest", source_project_rw)).await?;
    let dest_config = mgmt_api_call(format!("/projects/{}/postgrest", dest_project_rw)).await?;
    config_json.push(("Postgrest".to_string(), source_config, dest_config));

    let source_config = mgmt_api_call(format!("/projects/{}/functions", source_project_rw)).await?;
    let dest_config = mgmt_api_call(format!("/projects/{}/functions", dest_project_rw)).await?;
    config_json.push(("Edge Functions".to_string(), source_config, dest_config));

    let source_config = mgmt_api_call(format!("/projects/{}/secrets", source_project_rw)).await?;
    let dest_config = mgmt_api_call(format!("/projects/{}/secrets", dest_project_rw)).await?;
    config_json.push(("Project Secrets".to_string(), source_config, dest_config));

    let source_config = mgmt_api_call(format!("/projects/{}/config/database/postgres", source_project_rw)).await?;
    let dest_config = mgmt_api_call(format!("/projects/{}/config/database/postgres",dest_project_rw)).await?;
    config_json.push(("Postgres".to_string(), source_config, dest_config));

    for (config_type, source_json, dest_json) in config_json {
        let source_value: Value = serde_json::from_str(&source_json)?;
        let dest_value: Value = serde_json::from_str(&dest_json)?;

        let diff_option = JsonDiff::diff_string(&source_value, &dest_value, false);
        match diff_option {
            Some(diff_strings) => {
                let config_diffs = format_diff_output(diff_strings.as_str());
                project_config.push(ProjectConfig { 
                    name: config_type.clone(), 
                    diffs: config_diffs, 
                    config_json: source_json 
                });                    
            }
            None => {
                project_config.push(ProjectConfig { 
                    name: config_type.clone(), 
                    diffs: Vec::new(), 
                    config_json: "".to_string()
                });    
            }
        }
    }
    Ok(project_config)
}

#[cfg(feature = "ssr")]
fn format_diff_output(diff_str: &str) -> Vec<DiffEntry> {
    use std::collections::HashMap;

    let mut source_map: HashMap<String, String> = HashMap::new();
    let mut dest_map: HashMap<String, String> = HashMap::new();
    let mut diff_entries: Vec<DiffEntry> = Vec::new();

    for line in diff_str.lines() {
        if line.trim().starts_with("- ") {
            let parts: Vec<&str> = line.trim_start_matches("- ").splitn(2, ": ").collect();
            if parts.len() == 2 {
                source_map.insert(parts[0].to_string(), parts[1].to_string());
            }
        } else if line.trim().starts_with("+ ") {
            let parts: Vec<&str> = line.trim_start_matches("+ ").splitn(2, ": ").collect();
            if parts.len() == 2 {
                dest_map.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }

    for (key, source_val) in &source_map {
        if let Some(dest_val) = dest_map.get(key) {
            if source_val != dest_val {
                diff_entries.push(DiffEntry {
                    key: key.to_string(),
                    source_value: source_val.to_string(),
                    dest_value: dest_val.to_string(),
                });
            }
        }
    }

    diff_entries
}
