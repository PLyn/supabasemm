use crate::shared::models::ProjectConfig;
use leptos::prelude::*;

#[component]
pub fn ResultsView(
    results_rw: RwSignal<Vec<ProjectConfig>>,
    source_heading: String,
    dest_heading: String
) -> impl IntoView {
    view! {
        <div class="w-full max-w-8xl overflow-x-auto">
            <table class="table w-full border-collapse border border-black">
                <thead>
                    <tr>
                        <th class="p-2 text-center bg-gray-300 border border-black">"Service"</th>
                        <th class="p-2 text-center bg-gray-300 border border-black">"Config Item"</th>
                        <th class="p-2 text-center bg-gray-300 border border-black">{source_heading}</th>
                        <th class="p-2 text-center bg-gray-300 border border-black">{dest_heading}</th>
                    </tr>
                </thead>
                <tbody>
                    <For 
                        each=move || results_rw.get().into_iter()
                        key=|project_config| project_config.name.clone()
                        children=move |project_config| {
                            project_config.diffs.iter().map(|diff| {
                                view! {
                                    <tr class="hover:bg-gray-200">
                                        <td class="p-2 text-left border border-black">{project_config.name.clone()}</td>
                                        <td class="p-2 text-left border border-black">{diff.key.clone()}</td>
                                        <td class="p-2 border border-black">{diff.source_value.clone()}</td>
                                        <td class="p-2 border border-black">{diff.dest_value.clone()}</td>
                                    </tr>
                                }    
                            }).collect::<Vec<_>>().into_view()
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}
