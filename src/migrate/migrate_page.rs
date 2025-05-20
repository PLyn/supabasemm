use super::components::{AuthorizeForm, ConfigSelectForm, ProjectSelectForm};
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum MigrationStep {
    AuthorizeStep,
    ProjectsStep,
    ConfigStep,
}

#[component]
pub fn MigratePage() -> impl IntoView {
    let auth_status_rw = RwSignal::new("".to_string());

    let auth_config_rw = RwSignal::new(false);
    let postgrest_config_rw = RwSignal::new(false);
    let edge_fn_config_rw = RwSignal::new(false);
    let secrets_config_rw = RwSignal::new(false);
    let storage_config_rw = RwSignal::new(false);
    let branches_config_rw = RwSignal::new(false);

    let source_project_rw = RwSignal::new("".to_string());
    let dest_project_rw = RwSignal::new("".to_string());

    let current_step_rw = RwSignal::new(MigrationStep::AuthorizeStep);
    view! {
        <h2>Supabase Migrate project configuration</h2>
        <h2>Authorize</h2>

        {move || match current_step_rw.get() {
            MigrationStep::AuthorizeStep => view! {
                <>
                    <AuthorizeForm />

                    <button on:click=move |_| { current_step_rw.set(MigrationStep::ProjectsStep); }>Next</button>
                </>
            }.into_any(),
            MigrationStep::ProjectsStep => view! {
                <>
                    <ProjectSelectForm
                        source_project_rw
                        dest_project_rw />

                    <button on:click=move |_| { current_step_rw.set(MigrationStep::AuthorizeStep); }>Back</button>
                    <button on:click=move |_| { current_step_rw.set(MigrationStep::ConfigStep); }>Next</button>
                </>
            }.into_any(),
            MigrationStep::ConfigStep => view! {
                <>
                    <ConfigSelectForm
                       auth_config_rw
                       postgrest_config_rw
                       edge_fn_config_rw
                       secrets_config_rw
                       storage_config_rw
                       branches_config_rw />

                    <button on:click=move |_| { current_step_rw.set(MigrationStep::ProjectsStep); }>Back</button>
                    <button>Preview</button>
                </>
            }.into_any(),
        }}
    }
}
