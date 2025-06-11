use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        use dotenvy::dotenv;
        use std::env;

        dotenv().ok();

        let client_id = env::var("SUPA_CONNECT_CLIENT_ID")
            .map_err(|e| format!("SUPA_CONNECT_CLIENT_ID not found: {}", e))?;
        let client_secret = env::var("SUPA_CONNECT_CLIENT_SECRET")
            .map_err(|e| format!("SUPA_CONNECT_CLIENT_SECRET not found: {}", e))?;
        let redirect_url = env::var("REDIRECT_URL")
            .map_err(|e| format!("REDIRECT_URL not found: {}", e))?;

        Ok(Self {
            client_id,
            client_secret,
            redirect_url
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OAuthSessionData {
    pub pkce_verifier_secret: Option<String>,
    pub csrf_token_secret: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
}

#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    pub code: String,
    pub state: String,
}
