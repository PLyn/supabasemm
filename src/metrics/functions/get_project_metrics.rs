use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use super::api_call_with_header;
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectMetrics {
    pub timestamp: String,
    pub value: f64,
    pub metric_name: String,
    pub labels: Option<std::collections::HashMap<String, String>>,
}

#[server]
pub async fn get_project_metrics(project_ref: String) -> Result<Vec<ProjectMetrics>, ServerFnError> {
    // Imports
    use crate::server::api::get_api_call;
    use crate::server::api::handle_response_error;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use base64::{engine::general_purpose, Engine as _};

    let session: Session = extract().await?;

    let get_api_key_url = format!("https://api.supabase.com/v1/projects/{}/api-keys?reveal=true", project_ref);
    eprintln!("Fetching API keys from: {}", get_api_key_url);
    let api_keys_response = get_api_call(session.clone(), get_api_key_url).await?;

    let mut service_role_key_option: Option<String> = None;
    let service_role_key: String;

    if api_keys_response.status().is_success() {
        match api_keys_response.json::<Vec<ApiKey>>().await {
            Ok(api_keys) => {
                eprintln!("Successfully parsed API keys: {:?}", api_keys);
                for key in api_keys {
                    if key.name == "service_role" {
                        service_role_key_option = Some(key.api_key);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error parsing API keys JSON: {:?}", e);
                return Err(ServerFnError::ServerError(format!("Error parsing API keys JSON: {:?}", e)));
            }
        }
    } else {
        return Err(handle_response_error(api_keys_response).await);
    }

    match service_role_key_option {
        Some(key) => {
            eprintln!("Found service_role_key: {:?}", key);
            service_role_key = key;
        }
        None => {
            return Err(ServerFnError::ServerError("Service role API key not found".to_string()));
        }
    }

    let metrics_url = format!("https://{}.supabase.co/customer/v1/privileged/metrics", project_ref);
    eprintln!("Fetching metrics from: {}", metrics_url);
 
    let auth_string = format!("service_role:{}", service_role_key);
    let encoded_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());
    let auth_header_value = format!("Basic {}", encoded_auth);

    let metrics_response = api_call_with_header(metrics_url, auth_header_value).await;

    match metrics_response {
        Ok(response) => Ok(response),
        
        Err(e) => Err(ServerFnError::ServerError(format!("Error getting metrics: {:?}", e)))
    }
}