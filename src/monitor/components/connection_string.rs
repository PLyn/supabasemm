use leptos::html::Input;
use leptos::prelude::*;

#[component]
pub fn ConnectionStringForm(
    input_ref: NodeRef<Input>,
    connection_string: ReadSignal<String>,
    set_connection_string: WriteSignal<String>,
) -> impl IntoView {
    view!(
        <br />
        <h2>Set Database connection string</h2>
        <input
            node_ref=input_ref
            placeholder="Type something here."
            bind:value=(connection_string, set_connection_string) />
        <br />
    )
}
