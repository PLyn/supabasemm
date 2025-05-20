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
    // In a real application, you'd also ideally store and check an 'expires_at' timestamp
    // associated with the access token, as discussed previously, to ensure it's not merely present but also valid.
    // For this example, we'll assume merely its presence implies validity for simplicity,
    // but a robust solution would check expiration and potentially attempt refresh.

    if let Some(access_token) = access_token_option {
        // You might want to add a light validation here, e.g., if you can decode the JWT
        // and check its expiration (`exp` claim) without calling Supabase.
        // For a quick check, just having the token is often enough to skip.
        // The actual validation (e.g., calling a Supabase API) will happen when the user tries
        // to access a protected resource.
        eprintln!("Access Token: {:?}", access_token);
        eprintln!("Existing Supabase access token found in session. Skipping full OAuth flow.");
        // If an access token exists, redirect to a dashboard or projects page
        // instead of initiating a new OAuth flow.
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
