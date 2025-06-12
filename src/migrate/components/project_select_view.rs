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
        <div class="max-w-screen-lg w-full flex flex-col items-center mx-auto mt-4">
            <h1 class="mb-4 text-xl font-bold">"Migrate Project configuration"</h1>
            <label class="mb-4 text-lg">"Source Project"</label> 
            <select 
                class="select select-info w-full mb-4"
                on:change:target=move |ev| {
                    source_project_rw.set(ev.target().value().parse().unwrap());
                }  
                prop:value=move || source_project_rw.get()
                prop:Selected=move || source_project_rw.get()
            > 
                {move || {
                    projects_list
                        .get()
                        .into_iter()
                        .map(|project| {
                            let display_text = format!("{} - {} - {} - {}", project.id, project.name, project.region, project.status);
                            view! { <option value={project.id.clone()}>{display_text}</option> }})
                        .collect_view()
                }}
            </select>

            <AnimatedArrow />

            <label class="my-4 text-lg">"Destination Project"</label>
            <select 
                class="select select-info w-full mb-4" 
                on:change:target=move |ev| {
                    dest_project_rw.set(ev.target().value().parse().unwrap());
                }  
                prop:value=move || dest_project_rw.get()
                prop:Selected=move || dest_project_rw.get()
            > 
                {move || {
                    projects_list
                        .get()
                        .into_iter()
                        .map(|project| {
                            let display_text = format!("{} - {} - {} - {}", project.id, project.name, project.region, project.status);
                            view! { <option value={project.id.clone()}>{display_text}</option> }})
                        .collect_view()
                }}
            </select>

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
    )
}
