use leptos::prelude::*;
use super::ProjectMetrics; // Assuming ProjectMetrics is defined in the same crate

#[server]
pub async fn api_call_with_header(url: String, header: String) -> Result<Vec<ProjectMetrics>, ServerFnError> {
    // Imports
    use prometheus_parse::{self, Value, Scrape};
    use std::collections::HashMap;
    use chrono::Utc;
    use crate::server::api::handle_response_error;
    use reqwest::header::{AUTHORIZATION, HeaderValue};
    let client = reqwest::Client::new();

    eprintln!("Fetching from: {}", url);
    let result = client
        .get(&url)
        .header(AUTHORIZATION, HeaderValue::from_str(&header)?)
        .send()
        .await;

    match result {
        Ok(response) => {
            if response.status().is_success() {
                let metrics_text = response.text().await?;
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
                            // EBS Balance (AWS metrics)
                            "aws_ebs_burst_balance",
                            "aws_ebs_io_balance",
                            "aws_ebs_byte_balance",
                        ];
                        for sample in parsed_scrape.samples {
                            // Filter for desired metrics
                            if desired_metrics.contains(&sample.metric.as_str()) {
                                // For disk metrics, also check the mountpoint label
                                if sample.metric.starts_with("node_filesystem_") {
                                    let mountpoint = sample.labels.get("mountpoint");
                                    if mountpoint != Some(&"/".to_string()) && mountpoint != Some(&"/data".to_string()) {
                                        // Skip if not the root or data filesystem
                                        continue;
                                    }
                                }

                                let value = match sample.value {
                                    Value::Counter(v) | Value::Gauge(v) | Value::Untyped(v) => v,
                                    Value::Summary(summary_counts) => {
                                        eprintln!("Skipping direct Value::Summary for metric {}. Look for _sum or specific quantiles instead.", sample.metric);
                                        continue;
                                    }
                                    Value::Histogram(histogram_counts) => {
                                        eprintln!("Skipping direct Value::Histogram for metric {}. Look for _sum or specific buckets instead.", sample.metric);
                                        continue;
                                    }
                                };

                                // Convert labels HashMap to a String for storage, or parse them as needed
                                let labels_string: String = sample.labels
                                    .iter()
                                    .map(|(key, value)| format!("{}=\"{}\"", key, value))
                                    .collect::<Vec<String>>()
                                    .join(",");

                                project_metrics_list.push(ProjectMetrics {
                                    timestamp: now.clone(),
                                    value: value.to_string(),
                                    metric_name: sample.metric.clone(),
                                    labels: labels_string, // Store labels as a string
                                });
                            }
                        }
                        eprintln!("For loop end");
                        Ok(project_metrics_list)
                    }
                    Err(e) => Err(ServerFnError::ServerError(format!("Error parsing Prometheus metrics: {:?}", e))),
                }
            } else {
                Err(handle_response_error(response).await)
            }
        }
        Err(e) => Err(ServerFnError::ServerError(format!("Error fetching URL: {:?}", e)))
    }
}