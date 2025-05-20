use super::components::{AuthorizeForm, ConfigSelectForm, ProjectSelectForm};
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[derive(Clone, Copy, PartialEq)]
pub enum MigrationStep {
    AuthorizeStep,
    ProjectsStep,
    ConfigStep,
    LoadingStep,
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

        {move || match current_step_rw.get() {
            MigrationStep::AuthorizeStep => view! {
                <>
                    <h2>Authorize</h2>
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
                    <button on:click=move |_| {
                        current_step_rw.set(MigrationStep::LoadingStep);

                        spawn_local(async move {
                            TimeoutFuture::new(3000).await;
                            current_step_rw.set(MigrationStep::ProjectsStep); // Or a new MigrationStep::PreviewStep
                        });
                    }>Preview</button>
                </>
            }.into_any(),
            MigrationStep::LoadingStep => view! {
                <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;">
                    <h3>Loading...</h3>
                    <svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
                        <circle cx="50" cy="50" r="40" stroke="#007bff" stroke-width="4" fill="none">
                            <animateTransform
                                attributeName="transform"
                                attributeType="XML"
                                type="rotate"
                                from="0 50 50"
                                to="360 50 50"
                                dur="1s"
                                repeatCount="indefinite" />
                        </circle>
                    </svg>
                </div>
            }.into_any(),
        }}
    }
}
