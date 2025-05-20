use leptos::prelude::*;

#[component]
pub fn MigratePage() -> impl IntoView {
    let (auth_status, set_auth_status) = signal("".to_string());
    view! {
        <h2>Supabase Migrate project configuration</h2>
        <h2>Authorize</h2>

        <button on:click=move |_| { window().location().set_href("/connect-supabase/login").unwrap(); }>
        Sign in with Supabase
        </button>
    }
}
