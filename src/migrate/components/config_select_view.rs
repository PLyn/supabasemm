use leptos::prelude::*;
use crate::{migrate::{functions::generate_preview, migrate_page::ViewSteps}, shared::models::ProjectConfig};
use leptos::task::spawn_local;

#[component]
pub fn ConfigSelectView(
    config_items_rw: [RwSignal<(String, bool)>; 7],
    source_project_rw: RwSignal<String>,
    dest_project_rw: RwSignal<String>,
    results_rw: RwSignal<Vec<ProjectConfig>>,
    current_step_rw: RwSignal<ViewSteps>,
) -> impl IntoView {
    view! {
            <div class="flex flex-col items-center">
            <button class="btn btn-secondary my-4" on:click=move |_| { current_step_rw.set(ViewSteps::Projects); }>"Back"</button>

            <table class="table table-fixed w-full">
                <colgroup>
                    <col style="width: 40%;" />
                    <col style="width: 60%;" />
                </colgroup>
                <tbody>
                    {config_items_rw.into_iter().map(|item_signal| {
                        let (name, enabled) = item_signal.get();

                        view! {
                            <tr>
                                <td class="text-left">
                                    <label class="mr-2">{name.clone()}</label>
                                </td>
                                <td class="text-right">
                                    <input
                                        class="checkbox checkbox-info"
                                        type="checkbox"
                                        name={name.clone()}
                                        prop:checked=move || enabled
                                        on:change=move |ev| {
                                            let new_value = event_target_checked(&ev);
                                            item_signal.set((item_signal.get().0, new_value));
                                        }
                                    />
                                </td>
                            </tr>
                        }
                    })
                    .collect_view()
                    }
                </tbody>
            </table>

            <Show when=move || config_items_rw.iter().any(|signal| signal.get().1) >
                <button class="btn btn-primary mt-4"
                    on:click=move |_| { current_step_rw.set(ViewSteps::Loading);
                                        spawn_local(async move {
                                            let project_config_option = generate_preview(source_project_rw.get(), dest_project_rw.get()).await;
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
