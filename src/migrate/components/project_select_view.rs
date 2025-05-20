use crate::shared::models::Project;
use leptos::prelude::*;

#[component]
pub fn ProjectSelectForm(
    source_project_rw: RwSignal<String>,
    dest_project_rw: RwSignal<String>,
    projects_rw: RwSignal<Vec<Project>>,
) -> impl IntoView {
    view!(
        <br />
        <label>Select Source Project</label>
        <select
            on:change:target=move |ev| {
                source_project_rw.set(ev.target().value().parse().unwrap());
            }
            prop:value=move || source_project_rw.get().to_string()>
                <Suspense fallback=move || view! { <option value="">Loading projects...</option> }>
                    {move || {
                        projects_rw
                            .get()
                            .into_iter()
                            .map(|project| {
                                let display_text = format!(
                                "{} - {} - {} - {}",
                                project.id, project.name, project.region, project.status);
                                view! { <option value={project.id.clone()}>{display_text}</option> }
                            })
                            .collect_view()
                        }
                    }
                </Suspense>
        </select>
        <br />

        <div style="color: dodgerblue; margin: 20px 0; display: inline-block;"> // This div is fine within the fragment
            <svg width="200" height="200" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
                <polygon points="50,90 20,40 80,40 50,90" fill="currentColor" stroke="currentColor" stroke-width="2"/>
                <rect x="45" y="10" width="10" height="30" fill="currentColor" stroke="currentColor" stroke-width="2"/>
            </svg>
        </div>

        <br />
        <label>Select Destination Project</label>
        <select
        on:change:target=move |ev| {
            dest_project_rw.set(ev.target().value().parse().unwrap());
        }
        prop:value=move || dest_project_rw.get().to_string()
        >
        <Suspense fallback=move || view! { <option value="">Loading projects...</option> }>
            {move || {
                projects_rw
                    .get()
                    .into_iter()
                    .map(|project| {
                        let display_text = format!(
                        "{} - {} - {} - {}",
                        project.id, project.name, project.region, project.status);
                        view! { <option value={project.id.clone()}>{display_text}</option> }
                    })
                    .collect_view()
                }
            }
        </Suspense>
        </select>
        <br />
    )
}
