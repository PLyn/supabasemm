use super::components::{ConfigSelectView, ProjectSelectView, ResultsView};
use super::functions::migrate_config;
use crate::shared::models::{Project, ProjectConfig};
use crate::shared::server_functions::{check_auth_status, get_projects};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[derive(Clone, Copy, PartialEq)]
pub enum ViewSteps {
    Projects,
    Config,
    Loading,
    Preview,
    Results,
}
#[derive(Debug)]
pub enum ConfigItems {
    Auth = 0,
    Postgrest = 1,
    EdgeFunctions = 2,
    Secrets = 3,
    Storage = 4,
    Postgres = 5,
    Branches = 6,
}
pub const CONFIG_ITEM_COUNT: usize = 7;

#[component]
pub fn MigratePage() -> impl IntoView {
    let source_project_rw = RwSignal::new("".to_string());
    let dest_project_rw = RwSignal::new("".to_string());
    let current_step_rw = RwSignal::new(ViewSteps::Projects);
    let results_rw: RwSignal<Vec<ProjectConfig>> = RwSignal::new(Vec::new());
    let migration_status_rw: RwSignal<String> =
        RwSignal::new("Migration Status: Migration has not been run".to_string());
    let (projects_list, set_projects_list) = signal(Vec::<Project>::new());

    let config_items_rw: [RwSignal<bool>; CONFIG_ITEM_COUNT] = [
        RwSignal::new(false),
        RwSignal::new(false),
        RwSignal::new(false),
        RwSignal::new(false),
        RwSignal::new(false),
        RwSignal::new(false),
        RwSignal::new(false),
    ];

    Effect::new(move |_| {
        if current_step_rw.get() == ViewSteps::Projects {
            spawn_local(async move {
                if projects_list.get_untracked().is_empty() {
                    let projects_result = get_projects().await;
                    if let Ok(projects_list) = projects_result {
                        set_projects_list.set(projects_list.clone());
                        if !projects_list.is_empty() {
                            source_project_rw.set(projects_list[0].id.clone());
                            dest_project_rw.set(projects_list[0].id.clone());
                        }
                    }
                };
            });
            if !projects_list.get_untracked().is_empty() {
                source_project_rw.set(projects_list.get_untracked()[0].id.clone());
                dest_project_rw.set(projects_list.get_untracked()[0].id.clone());
            }
        }
    });

    view! {
        {move || match current_step_rw.get() {
            ViewSteps::Projects => view! {
                <ProjectSelectView
                    source_project_rw
                    dest_project_rw
                    projects_list
                    next_step_fn=move || { current_step_rw.set(ViewSteps::Config); }
                />}.into_any(),
            ViewSteps::Config => view! {
                <ConfigSelectView
                    config_items_rw
                    source_project_rw
                    dest_project_rw
                    results_rw
                    current_step_rw />
            }.into_any(),
            ViewSteps::Loading => view! {
                <div class="flex flex-col items-center justify-center h-screen">
                    <h3 class="text-lg font-semibold mb-4 text-gray-700">Loading...</h3>
                    <svg width="80" height="80" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
                        <rect x="15" y="30" width="10" height="20" fill="#3B82F6" rx="5">
                            <animate attributeName="height" values="20;40;20" dur="1s" repeatCount="indefinite" />
                            <animate attributeName="y" values="30;20;30" dur="1s" repeatCount="indefinite" />
                        </rect>
                        <rect x="35" y="30" width="10" height="20" fill="#8B5CF6" rx="5">
                            <animate attributeName="height" values="20;40;20" dur="1s" begin="0.2s" repeatCount="indefinite" />
                            <animate attributeName="y" values="30;20;30" dur="1s" begin="0.2s" repeatCount="indefinite" />
                        </rect>
                        <rect x="55" y="30" width="10" height="20" fill="#EC4899" rx="5">
                            <animate attributeName="height" values="20;40;20" dur="1s" begin="0.4s" repeatCount="indefinite" />
                            <animate attributeName="y" values="30;20;30" dur="1s" begin="0.4s" repeatCount="indefinite" />
                        </rect>
                    </svg>
                </div>
            }.into_any(),
            ViewSteps::Preview => view! {
                <div class="flex flex-col p-4 mt-4">
                    <button class="btn btn-secondary mb-4 max-w-sm mx-auto" on:click=move |_| { current_step_rw.set(ViewSteps::Config); }>"Back"</button>
                    <h3 class="text-2xl font-bold mb-4">"Preview Results"</h3>
                    <ResultsView
                        results_rw
                        source_heading="Source".to_string()
                        dest_heading="Destination".to_string() />

                    <button class="btn btn-primary mt-4 max-w-sm mx-auto" on:click=move |_| {
                        current_step_rw.set(ViewSteps::Loading);
                        spawn_local(async move {
                            let migrate_result = migrate_config(results_rw.get(), dest_project_rw.get()).await;
                            match migrate_result {
                                Ok(response) => {
                                    results_rw.set(response);
                                    migration_status_rw.set("Success".to_string())
                                }
                                Err(e) => {
                                    results_rw.set(Vec::new());
                                    migration_status_rw.set(e.to_string());
                                }
                            }
                            current_step_rw.set(ViewSteps::Results);
                        });
                    }>"Migrate Configuration!"</button>
                </div>
            }.into_any(),
            ViewSteps::Results => view! {
                <div class="flex flex-col items-center justify-center min-h-screen p-4">
                    <h3 class="text-1xl font-bold my-4">{move || migration_status_rw.get()}</h3>
                    <h3 class="text-2xl font-bold mb-4">"Migration Results"</h3>

                    <ResultsView
                        results_rw
                        source_heading="Config change to migrate".to_string()
                        dest_heading="Current config after migration".to_string() />
                </div>
            }.into_any()
        }
        }
    }
}
