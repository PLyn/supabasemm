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
        <div class="flex px-4">
            <h1>"Supabase M&M"</h1>
            <a href="/">Home</a>
            <div></div>
            <a href="/migrate">Migrate</a>
            <div></div>
            <a href="/monitor">Monitor</a>
            <div></div>
            <a href="/metrics">Metrics</a>
        </div>

        // content for this welcome page
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
