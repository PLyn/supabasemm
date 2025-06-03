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
        <div class="flex flex-col gap-4 py-2 mt-4 px-4">

            <h3 class="text-3xl font-bold">"Check database Cache hit ratio"</h3>
            <label>
            "Check the database cache hit ratio to determine if this is impacting your database performance.
            This will run the Supabase CLI command aganist your Database. See more details about the "
                <a href="https://supabase.com/docs/reference/cli/supabase-inspect-db-cache-hit" class="link link-info">
                "Supabase CLI command"</a>" in the docs"</label>

            <div class="mockup-code w-full mb-8">
                <pre data-prefix="$"><code>"supabase inspect db cache-hit --db-url <YOUR_DB_CONNECTION_STRING>"</code></pre>
            </div>

            <div class="flex flex-row">
                <label class="floating-label w-1/2 mr-2">
                    <input type="text" class="input input-primary input-md w-full"
                        node_ref=input_ref
                        placeholder="Database connection string"
                        bind:value=(connection_string, set_connection_string) />
                    <span>Database connection string</span>
                </label>

                <button class="btn btn-primary px-4 ml-2"
                on:click=move |_| {
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
            </div>

            <textarea placeholder="Output logs will be displayed here" class="textarea w-full" rows=30 bind:value=(results, set_results)></textarea>
        </div>
    }
}
