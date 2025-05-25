use crate::migrate::migrate_page::{ConfigNames, ConfigState};
use leptos::prelude::*;

#[component]
pub fn ConfigSelectForm(config_state_rw: RwSignal<ConfigState>) -> impl IntoView {
    view!(
        <div class="flex flex-col w-1/2 mx-auto">
            {config_state_rw.get().items.iter()
                .zip(config_state_rw.get().values.iter())
                .map(|(item, value_signal)| view! {
                    <div class="my-2">
                        <input 
                            class="checkbox checkbox-info"
                            type="checkbox" 
                            name=item.name
                            bind:value=*value_signal
                        />
                        <label class="ml-2">{item.label}</label>
                    </div>
                })
                .collect_view()}
        </div>
    )
}