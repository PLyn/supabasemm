#[cfg(feature = "ssr")]
use supabasemm::server::*;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum::{routing::get, Router};
    use handlers::{callback_handler, login_handler, projects_handler};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use server_init;
    use supabasemm::routes::*;
    use tower_http::compression::CompressionLayer;

    let (app_state, session_layer) = server_init()?;

    // Set up Leptos integration
    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/connect-supabase/login", get(login_handler))
        .route("/connect-supabase/oauth2/callback", get(callback_handler))
        .route("/connect-supabase/projects", get(projects_handler))
        .with_state(app_state)
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options.clone())
        .layer(session_layer)
        .layer(CompressionLayer::new());

    log!("listening on http://{}", &addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
