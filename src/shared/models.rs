use serde::{Deserialize, Serialize};

pub static AUTH_URL: &str = "https://api.supabase.com/v1/oauth/authorize";
pub static TOKEN_URL: &str = "https://api.supabase.com/v1/oauth/token";
pub static MGMT_API_BASE_URL: &str = "https://api.supabase.com/v1";

pub static REDIRECT_URL: &str = "http://localhost:3000/connect-supabase/oauth2/callback";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub region: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffEntry {
    pub config_type: String,
    pub key: String,
    pub source_value: String, 
    pub dest_value: String,   
}