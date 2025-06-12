use crate::migrate::functions::generate_preview;
use crate::migrate::migrate_page::{ConfigItems, ViewSteps, CONFIG_ITEM_COUNT};
use crate::shared::models::ProjectConfig;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn ConfigSelectView(
    config_items_rw: [RwSignal<bool>; CONFIG_ITEM_COUNT],
    source_id_rw: RwSignal<String>,
    dest_id_rw: RwSignal<String>,
    results_rw: RwSignal<Vec<ProjectConfig>>,
    current_step_rw: RwSignal<ViewSteps>,
) -> impl IntoView {
    view! {
            <div class="max-w-screen-sm mx-auto w-full p-4 flex flex-col items-center">
                <button class="btn btn-secondary my-4" on:click=move |_| { current_step_rw.set(ViewSteps::Projects); }>"Back"</button>
                <div class="w-full max-w-xs mx-auto">
                    <label class="flex items-center justify-between py-2 w-full cursor-pointer">
                        <span>{format!("{:?}", ConfigItems::Auth)}</span>
                        <input
                            class="checkbox checkbox-success"
                            type="checkbox"
                            name={format!("{:?}", ConfigItems::Auth)}
                            bind:checked=config_items_rw[ConfigItems::Auth as usize]
                        />
                    </label>
                    <div class="divider"></div>
                    <label class="flex items-center justify-between py-2 w-full cursor-pointer">
                        <span>{format!("{:?}", ConfigItems::Postgrest)}</span>
                        <input
                            class="checkbox checkbox-success"
                            type="checkbox"
                            name={format!("{:?}", ConfigItems::Postgrest)}
                            bind:checked=config_items_rw[ConfigItems::Postgrest as usize]
                        />
                    </label>
                    <div class="divider"></div>
                    <label class="flex items-center justify-between py-2 w-full cursor-pointer">
                        <span>{format!("{:?}", ConfigItems::EdgeFunctions)}</span>
                        <input
                            class="checkbox checkbox-success"
                            type="checkbox"
                            name={format!("{:?}", ConfigItems::EdgeFunctions)}
                            bind:checked=config_items_rw[ConfigItems::EdgeFunctions as usize]
                        />
                    </label>
                    <div class="divider"></div>
                    <label class="flex items-center justify-between py-2 w-full cursor-pointer">
                        <span>{format!("{:?}", ConfigItems::Secrets)}</span>
                        <input
                            class="checkbox checkbox-success"
                            type="checkbox"
                            name={format!("{:?}", ConfigItems::Secrets)}
                            bind:checked=config_items_rw[ConfigItems::Secrets as usize]
                        />
                    </label>
                    <div class="divider"></div>
                    <label class="flex items-center justify-between py-2 w-full cursor-pointer">
                        <span>{format!("{:?}", ConfigItems::Storage)}</span>
                        <input
                            class="checkbox checkbox-success"
                            type="checkbox"
                            name={format!("{:?}", ConfigItems::Storage)}
                            bind:checked=config_items_rw[ConfigItems::Storage as usize]
                        />
                    </label>
                    <div class="divider"></div>
                    <label class="flex items-center justify-between py-2 w-full cursor-pointer">
                        <span>{format!("{:?}", ConfigItems::Postgres)}</span>
                        <input
                            class="checkbox checkbox-success"
                            type="checkbox"
                            name={format!("{:?}", ConfigItems::Postgres)}
                            bind:checked=config_items_rw[ConfigItems::Postgres as usize]
                        />
                    </label>
                    <div class="divider"></div>
                    <label class="flex items-center justify-between py-2 w-full cursor-pointer">
                        <span>{format!("{:?}", ConfigItems::Branches)}</span>
                        <input
                            class="checkbox checkbox-success"
                            type="checkbox"
                            name={format!("{:?}", ConfigItems::Branches)}
                            bind:checked=config_items_rw[ConfigItems::Branches as usize]
                        />
                    </label>
                </div>
                <Show when=move || config_items_rw.iter().any(|signal| signal.get()) >
                    <button class="btn btn-primary mt-4"
                        on:click=move |_| {
                            current_step_rw.set(ViewSteps::Loading);
                            spawn_local(async move {
                                let project_config_option = generate_preview(source_id_rw.get(), dest_id_rw.get(), config_items_rw).await;
                                match project_config_option {
                                    Ok(project_config) => results_rw.set(project_config),
                                    Err(_) => results_rw.set(Vec::new()),
                                }
                                current_step_rw.set(ViewSteps::Preview);
                            }); }>
                        "Preview Changes"
                    </button>
                </Show>
            </div>
    }
}
