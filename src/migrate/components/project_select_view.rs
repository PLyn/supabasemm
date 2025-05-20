use crate::shared::models::Project;
use leptos::prelude::*;

#[component]
pub fn ProjectSelectForm(
    source_project_rw: RwSignal<String>,
    dest_project_rw: RwSignal<String>,
    projects_rw: RwSignal<Vec<Project>>,
    projects_validated_rw: RwSignal<bool>,
) -> impl IntoView {
    let source_error_message = Memo::new(move |_| {
        let source_id = source_project_rw.get();
        let projects = projects_rw.get();

        if source_id.is_empty() {
            return "Please select a source project.".to_string();
        }

        if let Some(project) = projects.iter().find(|p| p.id == source_id) {
            if project.status == "INACTIVE" {
                return "Selected source project is INACTIVE.".to_string();
            }
        }
        String::new() // No error
    });

    let dest_error_message = Memo::new(move |_| {
        let dest_id = dest_project_rw.get();
        let projects = projects_rw.get();

        if dest_id.is_empty() {
            return "Please select a destination project.".to_string();
        }

        if let Some(project) = projects.iter().find(|p| p.id == dest_id) {
            if project.status == "INACTIVE" {
                return "Selected destination project is INACTIVE.".to_string();
            }
        }
        String::new() // No error
    });

    let general_error_message = Memo::new(move |_| {
        let source_id = source_project_rw.get();
        let dest_id = dest_project_rw.get();

        if !source_id.is_empty() && !dest_id.is_empty() && source_id == dest_id {
            return "Source and destination projects cannot be the same.".to_string();
        }
        String::new() // No error
    });

    let validation_check = Memo::new(move |_| {
        if source_error_message.get().is_empty() && dest_error_message.get().is_empty() && general_error_message.get().is_empty() {
            projects_validated_rw.set(true);
           return "No Validation Errors Found. You may proceed!".to_string();
        }
        projects_validated_rw.set(false);
        String::new() 
    });


    view!(
        <br />        

        {move || (!general_error_message.get().is_empty()).then(|| view! { <p style="color: red;">{general_error_message.get()}</p> })}
        
        <br />

        {move || (!source_error_message.get().is_empty()).then(|| view! { <p style="color: red;">{source_error_message.get()}</p> })}

        <br />
        <label>Select Source Project</label>
        <select
            on:change:target=move |ev| {
                source_project_rw.set(ev.target().value().parse().unwrap());
            }
            prop:value=move || source_project_rw.get().to_string()>
                <Suspense fallback=move || view! { <option value="">Loading projects...</option> }>
                    <option value="" selected={move || source_project_rw.get().is_empty()}>
                        "No option selected"
                    </option>
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

        {move || (!dest_error_message.get().is_empty()).then(|| view! { <p style="color: red;">{dest_error_message.get()}</p> })}

        <br />
        <label>Select Destination Project</label>
        <select
        on:change:target=move |ev| {
            dest_project_rw.set(ev.target().value().parse().unwrap());
        }
        prop:value=move || dest_project_rw.get().to_string()
        >
        <Suspense fallback=move || view! { <option value="">Loading projects...</option> }>
            <option value="" selected={move || source_project_rw.get().is_empty()}>
                "No option selected"
            </option>
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

        {move || (!validation_check.get().is_empty()).then(|| view! { <p style="color: green;">{validation_check.get()}</p> })}
        <br />
    )
}
