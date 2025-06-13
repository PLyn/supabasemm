use crate::shared::models::{Project, ProjectConfig};
use crate::shared::server_functions::{check_auth_status, get_projects};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
      <h1>"Welcome to Leptos!"</h1>
      <button on:click=on_click>"Click Me: " {count}</button>
      <div>"Test area below"</div>
      <h1>"Supabase Connect Example (Rust/Axum)"</h1>
      <h1 class="text-3xl font-bold text-red-600">Hello world!</h1>
      <button class="btn btn-primary">test button</button>
      <p>"This is an example of implementing Supabase OAuth integration."</p>
      <p>
         <a href="/connect-supabase/login" rel="external">"Login with Supabase"</a>
      </p>
      <p>
         <a href="/connect-supabase/projects" rel="external">"View Projects (if authenticated)"</a>
      </p>
    }
}
