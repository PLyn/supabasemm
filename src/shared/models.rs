use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct SupabaseProject {
    pub id: String,
    pub name: String,
    pub region: String,
    pub status: String,
    // Add other fields if needed
}
