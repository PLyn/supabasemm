use leptos::prelude::*;

#[component]
pub fn MigratePage() -> impl IntoView {
    let (auth_status, set_auth_status) = signal("".to_string());
    let (auth_config, set_auth_config) = signal(false);
    let (postgrest_config, set_postgrest_config) = signal(false);
    let (edge_fn_config, set_edge_fn_config) = signal(false);
    let (secrets_config, set_secrets_config) = signal(false);
    let (storage_config, set_storage_config) = signal(false);
    let (branches_config, set_branches_config) = signal(false);

    let (source_project, set_source_project) = signal("".to_string());
    let (dest_project, set_dest_project) = signal("".to_string());

    let (step1, set_step1) = signal(true);
    let (step2, set_step2) = signal(false);
    let (step3, set_step3) = signal(false);

    view! {
        <h2>Supabase Migrate project configuration</h2>
        <h2>Authorize</h2>

        <Show
          when=move || { step1.get() }
        >
            <br />
            <button on:click=move |_| { window().location().set_href("/connect-supabase/login").unwrap(); }>
            Sign in with Supabase
            </button>
            <br />
        </Show>

        <Show
          when=move || { step2.get() }
        >
            <br />
            <label>Select Source Project</label>
            <select
            on:change:target=move |ev| {
                set_source_project.set(ev.target().value().parse().unwrap());
            }
            prop:value=move || source_project.get().to_string()
            >
            <option value="a">"a"</option>
            <option value="b">"b"</option>
            <option value="c">"c"</option>
            </select>
            <br />

            <br />
            <label>Select Destination Project</label>
            <select
            on:change:target=move |ev| {
                set_dest_project.set(ev.target().value().parse().unwrap());
            }
            prop:value=move || dest_project.get().to_string()
            >
            <option value="0">"0"</option>
            <option value="1">"1"</option>
            <option value="2">"2"</option>
            </select>
            <br />
        </Show>

        <Show
          when=move || { step3.get() }
        >
            <br />
            <input type="checkbox" name="test" bind:value=(auth_config, set_auth_config)/>
            <label>Migrate Auth Config</label>
            <br />

            <br />
            <input type="checkbox" name="test" bind:value=(postgrest_config, set_postgrest_config)/>
            <label>Migrate PostgREST Config</label>
            <br />

            <br />
            <input type="checkbox" name="test" bind:value=(edge_fn_config, set_edge_fn_config)/>
            <label>Migrate Edge Functions</label>
            <br />

            <br />
            <input type="checkbox" name="test" bind:value=(secrets_config, set_secrets_config)/>
            <label>Migrate Supabase Project Secrets</label>
            <br />

            <br />
            <input type="checkbox" name="test" bind:value=(storage_config, set_storage_config)/>
            <label>Migrate Storage config</label>
            <br />

            <br />
            <input type="checkbox" name="test" bind:value=(branches_config, set_branches_config)/>
            <label>Migrate branches</label>
            <br />
        </Show>

        <Show
          when=move || { !step1.get() }
        >
        <br />
        <button on:click=move |_| {
            if step2.get() {
                set_step1.set(true);
                set_step2.set(false);
                set_step3.set(false);
            } else if step3.get() {
                set_step1.set(false);
                set_step2.set(true);
                set_step3.set(false);
            }
        }>Back</button>
        <br />
        </Show>

        <Show
          when=move || { !step3.get() }
        >
        <br />
        <button on:click=move |_| {
            if step1.get() {
                set_step1.set(false);
                set_step2.set(true);
                set_step3.set(false);
            } else if step2.get() {
                set_step1.set(false);
                set_step2.set(false);
                set_step3.set(true);
            }
        }>Confirm</button>
        <br />
        </Show>

    }
}
