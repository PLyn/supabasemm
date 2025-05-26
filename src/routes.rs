use crate::home::HomePage;
use crate::migrate::MigratePage;
use crate::monitor::MonitorPage;
use crate::metrics::MetricsPage;
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
        <header class="bg-white shadow-md">
            <div class="container mx-auto px-4">
                <div class="flex items-center justify-between py-4">
                    // Title on the left
                    <h1 class="text-2xl font-bold text-gray-800">
                        "Supabase M&M"
                    </h1>
                    
                    // Navigation menu in the middle
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
                        <a 
                            href="/monitor" 
                            class="text-gray-600 hover:text-gray-900 font-medium transition duration-200 border-b-2 border-transparent hover:border-gray-900 pb-1"
                        >
                            "Monitor"
                        </a>
                        <a 
                            href="/metrics" 
                            class="text-gray-600 hover:text-gray-900 font-medium transition duration-200 border-b-2 border-transparent hover:border-gray-900 pb-1"
                        >
                            "Metrics"
                        </a>
                    </nav>
                    
                    // Sign in button on the right
                    <button 
                        class="bg-green-600 hover:bg-green-700 text-white font-medium py-2 px-4 rounded-md transition duration-200"
                        on:click=move |_| { window().location().set_href("/connect-supabase/login").unwrap(); }
                    >
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
                    <Route path=StaticSegment("/monitor") view=MonitorPage/>
                    <Route path=StaticSegment("/metrics") view=MetricsPage/>
                </Routes>
            </main>
        </Router>
    }
}