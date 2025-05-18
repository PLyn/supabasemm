use oauth2::basic::BasicClient;
use serde::{Deserialize, Serialize};

// --- Configuration Struct ---
#[derive(Clone)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_uri: String,
    pub mgmt_api_base_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        use dotenvy::dotenv;
        use std::env;

        eprintln!("Loading environment variables for OAuth2 configuration...");
        dotenv().ok();

        let client_id = env::var("SUPA_CONNECT_CLIENT_ID")
            .map_err(|e| format!("SUPA_CONNECT_CLIENT_ID not found: {}", e))?;
        let client_secret = env::var("SUPA_CONNECT_CLIENT_SECRET")
            .map_err(|e| format!("SUPA_CONNECT_CLIENT_SECRET not found: {}", e))?;
        let redirect_uri =
            env::var("REDIRECT_URI").map_err(|e| format!("REDIRECT_URI not found: {}", e))?;

        eprintln!("Client ID: {}", client_id);
        eprintln!("Redirect URI: {}", redirect_uri);

        Ok(Self {
            client_id,
            client_secret,
            // Fixed URLs to include /v1/ in the path
            auth_url: "https://api.supabase.com/v1/oauth/authorize".to_string(),
            token_url: "https://api.supabase.com/v1/oauth/token".to_string(),
            redirect_uri,
            mgmt_api_base_url: "https://api.supabase.com/v1".to_string(),
        })
    }
}

// --- Structs for API responses and session data ---
#[derive(Debug, Serialize, Deserialize)]
pub struct SupabaseProject {
    pub id: String,
    pub name: String,
    // Add other fields if needed
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OAuthSessionData {
    pub pkce_verifier_secret: Option<String>,
    pub csrf_token_secret: Option<String>,
}

// --- AppState for Axum handlers ---
#[derive(Clone)]
pub struct AppState {
    pub oauth_client: BasicClient,
    pub config: AppConfig,
}

#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    pub code: String,
    pub state: String, // This is the CSRF token from Supabase
}
