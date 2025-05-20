use crate::monitor::functions::get_projects;
use crate::shared::models::Project;

use super::components::{AuthorizeForm, ConfigSelectForm, ProjectSelectForm};
use super::functions::{check_auth_status, generate_preview};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[derive(Clone, Copy, PartialEq)]
pub enum ViewSteps {
    Projects,
    Config,
    Loading,
    Preview,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ConfigItem {
    Auth = 0,
    Postgrest = 1,
    EdgeFunction = 2,
    Secrets = 3,
    Storage = 4,
    Branches = 5,
}

#[component]
pub fn MigratePage() -> impl IntoView {
    let is_authenticated_rw = RwSignal::new(false);
    let source_project_rw = RwSignal::new("".to_string());
    let dest_project_rw = RwSignal::new("".to_string());
    let current_step_rw = RwSignal::new(ViewSteps::Projects);

    let config_items_rw: [RwSignal<bool>; 6] = std::array::from_fn(|_| RwSignal::new(false));
    let projects_rw: RwSignal<Vec<Project>> = RwSignal::new(Vec::new());

    Effect::new(move |_| {
        //Only added here to make this effect re-run every time current_step_rw changes
        let current_step = current_step_rw.get();

        spawn_local(async move {
            let auth_status = check_auth_status().await;
            let is_authenticated = match auth_status {
                Ok(status) => status, // If Ok, use the boolean status
                Err(e) => {
                    eprintln!("Error checking auth status: {:?}", e); // Log the error
                    false
                }
            };
            is_authenticated_rw.set(is_authenticated);
        });
    });
    Effect::new(move |_| {
        if is_authenticated_rw.get() {
            spawn_local(async move {
                let projects_result = get_projects().await;
                match projects_result {
                    Ok(projects_list) => {
                        projects_rw.set(projects_list);
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch projects: {:?}", e);
                    }
                }
            });
        }
    });

    view! {
        <h2>Supabase Migrate project configuration</h2>
        <h3>Authenticated? {is_authenticated_rw}</h3>

        <Show when=move || !is_authenticated_rw.get() >
                <>
                    <h2>Authorize</h2>
                    <AuthorizeForm />
                </>
        </Show>

        <Show when=move || is_authenticated_rw.get() >
            {move || match current_step_rw.get() {
                ViewSteps::Projects => view! {
                    <>
                        <ProjectSelectForm
                            source_project_rw
                            dest_project_rw
                            projects_rw
                        />
                        <button on:click=move |_| { current_step_rw.set(ViewSteps::Config); }>Next</button>
                    </>
                }.into_any(),
                ViewSteps::Config => view! {
                    <>
                        <ConfigSelectForm
                            config_items_rw />

                        <button on:click=move |_| { current_step_rw.set(ViewSteps::Projects); }>Back</button>
                        <button on:click=move |_| {
                            current_step_rw.set(ViewSteps::Loading);
                            spawn_local(async move {
                                generate_preview().await;
                                current_step_rw.set(ViewSteps::Preview);
                            });
                        }>Preview</button>
                    </>
                }.into_any(),
                ViewSteps::Loading => view! {
                    <>
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
                                        repeatCount="indefinite"
                                    />
                                </circle>
                            </svg>
                        </div>
                    </>
                }.into_any(),
                ViewSteps::Preview => view! {
                    <>
                        "Preview screen"
                    </>
                }.into_any()
            }}
        </Show>
    }
}
