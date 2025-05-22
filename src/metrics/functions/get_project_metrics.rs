use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::shared::models::ProjectMetrics;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiKey {
    pub name: String, 
    pub api_key: String,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub key_type: Option<String>,
    pub prefix: Option<String>,
    pub description: Option<String>,
    pub hash: Option<String>,
    pub secret_jwt_template: Option<SecretJwtTemplate>,
    pub inserted_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecretJwtTemplate {
    pub role: String,
}

#[server]
pub async fn get_project_metrics(project_ref: String) -> Result<Vec<ProjectMetrics>, ServerFnError> {
    use crate::shared::server_functions::mgmt_api_call;
    use crate::shared::server_functions::mgmt_api_call_with_header;
    use base64::{engine::general_purpose, Engine as _};
    use prometheus_parse::{self, Value, Scrape};
    use chrono::Utc;

    let mut service_role_key_option: Option<String> = None;
    let service_role_key: String;
    let get_api_key_url = format!("https://api.supabase.com/v1/projects/{}/api-keys?reveal=true", project_ref);
    let text_response = mgmt_api_call(get_api_key_url).await?;

    match serde_json::from_str::<Vec<ApiKey>>(&text_response) {
        Ok(api_keys) => {
            for key in api_keys {
                if key.name == "service_role" {
                    service_role_key_option = Some(key.api_key);
                    break;
                }
            }
        }
        Err(e) => return Err(ServerFnError::ServerError(format!("Error parsing API keys JSON: {:?}", e)))
    }

    match service_role_key_option {
        Some(key) => service_role_key = key,
        None => return Err(ServerFnError::ServerError("Service role API key not found".to_string()))  
    }

    let metrics_url = format!("https://{}.supabase.co/customer/v1/privileged/metrics", project_ref);
    eprintln!("Fetching metrics from: {}", metrics_url);
 
    let auth_string = format!("service_role:{}", service_role_key);
    let encoded_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());
    let auth_header_value = format!("Basic {}", encoded_auth);

    let metrics_response = mgmt_api_call_with_header(metrics_url, auth_header_value).await;

    match metrics_response {
        Ok(text) => {
            let lines_iterator = text.lines().map(|s| Ok(s.to_owned()));

            match Scrape::parse(lines_iterator) {
                Ok(parsed_scrape) => {
                    let mut project_metrics_list: Vec<ProjectMetrics> = Vec::new();
                    let now = Utc::now().to_rfc3339();

                    let desired_metrics: Vec<&str> = vec![
                        // CPU
                        "node_cpu_seconds_total",
                        "node_load1",
                        "node_load5",
                        "node_load15",
                        // RAM
                        "node_memory_MemTotal_bytes",
                        "node_memory_MemFree_bytes",
                        "node_memory_MemAvailable_bytes",
                        "node_memory_SwapTotal_bytes",
                        "node_memory_SwapFree_bytes",
                        // Disk I/O (Filesystem)
                        "node_filesystem_avail_bytes",
                        "node_filesystem_size_bytes",
                        // Disk I/O Operations
                        "node_disk_reads_completed_total",
                        "node_disk_writes_completed_total",
                        "node_disk_read_bytes_total",
                        "node_disk_written_bytes_total",
                        "node_disk_read_time_seconds_total",
                        "node_disk_write_time_seconds_total",
                        "node_disk_io_time_seconds_total",
                        "node_disk_io_time_weighted_seconds_total",
                    ];
                    for sample in parsed_scrape.samples {
                        if desired_metrics.contains(&sample.metric.as_str()) {
                            if sample.metric.starts_with("node_filesystem_") {
                                let mountpoint = sample.labels.get("mountpoint");
                                if mountpoint != Some(&"/".to_string()) && mountpoint != Some(&"/data".to_string()) {
                                    // Skip if not the root or data filesystem
                                    continue;
                                }
                            }

                            let value = match sample.value {
                                Value::Counter(v) | Value::Gauge(v) | Value::Untyped(v) => v,
                                Value::Summary(_summary_counts) => {
                                    eprintln!("Skipping direct Value::Summary for metric {}. Look for _sum or specific quantiles instead.", sample.metric);
                                    continue;
                                }
                                Value::Histogram(_histogram_counts) => {
                                    eprintln!("Skipping direct Value::Histogram for metric {}. Look for _sum or specific buckets instead.", sample.metric);
                                    continue;
                                }
                            };

                            let labels_string: String = sample.labels
                                .iter()
                                .map(|(key, value)| format!("{}=\"{}\"", key, value))
                                .collect::<Vec<String>>()
                                .join(",");

                            project_metrics_list.push(ProjectMetrics {
                                timestamp: now.clone(),
                                value: value.to_string(),
                                metric_name: sample.metric.clone(),
                                labels: labels_string,
                            });
                        }
                    }
                    Ok(project_metrics_list)
                }
                Err(e) => Err(ServerFnError::ServerError(format!("Error parsing Prometheus metrics: {:?}", e))),
            }
        }
        Err(e) => Err(ServerFnError::ServerError(format!("Error getting metrics: {:?}", e)))
    }
}