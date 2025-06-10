#[cfg(feature = "ssr")]
use crate::shared::models::{DiffEntry, ProjectConfig};

use leptos::prelude::*;
#[cfg(feature = "ssr")]
use serde_json::Value;

#[cfg(feature = "ssr")]
pub async fn json_diff(
    config_type: String,
    source_value: Value,
    dest_value: Value,
) -> Result<Option<ProjectConfig>, ServerFnError> {
    use json_structural_diff::JsonDiff;

    let diff_option = JsonDiff::diff_string(&source_value, &dest_value, false);
    if let Some(diff_strings) = diff_option {
        let (config_diffs, diff_map) = format_diff_output(diff_strings.as_str());
        let body_string = serde_json::to_string(&diff_map)?;

        if body_string.len() > 2 {
            return Ok(Some(ProjectConfig {
                name: config_type.clone(),
                diffs: config_diffs,
            }));
        }
    }
    Ok(None)
}

#[cfg(feature = "ssr")]
fn format_diff_output(diff_str: &str) -> (Vec<DiffEntry>, Value) {
    use serde_json::json;
    use std::collections::HashMap;

    let mut source_map: HashMap<String, String> = HashMap::new();
    let mut dest_map: HashMap<String, String> = HashMap::new();
    let mut diff_entries: Vec<DiffEntry> = Vec::new();
    let mut diff_map: HashMap<String, String> = HashMap::new();
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

                diff_map.insert(key.clone(), source_val.clone());
            }
        }
    }

    (diff_entries, json!(diff_map))
}
