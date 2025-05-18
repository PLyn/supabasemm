#[cfg(feature = "ssr")]
mod server;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum::{routing::get, Router};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use oauth2::{basic::BasicClient, ClientId};
    use server::server_models::{AppConfig, AppState};
    use supabasemm::routes::*;
    use tower_http::compression::CompressionLayer;
    use tower_sessions::{MemoryStore, SessionManagerLayer};

    use server::handlers::{callback_handler, login_handler, projects_handler};

    let app_config = AppConfig::from_env()?;

    // No longer using the BasicClient directly - implementing auth manually
    let app_state = AppState {
        oauth_client: BasicClient::new(ClientId::new(app_config.client_id.clone())),
        config: app_config.clone(),
    };

    // Create a more robust session store with explicit configuration
    let session_store = MemoryStore::default();

    // Configure session with longer duration and debug-friendly settings
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to false for HTTP, true for HTTPS
        .with_same_site(tower_sessions::cookie::SameSite::Lax);

    // Set up Leptos integration
    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Add routes
    let app = Router::new()
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
