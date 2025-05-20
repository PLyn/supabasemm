use crate::{
    server::server_models::{AppState, OAuthSessionData},
    shared::models::{AUTH_URL, REDIRECT_URL},
};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use oauth2::{CsrfToken, PkceCodeChallenge};
use tower_sessions::Session;

pub async fn login_handler(
    State(app_state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    eprintln!("Session ID at login: {:?}", session.id());

    let access_token_option: Option<String> =
        session.get("supabase_access_token").await.ok().flatten();

    if let Some(access_token) = access_token_option {
        eprintln!("Access Token: {:?}", access_token);
        eprintln!("Existing Supabase access token found in session. Skipping full OAuth flow.");
        return Redirect::to("/connect-supabase/projects").into_response();
    }

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let csrf_token = CsrfToken::new_random();

    let mut url = reqwest::Url::parse(AUTH_URL).expect("Failed to parse auth URL");

    eprintln!("Base auth URL: {}", url.as_str());

    url.query_pairs_mut()
        .append_pair("client_id", &app_state.config.client_id)
        .append_pair("redirect_uri", REDIRECT_URL)
        .append_pair("response_type", "code")
        .append_pair("state", csrf_token.secret())
        .append_pair("code_challenge", &pkce_challenge.as_str())
        .append_pair("code_challenge_method", "S256");

    let constructed_url = url.to_string();
    eprintln!("Full auth URL for redirect: {}", constructed_url);

    let session_data = OAuthSessionData {
        pkce_verifier_secret: Some(pkce_verifier.secret().to_string()),
        csrf_token_secret: Some(csrf_token.secret().to_string()),
    };

    // Store data in session both as a struct and as individual values for redundancy
    if let Err(e) = session.insert("oauth_data", session_data).await {
        eprintln!("Failed to insert oauth_data into session: {:?}", e);
    }

    if let Err(e) = session
        .insert("csrf_token_secret", csrf_token.secret().to_string())
        .await
    {
        eprintln!("Failed to insert csrf_token_secret into session: {:?}", e);
    }

    match session.get::<OAuthSessionData>("oauth_data").await {
        Ok(Some(_)) => eprintln!("Successfully verified oauth_data in session"),
        Ok(None) => eprintln!("WARNING: oauth_data was not found during verification"),
        Err(e) => eprintln!("Error verifying oauth_data in session: {:?}", e),
    }

    eprintln!("PKCE verifier and CSRF token stored in session. Redirecting to Supabase...");
    Redirect::to(&constructed_url).into_response()
}
