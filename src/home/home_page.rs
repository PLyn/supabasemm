use leptos::html::Input;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <div>"Test area below"</div>
        <h1>"Supabase Connect Example (Rust/Axum)"</h1>
         <p>"This is an example of implementing Supabase OAuth integration."</p>
         <p>
            <a href="/connect-supabase/login" rel="external">"Login with Supabase"</a>
         </p>
         <p>
            <a href="/connect-supabase/projects" rel="external">"View Projects (if authenticated)"</a>
         </p>
    }
}
