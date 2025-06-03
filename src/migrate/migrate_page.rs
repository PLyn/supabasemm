use crate::shared::models::{Project, ProjectConfig};
use super::components::{ConfigSelectForm, ProjectSelectForm, PreviewResultsView};
use super::functions::{generate_preview, migrate_config};
use crate::shared::server_functions::{check_auth_status, get_projects};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[derive(Clone, Copy, PartialEq)]
pub enum ViewSteps {
    Projects,
    Config,
    Loading,
    Preview,
}
#[derive(Debug)]
pub enum ConfigItems{
    Auth = 0,
    Postgrest = 1,
    EdgeFunctions = 2,
    Secrets = 3,
    Storage = 4,
    Branches = 5
}

#[component]
pub fn MigratePage() -> impl IntoView {
    let (is_authenticated, set_is_authenticated) = signal(false);

    let source_project_rw = RwSignal::new("".to_string());
    let dest_project_rw = RwSignal::new("".to_string());
    let current_step_rw = RwSignal::new(ViewSteps::Projects);
    let preview_results_rw: RwSignal<Vec<ProjectConfig>> = RwSignal::new(Vec::new());
    let migrate_results_rw: RwSignal<String> = RwSignal::new("".to_string());
    let migration_status_rw: RwSignal<String> = RwSignal::new("Migration Status: Migration has not been run".to_string());
    let (projects_list, set_projects_list) = signal(Vec::<Project>::new());

    let config_items_rw: [RwSignal<(String, bool)>; 6] = [
        RwSignal::new((format!("{:?}", ConfigItems::Auth), false)),
        RwSignal::new((format!("{:?}", ConfigItems::Postgrest), false)),
        RwSignal::new((format!("{:?}", ConfigItems::EdgeFunctions), false)),
        RwSignal::new((format!("{:?}", ConfigItems::Secrets), false)),
        RwSignal::new((format!("{:?}", ConfigItems::Storage), false)),
        RwSignal::new((format!("{:?}", ConfigItems::Branches), false))];

    Effect::new(move |_| {
        spawn_local(async move {
            let auth_check_result = check_auth_status().await;
            if let Ok(auth_status) = auth_check_result {
                set_is_authenticated.set(auth_status);
            };

            if is_authenticated.get_untracked() {
                current_step_rw.set(ViewSteps::Loading);
                let projects_result = get_projects().await;
                if let Ok(projects_list) = projects_result {
                    set_projects_list.set(projects_list);
                    current_step_rw.set(ViewSteps::Projects);
                };
            }
        });
    });

    view! {
        <Show when=move || !is_authenticated.get() >
            <div class="flex flex-col items-center">
                <h2 class="text-3xl font-bold mt-4">"Supabase Migrate project configuration"</h2>
                <h2 class="py-4">"Sign in using Supabase to allow access to your projects using the Management API"</h2>
                <button class="btn btn-primary" on:click=move |_| { window().location().set_href("/connect-supabase/login").unwrap(); }>
                    "Sign in with Supabase"
                </button>
            </div>
        </Show>
        <Show when=move || is_authenticated.get() >
            {move || match current_step_rw.get() {
                ViewSteps::Projects => view! {      
                    <ProjectSelectForm
                        source_project_rw
                        dest_project_rw
                        projects_list
                        next_step_fn=move || { current_step_rw.set(ViewSteps::Config); } 
                    />}.into_any(),
                ViewSteps::Config => view! {
                    <div class="flex flex-col items-center">
                        <button class="btn btn-secondary my-4" on:click=move |_| { current_step_rw.set(ViewSteps::Projects); }>"Back"</button>

                        <ConfigSelectForm config_items_rw />

                        <Show when=move || config_items_rw.iter().any(|signal| signal.get().1) >
                            <button class="btn btn-primary mt-4"
                                on:click=move |_| { config_next_step_fn(source_project_rw.get(), dest_project_rw.get(), preview_results_rw, current_step_rw); }>        
                                "Preview Changes"
                            </button>
                        </Show>
                    </div>}.into_any(),
                ViewSteps::Loading => view! {
                    <div class="flex flex-col items-center justify-center h-screen">
                        <h3>Loading...</h3>
                        <svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
                            <circle cx="50" cy="50" r="40" stroke="#007bff" stroke-width="4" fill="none">
                                <animateTransform attributeName="transform" attributeType="XML" type="rotate" from="0 50 50" to="360 50 50" dur="1s" repeatCount="indefinite" />
                            </circle>
                        </svg>
                    </div>}.into_any(),
                ViewSteps::Preview => view! {
                    <div class="flex flex-col items-center justify-center min-h-screen p-4">
                        <h3 class="text-1xl font-bold my-4">{move || migration_status_rw.get()}</h3>
                        <button class="btn btn-secondary mb-4" on:click=move |_| { current_step_rw.set(ViewSteps::Config); }>"Back"</button>

                        <button class="btn btn-primary mt-4" on:click=move |_| {
                            current_step_rw.set(ViewSteps::Loading);
                            spawn_local(async move {
                                let migrate_result = migrate_config(preview_results_rw.get(), dest_project_rw.get()).await;
                                match migrate_result {
                                    Ok(response_text) => {
                                        migrate_results_rw.set(response_text);
                                        //run server function compare results with preview to find ones not set
                                        // verify_results()
                                        migration_status_rw.set("Success".to_string())
                                    }
                                    Err(e) => { migration_status_rw.set(e.to_string()); }
                                }
                                current_step_rw.set(ViewSteps::Preview); 
                            });
                        }>"Migrate Project Configuration!"</button>

                        <PreviewResultsView preview_results_rw />
                    </div>}.into_any() 
                }
            }
        </Show>
    }
}

fn config_next_step_fn(source_project: String, dest_project: String, preview_results_rw: RwSignal<Vec<ProjectConfig>>, current_step_rw: RwSignal<ViewSteps>) {
    current_step_rw.set(ViewSteps::Loading);
    spawn_local(async move {
        let project_config_option = generate_preview(source_project, dest_project).await;
        match project_config_option {
            Ok(project_config) => preview_results_rw.set(project_config),
            Err(_) => preview_results_rw.set(Vec::new())
        } 
        current_step_rw.set(ViewSteps::Preview);    
    });
}