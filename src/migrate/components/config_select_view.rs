use leptos::prelude::*;

#[component]
pub fn ConfigSelectForm(
    auth_config_rw: RwSignal<bool>,
    postgrest_config_rw: RwSignal<bool>,
    edge_fn_config_rw: RwSignal<bool>,
    secrets_config_rw: RwSignal<bool>,
    storage_config_rw: RwSignal<bool>,
    branches_config_rw: RwSignal<bool>,
) -> impl IntoView {
    view!(
        <br />
        <input type="checkbox" name="test" bind:value=auth_config_rw/>
        <label>Migrate Auth Config</label>
        <br />

        <br />
        <input type="checkbox" name="test" bind:value=postgrest_config_rw/>
        <label>Migrate PostgREST Config</label>
        <br />

        <br />
        <input type="checkbox" name="test" bind:value=edge_fn_config_rw/>
        <label>Migrate Edge Functions</label>
        <br />

        <br />
        <input type="checkbox" name="test" bind:value=secrets_config_rw/>
        <label>Migrate Supabase Project Secrets</label>
        <br />

        <br />
        <input type="checkbox" name="test" bind:value=storage_config_rw/>
        <label>Migrate Storage config</label>
        <br />

        <br />
        <input type="checkbox" name="test" bind:value=branches_config_rw/>
        <label>Migrate branches</label>
        <br />
    )
}
