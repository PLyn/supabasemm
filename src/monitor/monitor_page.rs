use super::components::{ConnectionStringForm, ServerCommandForm};
use leptos::html::Input;
use leptos::prelude::*;

#[component]
pub fn MonitorPage() -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();
    let (connection_string, set_connection_string) = signal("".to_string());
    let (results, set_results) = signal("Output logs will be displayed here".to_string());

    view! {
        <h2>Supabase Monitor</h2>

        <ConnectionStringForm
            input_ref
            connection_string
            set_connection_string />

        <ServerCommandForm
           input_ref
           label="Check Cache hit Ratio".to_string()
           command="cache-hit".to_string()
           set_results />

        <textarea rows=20 cols=160 bind:value=(results, set_results)></textarea>
    }
}
