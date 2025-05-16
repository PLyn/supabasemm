#[cfg(feature = "ssr")]
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};
#[cfg(feature = "ssr")]
use dotenvy::dotenv; // For loading .env file
#[cfg(feature = "ssr")]
use oauth2::{basic::BasicClient, ClientId, CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
#[cfg(feature = "ssr")]
use reqwest::header::{ACCEPT, AUTHORIZATION};
#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use std::env;

// Session management
#[cfg(feature = "ssr")]
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};

#[cfg(feature = "ssr")]
use leptos::logging::log;
#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};
#[cfg(feature = "ssr")]
use supabasemm::routes::*;
#[cfg(feature = "ssr")]
use tower_http::compression::CompressionLayer; // Import CompressionLayer

// GET /connect-supabase
#[cfg(feature = "ssr")]
async fn root_handler() -> impl IntoResponse {
    Html(
        "<h1>Supabase Connect Example (Rust/Axum)</h1>\
         <p>This is an example of implementing Supabase OAuth integration.</p>\
         <p><a href=\"/connect-supabase/login\">Login with Supabase</a></p>\
         <p><a href=\"/connect-supabase/projects\">View Projects (if authenticated)</a></p>",
    )
}

// --- Configuration Struct ---
#[cfg(feature = "ssr")]
#[derive(Clone)]
struct AppConfig {
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
    redirect_uri: String,
    mgmt_api_base_url: String,
}

#[cfg(feature = "ssr")]
impl AppConfig {
    fn from_env() -> Result<Self, String> {
        eprintln!("Loading environment variables for OAuth2 configuration...");

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
#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
struct SupabaseProject {
    id: String,
    name: String,
    // Add other fields if needed
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, Default)]
struct OAuthSessionData {
    pkce_verifier_secret: Option<String>,
    csrf_token_secret: Option<String>,
}

// --- AppState for Axum handlers ---
#[cfg(feature = "ssr")]
#[derive(Clone)]
struct AppState {
    oauth_client: BasicClient,
    config: AppConfig,
}

// GET /connect-supabase/login
#[cfg(feature = "ssr")]
async fn login_handler(State(app_state): State<AppState>, session: Session) -> impl IntoResponse {
    eprintln!("Login handler called");
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

    session
        .insert("oauth_data", session_data)
        .await
        .expect("Failed to insert oauth_data into session");
    eprintln!("PKCE verifier and CSRF token stored in session. Redirecting to Supabase...");
    Redirect::to(&auth_url)
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct CallbackParams {
    code: String,
    state: String, // This is the CSRF token from Supabase
}

// GET /connect-supabase/oauth2/callback
#[cfg(feature = "ssr")]
async fn callback_handler(
    Query(params): Query<CallbackParams>,
    State(app_state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    eprintln!(
        "OAuth callback received. Code: {}, State: {}",
        params.code, params.state
    );

    let oauth_data: OAuthSessionData = session
        .get("oauth_data")
        .await
        .expect("Failed to get oauth_data from session")
        .unwrap_or_default();

    session
        .remove::<OAuthSessionData>("oauth_data")
        .await
        .expect("Failed to remove oauth_data from session");

    // Error handling for missing PKCE verifier
    if oauth_data.pkce_verifier_secret.is_none() {
        eprintln!("No PKCE verifier found in session");
        return Html(
            "<h1>Error</h1><p>No PKCE verifier found in session. Please try logging in again.</p>"
                .to_string(),
        );
    }
    let pkce_verifier_secret = oauth_data.pkce_verifier_secret.unwrap();

    // Error handling for missing CSRF token
    if oauth_data.csrf_token_secret.is_none() {
        eprintln!("No CSRF token found in session");
        return Html(
            "<h1>Error</h1><p>No CSRF token found in session. Please try logging in again.</p>"
                .to_string(),
        );
    }
    let original_csrf_secret = oauth_data.csrf_token_secret.unwrap();

    if original_csrf_secret != params.state {
        eprintln!(
            "CSRF token mismatch. Expected: {}, Got: {}",
            original_csrf_secret, params.state
        );
        return Html(
            "<h1>Error</h1><p>CSRF token mismatch. Please try logging in again.</p>".to_string(),
        );
    }
    eprintln!("CSRF token verified.");

    let pkce_verifier = PkceCodeVerifier::new(pkce_verifier_secret);

    // Use the reqwest client directly for token exchange
    let client = reqwest::Client::new();
    let token_url = &app_state.config.token_url;

    let params = [
        ("client_id", app_state.config.client_id.as_str()),
        ("client_secret", app_state.config.client_secret.as_str()),
        ("code", params.code.as_str()),
        ("code_verifier", pkce_verifier.secret()),
        ("grant_type", "authorization_code"),
        ("redirect_uri", app_state.config.redirect_uri.as_str()),
    ];

    let response = match client.post(token_url).form(&params).send().await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to exchange token: {:?}", e);
            return Html(format!(
                "<h1>Error</h1><p>Failed to exchange token: {}. Please try logging in again.</p>",
                e
            ));
        }
    };

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Could not read error body".to_string());
        eprintln!("Failed to exchange token (HTTP {}): {}", status, error_text);
        return Html(format!(
            "<h1>Error</h1><p>Failed to exchange token: HTTP {} - {}. Please try logging in again.</p>",
            status, error_text
        ));
    }

    #[derive(Deserialize)]
    struct TokenResponse {
        access_token: String,
        refresh_token: Option<String>,
        // Add other fields if needed
    }

    let token_data = match response.json::<TokenResponse>().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse token response: {:?}", e);
            return Html(format!(
                "<h1>Error</h1><p>Failed to parse token response: {}. Please try logging in again.</p>",
                e
            ));
        }
    };

    // Store token in session
    session
        .insert("supabase_access_token", token_data.access_token.clone())
        .await
        .expect("Failed to store access token in session");

    if let Some(refresh_token) = token_data.refresh_token {
        eprintln!(
            "Refresh Token received (store securely if needed for long-term use): {}",
            refresh_token
        );
        // For a real app, store refresh_token securely, associated with the user/integration
    }

    // Convert the redirect to HTML with a page that will redirect
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta http-equiv="refresh" content="0;url=/connect-supabase/projects">
            <title>Redirecting...</title>
        </head>
        <body>
            <p>Authentication successful! Redirecting to your projects...</p>
            <p>If you are not redirected, <a href="/connect-supabase/projects">click here</a>.</p>
        </body>
        </html>
        "#
    ))
}

// GET /connect-supabase/projects (protected - requires token)
#[cfg(feature = "ssr")]
async fn projects_handler(
    State(app_state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    eprintln!("Projects handler called");
    let token_option: Option<String> = session
        .get("supabase_access_token")
        .await
        .expect("Failed to get access token from session");

    if let Some(token) = token_option {
        eprintln!("Access token found in session.");
        let client = reqwest::Client::new();
        let url = format!("{}/projects", app_state.config.mgmt_api_base_url);

        eprintln!("Fetching projects from: {}", url);
        let result = client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/json")
            .send()
            .await;

        match result {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<SupabaseProject>>().await {
                        Ok(projects) => {
                            let project_list_html = projects
                                .iter()
                                .map(|p| format!("<li>ID: {}, Name: {}</li>", p.id, p.name))
                                .collect::<String>();
                            Html(format!("<h1>Your Supabase Projects</h1><ul>{}</ul><p><a href=\"/connect-supabase\">Back to Home</a></p>", project_list_html))
                        }
                        Err(e) => {
                            eprintln!("Failed to parse projects JSON: {}", e);
                            Html(format!("<p>Error parsing project data: {}.</p><p><a href=\"/connect-supabase\">Back to Home</a></p>", e))
                        }
                    }
                } else {
                    let status = response.status();
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Could not read error body".to_string());
                    eprintln!("Failed to fetch projects (HTTP {}): {}", status, error_text);
                    Html(format!("<p>Error fetching projects: HTTP {} - {}.</p><p><a href=\"/connect-supabase\">Back to Home</a></p>", status, error_text))
                }
            }
            Err(e) => {
                eprintln!("Network error while fetching projects: {}", e);
                Html(format!("<p>Network error fetching projects: {}.</p><p><a href=\"/connect-supabase\">Back to Home</a></p>", e))
            }
        }
    } else {
        eprintln!("No access token found in session. Redirecting to login.");
        // Return HTML with a redirect instead of using Redirect
        Html(format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta http-equiv="refresh" content="0;url=/connect-supabase/login">
                <title>Redirecting to login...</title>
            </head>
            <body>
                <p>No access token found. Redirecting to login page...</p>
                <p>If you are not redirected, <a href="/connect-supabase/login">click here</a>.</p>
            </body>
            </html>
            "#
        ))
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file

    let app_config = AppConfig::from_env()?;

    // No longer using the BasicClient directly - implementing auth manually
    let app_state = AppState {
        oauth_client: BasicClient::new(ClientId::new(app_config.client_id.clone())),
        config: app_config.clone(),
    };

    let session_store = MemoryStore::default();

    // Fix the session layer configuration
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false); // Set to true if using HTTPS

    // Set up Leptos integration
    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Add routes
    let app = Router::new()
        .route("/connect-supabase", get(root_handler))
        .route("/connect-supabase/login", get(login_handler))
        .route("/connect-supabase/oauth2/callback", get(callback_handler))
        .route("/connect-supabase/projects", get(projects_handler))
        .layer(session_layer)
        .with_state(app_state)
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();

            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options.clone())
        .layer(CompressionLayer::new());

    // Run our app with hyper - updated for Axum 0.8
    log!("listening on http://{}", &addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    // Fixed the serve method to work with Axum 0.8
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
