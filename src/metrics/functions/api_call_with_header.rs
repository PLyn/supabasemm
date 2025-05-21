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

                        for sample in parsed_scrape.samples {
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

                            let labels_map: HashMap<String, String> = (*sample.labels).clone();

                            project_metrics_list.push(ProjectMetrics {
                                timestamp: now.clone(), 
                                value,
                                metric_name: sample.metric.clone(),
                                labels: Some(labels_map), 
                            });
                        }
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