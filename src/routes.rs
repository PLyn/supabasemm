use crate::home::HomePage;
//use crate::metrics::MetricsPage;
use crate::migrate::MigratePage;
//use crate::monitor::MonitorPage;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/supabasemm.css"/>
        // sets the document title
        <Title text="Supa Migrate and Monitor"/>

        // Header section
        <header class="shadow-md">
            <div class="container mx-auto px-4">
                <div class="flex items-center justify-between py-4">
                    // Title on the left
                    <h1 class="text-2xl font-bold text-gray-800">
                        "Supabase M&M"
                    </h1>

                    <nav class="flex space-x-8">
                        <a
                            href="/"
                            class="text-gray-600 hover:text-gray-900 font-medium transition duration-200 border-b-2 border-transparent hover:border-gray-900 pb-1"
                        >
                            "Home"
                        </a>
                        <a
                            href="/migrate"
                            class="text-gray-600 hover:text-gray-900 font-medium transition duration-200 border-b-2 border-transparent hover:border-gray-900 pb-1"
                        >
                            "Migrate"
                        </a>
                    </nav>

                    <button class="btn btn-primary"
                        on:click=move |_| { window().location().set_href("/connect-supabase/login").unwrap(); }>
                        "Sign in with Supabase"
                    </button>
                </div>
            </div>
        </header>

        // Main content
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("/migrate") view=MigratePage/>
                </Routes>
            </main>
        </Router>
    }
}
