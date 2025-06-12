use leptos::prelude::*;
#[cfg(feature = "ssr")]
use tower_sessions::Session;

#[cfg(feature = "ssr")]
pub async fn check_auth_status(session: Session) -> Result<bool, ServerFnError> {
    let access_token: Option<String> = session.get("supabase_access_token").await?;
    eprintln!("Auth status: {}", access_token.is_some());
    Ok(access_token.is_some())
}
