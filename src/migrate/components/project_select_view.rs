use super::AnimatedArrow;
use crate::shared::models::Project;
use leptos::prelude::*;

#[component]
pub fn ProjectSelectForm(
    source_project_rw: RwSignal<String>,
    dest_project_rw: RwSignal<String>,
    projects_list: ReadSignal<Vec<Project>>,
    set_is_project_select_validated: WriteSignal<bool>,
) -> impl IntoView {
    let (validate_source, validate_destination, validate_form, is_validated) = validate_errors(
        source_project_rw,
        dest_project_rw,
        projects_list,
        set_is_project_select_validated,
    );
    view!(
        <div class="px-2 py-4">
            <label class="mr-4">"Select Source Project"</label>
            <select class="select select-info"
                on:change:target=move |ev| {
                    source_project_rw.set(ev.target().value().parse().unwrap());
                }
                prop:value=move || source_project_rw.get().to_string()>
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
                            }
                        }
                    </Suspense>
            </select>
        </div>

        <AnimatedArrow />

        <div class="px-2 py-4">
            <label class="mr-4">Select Destination Project</label>
            <select class="select select-info"
            on:change:target=move |ev| {
                dest_project_rw.set(ev.target().value().parse().unwrap());
            }
            prop:value=move || dest_project_rw.get().to_string()
            >
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
                    }
                }
            </Suspense>
            </select>
        </div>

        <Show when=move || !is_validated.get().is_empty() >
            <p style="color: green;">{is_validated.get()}</p>
        </Show>

        <Show when=move || is_validated.get().is_empty() >
            <ul>
            <li><p style="color: red;">{validate_form.get()}</p></li>
            <li><p style="color: red;">{validate_source.get()}</p></li>
            <li><p style="color: red;">{validate_destination.get()}</p></li>
            </ul>
        </Show>
    )
}

fn validate_errors(
    source_project_rw: RwSignal<String>,
    dest_project_rw: RwSignal<String>,
    projects_list: ReadSignal<Vec<Project>>,
    set_is_project_select_validated: WriteSignal<bool>,
) -> (
    Signal<String>,
    Signal<String>,
    Signal<String>,
    Signal<String>,
) {
    let validate_source = Signal::derive(move || {
        let source_project = source_project_rw.get();
        let projects = projects_list.get();
        if source_project.is_empty() {
            return "Please select a source project.".to_string();
        }

        if let Some(project) = projects.iter().find(|p| p.id == source_project) {
            if project.status == "INACTIVE" {
                return "Selected source project is INACTIVE.".to_string();
            }
        }

        String::new() // No error
    });

    let validate_destination = Signal::derive(move || {
        let projects = projects_list.get();
        let dest_project = dest_project_rw.get();
        if dest_project.is_empty() {
            return "Please select a destination project.".to_string();
        }

        if let Some(project) = projects.iter().find(|p| p.id == dest_project) {
            if project.status == "INACTIVE" {
                return "Selected destination project is INACTIVE.".to_string();
            }
        }

        String::new() // No error
    });

    let validate_form = Signal::derive(move || {
        let source_project = source_project_rw.get();
        let dest_project = dest_project_rw.get();
        if !source_project.is_empty() && !dest_project.is_empty() && source_project == dest_project
        {
            return "Source and destination projects cannot be the same.".to_string();
        }

        String::new() // No error
    });

    let is_validated = Signal::derive(move || {
        if validate_source.get().is_empty()
            && validate_destination.get().is_empty()
            && validate_form.get().is_empty()
        {
            set_is_project_select_validated.set(true);
            return "No Validation Errors Found. You may proceed!".to_string();
        }
        set_is_project_select_validated.set(false);
        String::new()
    });

    (
        validate_source,
        validate_destination,
        validate_form,
        is_validated,
    )
}
