use crate::migrate::ConfigItem;
use leptos::prelude::*;

#[component]
pub fn ConfigSelectForm(config_items_rw: [RwSignal<bool>; 6]) -> impl IntoView {
    view!(
        <br />
        <input type="checkbox" name="test" bind:value=config_items_rw[ConfigItem::Auth as usize]/>
        <label>Migrate Auth Config</label>
        <br />

        <br />
        <input type="checkbox" name="test" bind:value=config_items_rw[ConfigItem::Postgrest as usize]/>
        <label>Migrate PostgREST Config</label>
        <br />

        <br />
        <input type="checkbox" name="test" bind:value=config_items_rw[ConfigItem::EdgeFunction as usize]/>
        <label>Migrate Edge Functions</label>
        <br />

        <br />
        <input type="checkbox" name="test" bind:value=config_items_rw[ConfigItem::Secrets as usize]/>
        <label>Migrate Supabase Project Secrets</label>
        <br />

        <br />
        <input type="checkbox" name="test" bind:value=config_items_rw[ConfigItem::Storage as usize]/>
        <label>Migrate Storage config</label>
        <br />

        <br />
        <input type="checkbox" name="test" bind:value=config_items_rw[ConfigItem::Branches as usize]/>
        <label>Migrate branches</label>
        <br />
    )
}
