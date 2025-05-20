use crate::shared::call_server_command;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn ServerCommandForm(
    input_ref: NodeRef<Input>,
    label: String,
    command: String,
    set_results: WriteSignal<String>,
) -> impl IntoView {
    view!(
        <br />
        <button on:click=move |_| {
            let run_command = command.clone();
            spawn_local(async move {
                let connection_string = input_ref.get_untracked().unwrap().value();
                let results = call_server_command(run_command, connection_string)
                    .await
                    .unwrap_or_else(|e| e.to_string());

                set_results.set(results);
            });
        }>
        {label}
        </button>
        <br />
    )
}
