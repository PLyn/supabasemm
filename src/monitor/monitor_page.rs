use leptos::html::Input;
use leptos::prelude::*;
use leptos::task::spawn_local;

use super::call_supabase_cli;

#[component]
pub fn MonitorPage() -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();
    let (connection_string, set_connection_string) = signal("".to_string());
    let (results, set_results) = signal("Output logs will be displayed here".to_string());

    view! {
        <h2>Supabase Monitor</h2>

        <br />
        <h2>Set Database connection string</h2>
        <input
            node_ref=input_ref
            placeholder="Type something here."
            bind:value=(connection_string, set_connection_string) />
        <br />

        <button on:click=move |_| {
            spawn_local(async move {
                let connection_string = input_ref.get_untracked().unwrap().value();
                let results = call_supabase_cli("cache-hit".to_string(), connection_string)
                    .await
                    .unwrap_or_else(|e| e.to_string());

                set_results.set(results);
            });
        }>
        "Check Cache hit Ratio"
        </button>
        <br />

        <textarea rows=20 cols=160 bind:value=(results, set_results)></textarea>
    }
}
