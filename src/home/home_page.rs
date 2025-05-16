use super::check_cache_hit_ratio;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    let input_ref = NodeRef::<Input>::new();

    let (shout_result, set_shout_result) = signal("Submit".to_string());
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <div>"Test area below"</div>
        <h1>Currently set Database connection string</h1>

        <input node_ref=input_ref placeholder="Type something here." bind:value=(shout_result, set_shout_result) />
        <button on:click=move |_| {
            spawn_local(async move {
                let input_value = input_ref.get().unwrap().value();
                let uppercase_text = check_cache_hit_ratio(input_value).await.unwrap_or_else(|e| e.to_string());
                set_shout_result.set(uppercase_text);
            });
        }>
        {shout_result}
        </button>
    }
}
