use leptos::prelude::*;

#[server]
pub async fn check_auth_status() -> Result<bool, ServerFnError> {
    use leptos_axum::extract;
    use tower_sessions::Session;

    let session: Session = extract().await?;
    let access_token: Option<String> = session.get("supabase_access_token").await?;
    eprintln!("Auth status: {}", access_token.is_some());
    Ok(access_token.is_some())
}
