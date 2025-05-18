use crate::server::server_models::{AppState, OAuthSessionData};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use oauth2::{CsrfToken, PkceCodeChallenge};
use tower_sessions::Session;

// GET /connect-supabase/login

pub async fn login_handler(
    State(app_state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    eprintln!("Login handler called");
    eprintln!("Session ID at login: {:?}", session.id());

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let csrf_token = CsrfToken::new_random();

    // Create the full auth URL with the Supabase domain
    // Make sure we're using the full absolute URL, not a relative one
    let mut url =
        reqwest::Url::parse(&app_state.config.auth_url).expect("Failed to parse auth URL");

    eprintln!("Base auth URL: {}", url.as_str());

    // Add query parameters for OAuth flow
    url.query_pairs_mut()
        .append_pair("client_id", &app_state.config.client_id)
        .append_pair("redirect_uri", &app_state.config.redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("state", csrf_token.secret())
        .append_pair("code_challenge", &pkce_challenge.as_str())
        .append_pair("code_challenge_method", "S256");
    //.append_pair("scope", "read:organizations read:projects");

    let auth_url = url.to_string();
    eprintln!("Full auth URL for redirect: {}", auth_url);

    let session_data = OAuthSessionData {
        pkce_verifier_secret: Some(pkce_verifier.secret().to_string()),
        csrf_token_secret: Some(csrf_token.secret().to_string()),
    };

    // Store data in session both as a struct and as individual values for redundancy
    if let Err(e) = session.insert("oauth_data", session_data).await {
        eprintln!("Failed to insert oauth_data into session: {:?}", e);
    }

    // Also store as individual keys as a backup
    if let Err(e) = session
        .insert("pkce_verifier_secret", pkce_verifier.secret().to_string())
        .await
    {
        eprintln!(
            "Failed to insert pkce_verifier_secret into session: {:?}",
            e
        );
    }

    if let Err(e) = session
        .insert("csrf_token_secret", csrf_token.secret().to_string())
        .await
    {
        eprintln!("Failed to insert csrf_token_secret into session: {:?}", e);
    }

    // Attempt to immediately read back to verify data was stored
    match session.get::<OAuthSessionData>("oauth_data").await {
        Ok(Some(_)) => eprintln!("Successfully verified oauth_data in session"),
        Ok(None) => eprintln!("WARNING: oauth_data was not found during verification"),
        Err(e) => eprintln!("Error verifying oauth_data in session: {:?}", e),
    }

    eprintln!("PKCE verifier and CSRF token stored in session. Redirecting to Supabase...");
    Redirect::to(&auth_url)
}
