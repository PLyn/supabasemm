// src/metrics/functions/websocket_metrics.rs
use leptos::prelude::*;
use server_fn::{codec::JsonEncoding, BoxedStream, ServerFnError, Websocket};
use crate::shared::models::ProjectMetrics;

#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]
pub async fn websocket_metrics_stream(
    input: BoxedStream<String, ServerFnError>,
) -> Result<BoxedStream<Vec<ProjectMetrics>, ServerFnError>, ServerFnError> {
    use futures::{channel::mpsc, SinkExt, StreamExt};
    use tokio::time::{interval, Duration};
    use leptos_axum::extract;
    use tower_sessions::Session;
    
    let session: Session = extract().await?;
    let access_token: Option<String> = session.get("supabase_access_token").await?;
    
    let access_token = match access_token {
        Some(token) => token,
        None => return Err(ServerFnError::ServerError("No access token found in session".to_string()))
    };
    
    let mut input = input;
    let (mut tx, rx) = mpsc::channel(1);

    tokio::spawn(async move {
        let mut project_ref: Option<String> = None;
        let mut metrics_interval = interval(Duration::from_secs(60)); // 1 minute
        
        loop {
            tokio::select! {
                msg = input.next() => {
                    match msg {
                        Some(Ok(new_project_ref)) => {
                            if !new_project_ref.is_empty() {
                                project_ref = Some(new_project_ref);
                                println!("WebSocket: Project ref updated to: {:?}", project_ref);
                                
                                if let Some(ref proj_ref) = project_ref {
                                    
                                    match get_project_metrics_internal(proj_ref.clone(), access_token.clone()).await {
                                        Ok(metrics) => {
                                            if let Err(e) = tx.send(Ok(metrics)).await {
                                                println!("Failed to send initial metrics: {}", e);
                                                break;
                                            }
                                        }
                                        Err(e) => {
                                            println!("Error getting initial metrics: {}", e);
                                            if let Err(send_err) = tx.send(Err(ServerFnError::ServerError(format!("Metrics error: {}", e)))).await {
                                                println!("Failed to send error: {}", send_err);
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(Err(e)) => {
                            println!("WebSocket input error: {}", e);
                            if let Err(send_err) = tx.send(Err(e)).await {
                                println!("Failed to send input error: {}", send_err);
                                break;
                            }
                        }
                        None => {
                            println!("WebSocket input stream closed");
                            break;
                        }
                    }
                }
                
                _ = metrics_interval.tick() => {
                    if let Some(ref proj_ref) = project_ref {
                        match get_project_metrics_internal(proj_ref.clone(), access_token.clone()).await {
                            Ok(metrics) => {
                                if let Err(e) = tx.send(Ok(metrics)).await {
                                    println!("Failed to send periodic metrics: {}", e);
                                    break;
                                }
                            }
                            Err(e) => {
                                println!("Error getting periodic metrics: {}", e);
                                if let Err(send_err) = tx.send(Err(ServerFnError::ServerError(format!("Metrics error: {}", e)))).await {
                                    println!("Failed to send periodic error: {}", send_err);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    Ok(rx.into())
}

#[cfg(feature = "ssr")]
pub async fn get_project_metrics_internal(
    project_ref: String,
    access_token: String,
) -> Result<Vec<ProjectMetrics>, ServerFnError> {
    use base64::{engine::general_purpose, Engine as _};
    use prometheus_parse::{self, Value, Scrape};
    use chrono::Utc;
    use reqwest::header::{HeaderValue, AUTHORIZATION, ACCEPT};
    use crate::shared::models::ApiKeyStruct;

    let mut service_role_key_option: Option<String> = None;
    let service_role_key: String;
    let get_api_key_url = format!("https://api.supabase.com/v1/projects/{}/api-keys?reveal=true", project_ref);
    
    let client = reqwest::Client::new();
    
    let text_response = client
        .get(&get_api_key_url)
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .text()
        .await?;

    match serde_json::from_str::<Vec<ApiKeyStruct>>(&text_response) {
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

    let auth_string = format!("service_role:{}", service_role_key);
    let encoded_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());
    let auth_header_value = format!("Basic {}", encoded_auth);
    
    let metrics_text = client
        .get(&metrics_url)
        .header(AUTHORIZATION, HeaderValue::from_str(&auth_header_value)?)
        .send()
        .await?
        .text()
        .await?;

    let lines_iterator = metrics_text.lines().map(|s| Ok(s.to_owned()));

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
                            continue;
                        }
                    }

                    let value = match sample.value {
                        Value::Counter(v) | Value::Gauge(v) | Value::Untyped(v) => v,
                        Value::Summary(_summary_counts) => {
                            continue;
                        }
                        Value::Histogram(_histogram_counts) => {
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