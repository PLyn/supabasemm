use super::check_cache_hit_ratio;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Renders the home page of your application.
#[component]
pub fn MigratePage() -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();

    let (shout_result, set_shout_result) = signal("Submit".to_string());
    view! {
        <h2>Supabase Migrate</h2>
        <h2>Currently set Database connection string</h2>

        <input node_ref=input_ref placeholder="Type something here." bind:value=(shout_result, set_shout_result) />
        <button on:click=move |_| {
            spawn_local(async move {
                let uppercase_text = check_cache_hit_ratio("test".to_string()).await.unwrap_or_else(|e| e.to_string());
                set_shout_result.set(uppercase_text);
            });
        }>
        Run Command
        </button>
    }
}
