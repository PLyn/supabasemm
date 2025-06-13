use crate::shared::models::ProjectConfig;
use leptos::prelude::*;

#[component]
pub fn ResultsView(
    results_rw: RwSignal<Vec<ProjectConfig>>,
    source_heading: String,
    dest_heading: String,
) -> impl IntoView {
    view! {
        <div class="w-full max-w-8xl space-y-6">
            <For
                each=move || results_rw.get().into_iter()
                key=|project_config| project_config.name.clone()
                children=move |project_config| {
                    view! {
                        <div class="overflow-x-auto">
                            <h2 class="text-lg font-bold mb-2">{project_config.name.clone()}</h2>
                            <table class="table w-full border-collapse border border-black">
                                <thead>
                                    <tr>
                                        <th class="p-2 text-center bg-gray-300 border border-black">"Config Item"</th>
                                        <th class="p-2 text-center bg-gray-300 border border-black">{source_heading.clone()}</th>
                                        <th class="p-2 text-center bg-gray-300 border border-black">{dest_heading.clone()}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <For
                                        each=move || project_config.diffs.clone().into_iter()
                                        key=|diff| diff.key.clone()
                                        children=move |diff| {
                                            view! {
                                                <tr class="hover:bg-gray-200">
                                                    <td class="p-2 text-left border border-black">{diff.key}</td>
                                                    <td class="p-2 border border-black">{diff.source_value}</td>
                                                    <td class="p-2 border border-black">{diff.dest_value}</td>
                                                </tr>
                                            }
                                        }
                                    />
                                </tbody>
                            </table>
                        </div>
                    }
                }
            />
        </div>
    }
}
