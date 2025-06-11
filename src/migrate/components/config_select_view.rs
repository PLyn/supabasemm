use crate::migrate::functions::generate_preview;
use crate::migrate::migrate_page::{ConfigItems, ViewSteps, CONFIG_ITEM_COUNT};
use crate::shared::models::ProjectConfig;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn ConfigSelectView(
    config_items_rw: [RwSignal<bool>; CONFIG_ITEM_COUNT],
    source_project_rw: RwSignal<String>,
    dest_project_rw: RwSignal<String>,
    results_rw: RwSignal<Vec<ProjectConfig>>,
    current_step_rw: RwSignal<ViewSteps>,
) -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center">
            <div class="max-w-screen-sm mx-auto w-full p-4"> // Added p-4 for some padding
                <div class="flex flex-col items-center">
                    <button class="btn btn-secondary my-4" on:click=move |_| { current_step_rw.set(ViewSteps::Projects); }>"Back"</button>

                    <table class="table table-fixed w-full">
                        <colgroup>
                            <col style="width: 40%;" />
                            <col style="width: 60%;" />
                        </colgroup>
                        <tbody>
                            <tr>
                                <td class="text-left">
                                    <label class="mr-2">{format!("{:?}", ConfigItems::Auth)}</label>
                                </td>
                                <td class="text-right">
                                    <input
                                        class="checkbox checkbox-info"
                                        type="checkbox"
                                        name={format!("{:?}", ConfigItems::Auth)}
                                        bind:checked=config_items_rw[ConfigItems::Auth as usize]
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-left">
                                    <label class="mr-2">{format!("{:?}", ConfigItems::Postgrest)}</label>
                                </td>
                                <td class="text-right">
                                    <input
                                        class="checkbox checkbox-info"
                                        type="checkbox"
                                        name={format!("{:?}", ConfigItems::Postgrest)}
                                        bind:checked=config_items_rw[ConfigItems::Postgrest as usize]
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-left">
                                    <label class="mr-2">{format!("{:?}", ConfigItems::EdgeFunctions)}</label>
                                </td>
                                <td class="text-right">
                                    <input
                                        class="checkbox checkbox-info"
                                        type="checkbox"
                                        name={format!("{:?}", ConfigItems::EdgeFunctions)}
                                        bind:checked=config_items_rw[ConfigItems::EdgeFunctions as usize]
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-left">
                                    <label class="mr-2">{format!("{:?}", ConfigItems::Secrets)}</label>
                                </td>
                                <td class="text-right">
                                    <input
                                        class="checkbox checkbox-info"
                                        type="checkbox"
                                        name={format!("{:?}", ConfigItems::Secrets)}
                                        bind:checked=config_items_rw[ConfigItems::Secrets as usize]
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-left">
                                    <label class="mr-2">{format!("{:?}", ConfigItems::Storage)}</label>
                                </td>
                                <td class="text-right">
                                    <input
                                        class="checkbox checkbox-info"
                                        type="checkbox"
                                        name={format!("{:?}", ConfigItems::Storage)}
                                        bind:checked=config_items_rw[ConfigItems::Storage as usize]
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-left">
                                    <label class="mr-2">{format!("{:?}", ConfigItems::Postgres)}</label>
                                </td>
                                <td class="text-right">
                                    <input
                                        class="checkbox checkbox-info"
                                        type="checkbox"
                                        name={format!("{:?}", ConfigItems::Postgres)}
                                        bind:checked=config_items_rw[ConfigItems::Postgres as usize]
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-left">
                                    <label class="mr-2">{format!("{:?}", ConfigItems::Branches)}</label>
                                </td>
                                <td class="text-right">
                                    <input
                                        class="checkbox checkbox-info"
                                        type="checkbox"
                                        name={format!("{:?}", ConfigItems::Branches)}
                                        bind:checked=config_items_rw[ConfigItems::Branches as usize]
                                    />
                                </td>
                            </tr>
                        </tbody>
                    </table>

                    <Show when=move || config_items_rw.iter().any(|signal| signal.get()) >
                        <button class="btn btn-primary mt-4"
                            on:click=move |_| {
                                current_step_rw.set(ViewSteps::Loading);
                                spawn_local(async move {
                                    let project_config_option = generate_preview(source_project_rw.get(), dest_project_rw.get(), config_items_rw).await;
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
            </div>
        </div>
    }
}
