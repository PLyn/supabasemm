use super::AnimatedArrow;
use crate::shared::models::Project;
use leptos::prelude::*;

#[component]
pub fn ProjectSelectView(
    source_project_rw: RwSignal<String>,
    dest_project_rw: RwSignal<String>,
    projects_list: ReadSignal<Vec<Project>>,
    next_step_fn: impl Fn() + 'static + Clone + Send + Sync,
) -> impl IntoView {
    let validate_source = Signal::derive(move || {
        if source_project_rw.get().is_empty() {
            return "Please select a source project.".to_string();
        }

        if let Some(project) = projects_list
            .get()
            .iter()
            .find(|p| p.id == source_project_rw.get())
        {
            if project.status == "INACTIVE" {
                return "Selected source project is INACTIVE.".to_string();
            }
        }
        String::new()
    });

    let validate_destination = Signal::derive(move || {
        if dest_project_rw.get().is_empty() {
            return "Please select a destination project.".to_string();
        }

        if let Some(project) = projects_list
            .get()
            .iter()
            .find(|p| p.id == dest_project_rw.get())
        {
            if project.status == "INACTIVE" {
                return "Selected destination project is INACTIVE.".to_string();
            }
        }
        String::new()
    });

    let validate_form = Signal::derive(move || {
        if !source_project_rw.get().is_empty()
            && !dest_project_rw.get().is_empty()
            && source_project_rw.get() == dest_project_rw.get()
        {
            return "Source and destination projects cannot be the same.".to_string();
        }
        String::new()
    });

    let is_validated = Signal::derive(move || {
        if validate_source.get().is_empty()
            && validate_destination.get().is_empty()
            && validate_form.get().is_empty()
        {
            return "No Validation Errors Found. You may proceed!".to_string();
        }
        String::new()
    });

    view!(
        <div class="min-h-screen flex items-center justify-center p-4">
            <div class="max-w-screen-lg mx-auto w-full flex flex-col items-center">
                <div class="px-2 py-4 flex items-center w-full justify-between">
                    <h1 class="mb-4">"Select source project to copy project-specific configuration to the destination project"</h1>
                </div>
                <div class="px-2 py-4 flex items-center w-full justify-between">
                    <label class="mr-4 font-bold min-w-[150px]">"Source Project"</label> 
                    <select class="select select-info w-full" bind:Selected=source_project_rw > 
                        <Suspense fallback=move || view! { <option value="">Loading projects...</option> }>
                            {move || {
                                if !projects_list.get().is_empty() && source_project_rw.get().is_empty() {
                                    source_project_rw.set(projects_list.get()[0].id.clone());
                                }

                                projects_list
                                    .get()
                                    .into_iter()
                                    .map(|project| {
                                        let display_text = format!("{} - {} - {} - {}", project.id, project.name, project.region, project.status);
                                        view! { <option value={project.id.clone()}>{display_text}</option> }})
                                    .collect_view()
                            }}
                        </Suspense>
                    </select>
                </div>

                <AnimatedArrow />

                <div class="px-2 py-4 flex items-center w-full justify-between"> 
                    <label class="mr-4 font-bold min-w-[150px]">"Destination Project"</label>
                    <select class="select select-info w-full" bind:Selected=dest_project_rw > 
                        <Suspense fallback=move || view! { <option value="">Loading projects...</option> }>
                            {move || {
                                if !projects_list.get().is_empty() && dest_project_rw.get().is_empty() {
                                    dest_project_rw.set(projects_list.get()[0].id.clone());
                                }

                                projects_list
                                    .get()
                                    .into_iter()
                                    .map(|project| {
                                        let display_text = format!("{} - {} - {} - {}", project.id, project.name, project.region, project.status);
                                        view! { <option value={project.id.clone()}>{display_text}</option> }})
                                    .collect_view()
                            }}
                        </Suspense>
                    </select>
                </div>

                <Show when=move || is_validated.get().is_empty() >
                    <ul>
                    <li><p style="color: red;">{validate_form.get()}</p></li>
                    <li><p style="color: red;">{validate_source.get()}</p></li>
                    <li><p style="color: red;">{validate_destination.get()}</p></li>
                    </ul>
                </Show>

                <Show when=move || !is_validated.get().is_empty() >
                    <p style="color: green;">{is_validated.get()}</p>
                    {
                        let step_fn = next_step_fn.clone();
                        view! { <button class="btn btn-primary mt-4" on:click=move |_| { step_fn(); }>"Next"</button>}
                    }
                </Show>
            </div>
        </div>
    )
}
