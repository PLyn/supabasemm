use super::check_cache_hit_ratio;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Renders the home page of your application.
#[component]
pub fn MonitorPage() -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();
    let (conn_string, set_conn_string) = signal("".to_string());
    let (result_logs, set_result_logs) = signal("Output logs will be displayed here".to_string());
    view! {
        <h2>Supabase Monitor</h2>
        <h2>Set Database connection string</h2>

        <input node_ref=input_ref placeholder="Type something here." bind:value=(conn_string, set_conn_string) />
        <br />
        <button on:click=move |_| {
            spawn_local(async move {
                let input_value = input_ref.get().unwrap().value();
                let uppercase_text = check_cache_hit_ratio(input_value).await.unwrap_or_else(|e| e.to_string());
                set_result_logs.set(uppercase_text);
            });
        }>
        Check Cache hit Ratio
        </button>
        <br />
        <textarea rows=5 cols=40 bind:value=(result_logs, set_result_logs)></textarea>
    }
}
