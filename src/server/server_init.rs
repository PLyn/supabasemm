use super::server_models::{AppConfig, AppState};
use tower_sessions::{MemoryStore, SessionManagerLayer};

pub fn server_init(
) -> Result<(AppState, SessionManagerLayer<MemoryStore>), Box<dyn std::error::Error>> {
    let app_config = AppConfig::from_env()?;

    let app_state = AppState {
        config: app_config.clone(),
    };

    let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(tower_sessions::cookie::SameSite::Lax);

    Ok((app_state, session_layer))
}
