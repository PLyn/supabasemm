use leptos::prelude::*;
use crate::shared::models::DiffEntry;
use crate::shared::server_functions::mgmt_api_call;

#[server]
pub async fn generate_preview(
    source_project_rw: String,
    dest_project_rw: String,
) -> Result<Vec<DiffEntry>, ServerFnError> {
    use serde_json::Value;
    use json_structural_diff::JsonDiff;
    
    let mut diff_entries: Vec<DiffEntry> = Vec::new();
    let mut results: Vec<(String, Result<String, ServerFnError>, Result<String, ServerFnError>)> = Vec::new();

    let source_config = mgmt_api_call(format!("/projects/{}/config/auth", source_project_rw)).await;
    let dest_config = mgmt_api_call(format!("/projects/{}/config/auth", dest_project_rw)).await;
    results.push(("Auth".to_string(), source_config, dest_config));

    let source_config = mgmt_api_call(format!("/projects/{}/postgrest", source_project_rw)).await;
    let dest_config = mgmt_api_call(format!("/projects/{}/postgrest", dest_project_rw)).await;
    results.push(("Postgrest".to_string(), source_config, dest_config));

    let source_config = mgmt_api_call(format!("/projects/{}/functions", source_project_rw)).await;
    let dest_config = mgmt_api_call(format!("/projects/{}/functions", dest_project_rw)).await;
    results.push(("Edge Functions".to_string(), source_config, dest_config));

    let source_config = mgmt_api_call(format!("/projects/{}/secrets", source_project_rw)).await;
    let dest_config = mgmt_api_call(format!("/projects/{}/secrets", dest_project_rw)).await;
    results.push(("Project Secrets".to_string(), source_config, dest_config));

    let source_config = mgmt_api_call(format!("/projects/{}/config/database/postgres", source_project_rw)).await;
    let dest_config = mgmt_api_call(format!("/projects/{}/config/database/postgres", dest_project_rw)).await;
    results.push(("Postgres".to_string(), source_config, dest_config));

    for (config_type, source_config_result, dest_config_result) in results {
        match (source_config_result, dest_config_result) {
            (Ok(source_json_string), Ok(dest_json_string)) => {
                let source_value: Result<Value, _> = serde_json::from_str(&source_json_string);
                let dest_value: Result<Value, _> = serde_json::from_str(&dest_json_string);

                match (source_value, dest_value) {
                    (Ok(s_val), Ok(d_val)) => {
                        let diff = JsonDiff::diff_string(&s_val, &d_val, false);
                        if let Some(diff_str) = diff {
                            let config_diffs = format_diff_output(config_type.clone(), diff_str.as_str());
                            diff_entries.extend(config_diffs);
                        } else {
                            diff_entries.push(DiffEntry {
                                config_type: config_type.clone(),
                                key: "".to_string(),
                                source_value: "No differences in config found".to_string(),
                                dest_value: "No differences in config found".to_string(),
                            });
                        }
                    }
                    (Err(e), _) => {
                        diff_entries.push(DiffEntry {
                            config_type: config_type.clone(),
                            key: format!("Error parsing source config JSON: {}", e),
                            source_value: "".to_string(),
                            dest_value: "".to_string(),
                        });
                    }
                    (_, Err(e)) => {
                        diff_entries.push(DiffEntry {
                            config_type: config_type.clone(),
                            key: format!("Error parsing destination config JSON: {}", e),
                            source_value: "".to_string(),
                            dest_value: "".to_string(),
                        });
                    }
                }
            }
            (Err(e), _) => {
                diff_entries.push(DiffEntry {
                    config_type: config_type.clone(),
                    key: format!("Error fetching source config: {}", e),
                    source_value: "".to_string(),
                    dest_value: "".to_string(),
                });                
            }
            (_, Err(e)) => {
                diff_entries.push(DiffEntry {
                    config_type: config_type.clone(),
                    key: format!("Error fetching destination config: {}", e),
                    source_value: "".to_string(),
                    dest_value: "".to_string(),
                }); 
            }
        }
    }

    Ok(diff_entries)
}


fn format_diff_output(config_type: String, diff_str: &str) -> Vec<DiffEntry> {
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
                    config_type: config_type.clone(),
                    key: key.to_string(),
                    source_value: source_val.to_string(),
                    dest_value: dest_val.to_string(),
                });
            }
        }
    }

    diff_entries
}