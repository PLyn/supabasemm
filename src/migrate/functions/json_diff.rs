use crate::shared::models::{DiffEntry, ProjectConfig};
use leptos::prelude::*;
use std::collections::HashMap;

#[cfg(feature = "ssr")]
use serde_json::{Value, Map};

#[cfg(feature = "ssr")]
pub async fn json_diff(
    config_type: String,
    source_value: Value,
    dest_value: Value,
) -> Result<Option<ProjectConfig>, ServerFnError> {
    let diff_entries = calculate_diff(&source_value, &dest_value)?;
    
    if diff_entries.is_empty() {
        Ok(None)
    } else {
        Ok(Some(ProjectConfig {
            name: config_type,
            diffs: diff_entries,
        }))
    }
}

#[cfg(feature = "ssr")]
fn calculate_diff(source: &Value, dest: &Value) -> Result<Vec<DiffEntry>, ServerFnError> {
    let mut diff_entries = Vec::new();
    diff_values("", source, dest, &mut diff_entries);
    Ok(diff_entries)
}

#[cfg(feature = "ssr")]
fn diff_values(path: &str, source: &Value, dest: &Value, diffs: &mut Vec<DiffEntry>) {
    use Value::*;
    
    match (source, dest) {
        (Array(src), Array(dst)) => diff_arrays(path, src, dst, diffs),
        (Object(src), Object(dst)) => diff_objects(path, src, dst, diffs),
        _ if source != dest => {
            diffs.push(DiffEntry {
                key: if path.is_empty() { "root" } else { path }.to_string(),
                source_value: format_value(source),
                dest_value: format_value(dest),
            });
        }
        _ => {} // Values are equal
    }
}

#[cfg(feature = "ssr")]
fn diff_arrays(path: &str, src: &[Value], dst: &[Value], diffs: &mut Vec<DiffEntry>) {
    // Report length difference if arrays are different sizes
    if src.len() != dst.len() {
        diffs.push(DiffEntry {
            key: format!("{}{}length", path, if path.is_empty() { "" } else { "." }),
            source_value: src.len().to_string(),
            dest_value: dst.len().to_string(),
        });
    }
    
    // Check if arrays contain objects with IDs
    let src_map = to_id_map(src);
    let dst_map = to_id_map(dst);
    
    match (src_map, dst_map) {
        (Some(src_ids), Some(dst_ids)) => {
            // Both arrays have objects with IDs
            diff_by_id(path, src_ids, dst_ids, diffs);
        }
        (Some(src_ids), None) => {
            // Only source has objects with IDs, destination is empty or has no IDs
            for (id, val) in src_ids {
                diffs.push(DiffEntry {
                    key: format!("{}{}id:{}", path, if path.is_empty() { "" } else { "." }, id),
                    source_value: format_value(val),
                    dest_value: "null".to_string(),
                });
            }
        }
        (None, Some(dst_ids)) => {
            // Only destination has objects with IDs
            for (id, val) in dst_ids {
                diffs.push(DiffEntry {
                    key: format!("{}{}id:{}", path, if path.is_empty() { "" } else { "." }, id),
                    source_value: "null".to_string(),
                    dest_value: format_value(val),
                });
            }
        }
        (None, None) => {
            // Neither array has objects with IDs, use index-based comparison
            diff_by_index(path, src, dst, diffs);
        }
    }
}

#[cfg(feature = "ssr")]
fn to_id_map(arr: &[Value]) -> Option<HashMap<String, &Value>> {
    let mut map = HashMap::new();
    let mut has_ids = false;
    
    for item in arr {
        if let Value::Object(obj) = item {
            if let Some(Value::String(id)) = obj.get("id") {
                map.insert(id.clone(), item);
                has_ids = true;
            }
        }
    }
    
    if has_ids { Some(map) } else { None }
}

#[cfg(feature = "ssr")]
fn diff_by_id(
    path: &str,
    src_map: HashMap<String, &Value>,
    dst_map: HashMap<String, &Value>,
    diffs: &mut Vec<DiffEntry>
) {
    // Find removed items
    for (id, src_val) in &src_map {
        if !dst_map.contains_key(id) {
            diffs.push(DiffEntry {
                key: format!("{}{}id:{}", path, if path.is_empty() { "" } else { "." }, id),
                source_value: format_value(src_val),
                dest_value: "null".to_string(),
            });
        }
    }
    
    // Find added items
    for (id, dst_val) in &dst_map {
        if !src_map.contains_key(id) {
            diffs.push(DiffEntry {
                key: format!("{}{}id:{}", path, if path.is_empty() { "" } else { "." }, id),
                source_value: "null".to_string(),
                dest_value: format_value(dst_val),
            });
        }
    }
    
    // Compare modified items
    for (id, src_val) in &src_map {
        if let Some(&dst_val) = dst_map.get(id) {
            let item_path = format!("{}{}id:{}", path, if path.is_empty() { "" } else { "." }, id);
            diff_values(&item_path, src_val, dst_val, diffs);
        }
    }
}

#[cfg(feature = "ssr")]
fn diff_by_index(path: &str, src: &[Value], dst: &[Value], diffs: &mut Vec<DiffEntry>) {
    let max_len = src.len().max(dst.len());
    
    for i in 0..max_len {
        let item_path = format!("{}[{}]", path, i);
        
        match (src.get(i), dst.get(i)) {
            (Some(s), Some(d)) => diff_values(&item_path, s, d, diffs),
            (Some(s), None) => diffs.push(DiffEntry {
                key: item_path,
                source_value: format_value(s),
                dest_value: "null".to_string(),
            }),
            (None, Some(d)) => diffs.push(DiffEntry {
                key: item_path,
                source_value: "null".to_string(),
                dest_value: format_value(d),
            }),
            _ => {}
        }
    }
}

#[cfg(feature = "ssr")]
fn diff_objects(path: &str, src: &Map<String, Value>, dst: &Map<String, Value>, diffs: &mut Vec<DiffEntry>) {
    // Check all source keys
    for (key, src_val) in src {
        let field_path = if path.is_empty() { 
            key.clone() 
        } else { 
            format!("{}.{}", path, key) 
        };
        
        match dst.get(key) {
            Some(dst_val) => diff_values(&field_path, src_val, dst_val, diffs),
            None => diffs.push(DiffEntry {
                key: field_path,
                source_value: format_value(src_val),
                dest_value: "null".to_string(),
            }),
        }
    }
    
    // Check destination-only keys
    for (key, dst_val) in dst {
        if !src.contains_key(key) {
            let field_path = if path.is_empty() { 
                key.clone() 
            } else { 
                format!("{}.{}", path, key) 
            };
            diffs.push(DiffEntry {
                key: field_path,
                source_value: "null".to_string(),
                dest_value: format_value(dst_val),
            });
        }
    }
}

#[cfg(feature = "ssr")]
fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => "null".to_string(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(_) | Value::Object(_) => value.to_string(),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_object_diff() {
        let source: Value = serde_json::from_str(r#"{"a": 1, "b": 2}"#).unwrap();
        let dest: Value = serde_json::from_str(r#"{"a": 1, "b": 3, "c": 4}"#).unwrap();
        
        let result = json_diff("test".to_string(), source, dest).await.unwrap();
        let config = result.unwrap();
        
        assert_eq!(config.diffs.len(), 2); // b changed, c added
        assert!(config.diffs.iter().any(|d| d.key == "b" && d.dest_value == "3"));
        assert!(config.diffs.iter().any(|d| d.key == "c" && d.source_value == "null"));
    }
    
    #[tokio::test]
    async fn test_edge_functions_diff() {
        let source = r#"[
            {"id": "func1", "version": 1},
            {"id": "func2", "version": 1}
        ]"#;
        let dest = r#"[]"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("test".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        assert!(config.diffs.iter().any(|d| d.key == "length"));
        assert!(config.diffs.iter().any(|d| d.key == "id:func1"));
        assert!(config.diffs.iter().any(|d| d.key == "id:func2"));
    }
    
    #[tokio::test]
    async fn test_no_diff() {
        let source = r#"{"a": 1, "b": "test", "c": true}"#;
        let dest = r#"{"a": 1, "b": "test", "c": true}"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("test".to_string(), source_value, dest_value).await.unwrap();
        assert!(result.is_none());
    }
    
    #[tokio::test]
    async fn test_nested_object_diff() {
        let source = r#"{
            "user": {
                "name": "John",
                "age": 30,
                "address": {
                    "street": "123 Main St",
                    "city": "Boston"
                }
            }
        }"#;
        let dest = r#"{
            "user": {
                "name": "John",
                "age": 31,
                "address": {
                    "street": "123 Main St",
                    "city": "New York",
                    "zip": "10001"
                }
            }
        }"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("test".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        assert_eq!(config.diffs.len(), 3);
        assert!(config.diffs.iter().any(|d| d.key == "user.age" && d.dest_value == "31"));
        assert!(config.diffs.iter().any(|d| d.key == "user.address.city" && d.dest_value == "New York"));
        assert!(config.diffs.iter().any(|d| d.key == "user.address.zip" && d.source_value == "null"));
    }
    
    #[tokio::test]
    async fn test_array_of_primitives() {
        let source = r#"[1, 2, 3, 4]"#;
        let dest = r#"[1, 2, 5]"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("test".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        assert!(config.diffs.iter().any(|d| d.key == "length"));
        assert!(config.diffs.iter().any(|d| d.key == "[2]" && d.source_value == "3" && d.dest_value == "5"));
        assert!(config.diffs.iter().any(|d| d.key == "[3]" && d.source_value == "4" && d.dest_value == "null"));
    }
    
    #[tokio::test]
    async fn test_mixed_types() {
        let source = r#"{"value": "123"}"#;
        let dest = r#"{"value": 123}"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("test".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        assert_eq!(config.diffs.len(), 1);
        assert!(config.diffs.iter().any(|d| d.key == "value" && d.source_value == "123" && d.dest_value == "123"));
    }
    
    #[tokio::test]
    async fn test_null_values() {
        let source = r#"{"a": null, "b": "value"}"#;
        let dest = r#"{"a": "new_value", "b": null}"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("test".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        assert_eq!(config.diffs.len(), 2);
        assert!(config.diffs.iter().any(|d| d.key == "a" && d.source_value == "null" && d.dest_value == "new_value"));
        assert!(config.diffs.iter().any(|d| d.key == "b" && d.source_value == "value" && d.dest_value == "null"));
    }
    
    #[tokio::test]
    async fn test_boolean_diff() {
        let source = r#"{"active": true, "verified": false}"#;
        let dest = r#"{"active": false, "verified": false}"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("test".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        assert_eq!(config.diffs.len(), 1);
        assert!(config.diffs.iter().any(|d| d.key == "active" && d.source_value == "true" && d.dest_value == "false"));
    }
    
    #[tokio::test]
    async fn test_array_to_object() {
        let source = r#"{"data": [1, 2, 3]}"#;
        let dest = r#"{"data": {"key": "value"}}"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("test".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        assert_eq!(config.diffs.len(), 1);
        assert!(config.diffs.iter().any(|d| d.key == "data"));
    }
    
    #[tokio::test]
    async fn test_empty_to_populated() {
        let source = r#"{}"#;
        let dest = r#"{"a": 1, "b": "test"}"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("test".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        assert_eq!(config.diffs.len(), 2);
        assert!(config.diffs.iter().any(|d| d.key == "a" && d.source_value == "null"));
        assert!(config.diffs.iter().any(|d| d.key == "b" && d.source_value == "null"));
    }
    
    #[tokio::test]
    async fn test_array_with_modified_objects() {
        let source = r#"[
            {"id": "1", "name": "Item 1", "price": 10},
            {"id": "2", "name": "Item 2", "price": 20}
        ]"#;
        let dest = r#"[
            {"id": "1", "name": "Item 1", "price": 15},
            {"id": "2", "name": "Item 2 Updated", "price": 20}
        ]"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("test".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        assert_eq!(config.diffs.len(), 2);
        assert!(config.diffs.iter().any(|d| d.key == "id:1.price" && d.source_value == "10" && d.dest_value == "15"));
        assert!(config.diffs.iter().any(|d| d.key == "id:2.name" && d.dest_value == "Item 2 Updated"));
    }
    
    #[tokio::test]
    async fn test_complex_edge_functions() {
        let source = r#"[{
            "verify_jwt": true,
            "id": "d5f0075a-a49a-4243-aec8-7a750b69ee5d",
            "slug": "log-hi-message",
            "version": 7,
            "name": "log-hi-message",
            "status": "ACTIVE",
            "entrypoint_path": "file:///tmp/user_fn_zfgioqfrunhcvbvoxvwq_d5f0075a-a49a-4243-aec8-7a750b69ee5d_5/index.ts",
            "import_map_path": null,
            "import_map": false,
            "created_at": 1746908962504,
            "updated_at": 1748106892462
        }]"#;
        
        let dest = r#"[{
            "verify_jwt": false,
            "id": "d5f0075a-a49a-4243-aec8-7a750b69ee5d",
            "slug": "log-hi-message",
            "version": 8,
            "name": "log-hi-message-v2",
            "status": "INACTIVE",
            "entrypoint_path": "file:///tmp/user_fn_zfgioqfrunhcvbvoxvwq_d5f0075a-a49a-4243-aec8-7a750b69ee5d_6/index.ts",
            "import_map_path": "/path/to/import.map",
            "import_map": true,
            "created_at": 1746908962504,
            "updated_at": 1748106892463
        }]"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("test".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        // Should detect multiple field changes
        assert!(config.diffs.iter().any(|d| d.key == "id:d5f0075a-a49a-4243-aec8-7a750b69ee5d.verify_jwt"));
        assert!(config.diffs.iter().any(|d| d.key == "id:d5f0075a-a49a-4243-aec8-7a750b69ee5d.version"));
        assert!(config.diffs.iter().any(|d| d.key == "id:d5f0075a-a49a-4243-aec8-7a750b69ee5d.name"));
        assert!(config.diffs.iter().any(|d| d.key == "id:d5f0075a-a49a-4243-aec8-7a750b69ee5d.status"));
        assert!(config.diffs.iter().any(|d| d.key == "id:d5f0075a-a49a-4243-aec8-7a750b69ee5d.import_map"));
        assert!(config.diffs.iter().any(|d| d.key == "id:d5f0075a-a49a-4243-aec8-7a750b69ee5d.import_map_path"));
    }
    
    #[tokio::test]
    async fn test_arrays_without_ids() {
        let source = r#"[
            {"name": "Item 1", "value": 100},
            {"name": "Item 2", "value": 200}
        ]"#;
        let dest = r#"[
            {"name": "Item 1", "value": 150},
            {"name": "Item 3", "value": 300}
        ]"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("test".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        // Should use index-based comparison
        assert!(config.diffs.iter().any(|d| d.key == "[0].value"));
        assert!(config.diffs.iter().any(|d| d.key == "[1].name"));
        assert!(config.diffs.iter().any(|d| d.key == "[1].value"));
    }
    
    #[tokio::test]
    async fn test_postgres_config_diff() {
        let source = r#"{
            "effective_cache_size": "4GB",
            "max_connections": 100,
            "maintenance_work_mem": "256MB",
            "session_replication_role": "origin",
            "shared_buffers": "1GB",
            "track_commit_timestamp": false,
            "work_mem": "4MB"
        }"#;
        let dest = r#"{
            "effective_cache_size": "8GB",
            "max_connections": 200,
            "maintenance_work_mem": "256MB",
            "session_replication_role": "replica",
            "shared_buffers": "2GB",
            "track_commit_timestamp": true,
            "work_mem": "8MB",
            "max_parallel_workers": 8
        }"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("postgres_config".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        // Should detect all changes including added field
        assert!(config.diffs.iter().any(|d| d.key == "effective_cache_size" && d.source_value == "4GB" && d.dest_value == "8GB"));
        assert!(config.diffs.iter().any(|d| d.key == "max_connections" && d.source_value == "100" && d.dest_value == "200"));
        assert!(config.diffs.iter().any(|d| d.key == "max_parallel_workers" && d.source_value == "null" && d.dest_value == "8"));
    }
    
    #[tokio::test]
    async fn test_auth_config_with_many_nulls() {
        let source = r#"{
            "disable_signup": false,
            "external_google_enabled": true,
            "external_google_client_id": "old-client-id",
            "external_google_secret": null,
            "mailer_otp_exp": 3600,
            "password_min_length": 6,
            "security_captcha_enabled": false,
            "security_captcha_provider": "hcaptcha",
            "sms_provider": "twilio"
        }"#;
        let dest = r#"{
            "disable_signup": false,
            "external_google_enabled": true,
            "external_google_client_id": "new-client-id",
            "external_google_secret": "new-secret",
            "mailer_otp_exp": 3600,
            "password_min_length": 8,
            "security_captcha_enabled": true,
            "security_captcha_provider": "turnstile",
            "sms_provider": "messagebird"
        }"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("auth_config".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        // Should detect null to value changes
        assert!(config.diffs.iter().any(|d| d.key == "external_google_secret" && d.source_value == "null" && d.dest_value == "new-secret"));
        assert!(config.diffs.iter().any(|d| d.key == "security_captcha_enabled" && d.source_value == "false" && d.dest_value == "true"));
    }
    
    #[tokio::test]
    async fn test_branch_environments_diff() {
        let source = r#"[
            {
                "id": "branch-1",
                "name": "staging",
                "project_ref": "proj-123",
                "parent_project_ref": null,
                "is_default": false,
                "git_branch": "staging",
                "pr_number": null,
                "persistent": true,
                "status": "ACTIVE",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z"
            },
            {
                "id": "branch-2",
                "name": "feature-x",
                "project_ref": "proj-124",
                "parent_project_ref": "proj-123",
                "is_default": false,
                "git_branch": "feature/x",
                "pr_number": 42,
                "persistent": false,
                "status": "CREATING_PROJECT",
                "created_at": "2024-01-02T00:00:00Z",
                "updated_at": "2024-01-02T00:00:00Z"
            }
        ]"#;
        let dest = r#"[
            {
                "id": "branch-1",
                "name": "staging",
                "project_ref": "proj-123",
                "parent_project_ref": null,
                "is_default": true,
                "git_branch": "staging",
                "pr_number": null,
                "persistent": true,
                "status": "ACTIVE",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-03T00:00:00Z"
            },
            {
                "id": "branch-3",
                "name": "feature-y",
                "project_ref": "proj-125",
                "parent_project_ref": "proj-123",
                "is_default": false,
                "git_branch": "feature/y",
                "pr_number": 43,
                "persistent": false,
                "status": "ACTIVE",
                "created_at": "2024-01-03T00:00:00Z",
                "updated_at": "2024-01-03T00:00:00Z"
            }
        ]"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("branches".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        // Should detect branch-2 removed, branch-3 added, branch-1 modified
        assert!(config.diffs.iter().any(|d| d.key == "id:branch-2"));
        assert!(config.diffs.iter().any(|d| d.key == "id:branch-3"));
        assert!(config.diffs.iter().any(|d| d.key == "id:branch-1.is_default"));
        assert!(config.diffs.iter().any(|d| d.key == "id:branch-1.updated_at"));
    }
    
    #[tokio::test]
    async fn test_large_config_with_mostly_nulls() {
        let source = r#"{
            "api_max_request_duration": null,
            "db_max_pool_size": null,
            "disable_signup": false,
            "external_email_enabled": true,
            "jwt_exp": 3600,
            "mailer_otp_exp": 3600,
            "password_required_characters": "abcdefghijklmnopqrstuvwxyz",
            "security_captcha_provider": "hcaptcha",
            "site_url": "https://old.example.com"
        }"#;
        let dest = r#"{
            "api_max_request_duration": 30,
            "db_max_pool_size": 25,
            "disable_signup": false,
            "external_email_enabled": true,
            "jwt_exp": 3600,
            "mailer_otp_exp": 3600,
            "password_required_characters": "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            "security_captcha_provider": "turnstile",
            "site_url": "https://new.example.com"
        }"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("config".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        // Should only report actual changes, not unchanged nulls
        assert!(config.diffs.iter().any(|d| d.key == "api_max_request_duration"));
        assert!(config.diffs.iter().any(|d| d.key == "db_max_pool_size"));
        assert!(config.diffs.iter().any(|d| d.key == "password_required_characters"));
        assert!(config.diffs.iter().any(|d| d.key == "site_url"));
        // Should not include unchanged fields
        assert!(!config.diffs.iter().any(|d| d.key == "jwt_exp"));
    }
    
    #[tokio::test]
    async fn test_special_characters_in_values() {
        let source = r#"{
            "smtp_pass": "old\"password\"with\\quotes",
            "uri_allow_list": "https://example.com,https://test.com",
            "mailer_templates_confirmation_content": "<h1>Welcome {{name}}</h1>",
            "external_azure_url": "https://login.microsoftonline.com/{{tenant}}/v2.0"
        }"#;
        let dest = r#"{
            "smtp_pass": "new'password'with\ttabs",
            "uri_allow_list": "https://example.com,https://test.com,https://new.com",
            "mailer_templates_confirmation_content": "<h2>Hello {{name}}!</h2>",
            "external_azure_url": "https://login.microsoftonline.com/common/v2.0"
        }"#;
        
        let source_value: Value = serde_json::from_str(source).unwrap();
        let dest_value: Value = serde_json::from_str(dest).unwrap();
        
        let result = json_diff("special_chars".to_string(), source_value, dest_value).await.unwrap();
        let config = result.unwrap();
        
        // Should handle special characters correctly
        assert_eq!(config.diffs.len(), 4);
        assert!(config.diffs.iter().any(|d| d.key == "smtp_pass"));
        assert!(config.diffs.iter().any(|d| d.key == "uri_allow_list"));
    }
}