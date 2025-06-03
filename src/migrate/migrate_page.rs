use crate::shared::models::{DiffEntry, Project, ProjectConfig};

use super::components::{ConfigSelectForm, ProjectSelectForm};
use super::functions::{generate_preview, migrate_config};
use crate::shared::server_functions::{check_auth_status, get_projects};
use leptos::prelude::*;
use leptos::task::spawn_local;

const CONFIG_COUNT: usize = 6;

#[derive(Clone, Copy, PartialEq)]
pub enum ViewSteps {
    Projects,
    Config,
    Loading,
    Preview,
}

#[derive(Clone, Copy)]
pub struct ConfigState {
    pub items: [(&'static str, &'static str); CONFIG_COUNT],
    pub values: [RwSignal<bool>; CONFIG_COUNT],
    pub count: usize,
}

impl ConfigState {
    pub fn new() -> Self {
        Self {
            items: [
                ("auth", "Migrate Auth Config"),
                ("postgrest", "Migrate Postgrest Config"),
                ("edge_function", "Migrate Edge Function Config"),
                ("secrets", "Migrate Secrets Config"),
                ("storage", "Migrate Storage Config"),
                ("branches", "Migrate Branches Config"),
            ],
            values: std::array::from_fn(|_| RwSignal::new(false)),
            count: CONFIG_COUNT
        }
    }
    pub fn any_config_selected(self) -> bool {
        self.values.iter().any(|signal| signal.get())
    }
}

#[component]
pub fn MigratePage() -> impl IntoView {
    let (is_authenticated, set_is_authenticated) = signal(false);

    let source_project_rw = RwSignal::new("".to_string());
    let dest_project_rw = RwSignal::new("".to_string());
    let current_step_rw = RwSignal::new(ViewSteps::Projects);

    let (projects_list, set_projects_list) = signal(Vec::<Project>::new());
    let (is_projects_select_validated, set_is_project_select_validated) = signal(false);

    let preview_results_rw: RwSignal<Vec<ProjectConfig>> = RwSignal::new(Vec::new());
    let config_state_rw: RwSignal<ConfigState> = RwSignal::new(ConfigState::new());

    let migration_status_rw = RwSignal::new("Migration Status: Migration has not been run");

    Effect::new(move |_| {
        spawn_local(async move {
            let auth_check_result = check_auth_status().await;
            if let Ok(auth_status) = auth_check_result {
                set_is_authenticated.set(auth_status);
            };

            if is_authenticated.get_untracked() {
                let projects_result = get_projects().await;
                if let Ok(projects_list) = projects_result {
                    set_projects_list.set(projects_list);
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
                    <div class="flex flex-col items-center">
                        <ProjectSelectForm
                            source_project_rw
                            dest_project_rw
                            projects_list
                            set_is_project_select_validated />

                        <Show when=move || is_projects_select_validated.get() >
                            <button class="btn btn-primary mt-4"
                                on:click=move |_| { current_step_rw.set(ViewSteps::Config); }>
                                Next
                            </button>
                        </Show>
                    </div>
                }.into_any(),

                ViewSteps::Config => view! {
                    <div class="flex flex-col items-center">
                        <button class="btn btn-secondary my-4" on:click=move |_| { current_step_rw.set(ViewSteps::Projects); }>"Back"</button>

                        <ConfigSelectForm
                            config_state_rw />

                        <Show when=move || config_state_rw.get().any_config_selected() >
                            <button class="btn btn-primary mt-4"
                                on:click=move |_| {
                                current_step_rw.set(ViewSteps::Loading);
                                spawn_local(async move {
                                    let project_config_option = generate_preview(source_project_rw.get_untracked(), dest_project_rw.get_untracked()).await;
                                    match project_config_option {
                                        Ok(project_config) => preview_results_rw.set(project_config),
                                        Err(_) => preview_results_rw.set(Vec::new())
                                    }
                                    current_step_rw.set(ViewSteps::Preview);
                                });
                            }>"Preview Changes"</button>
                        </Show>
                    </div>
                }.into_any(),

                ViewSteps::Loading => view! {
                    <div class="flex flex-col items-center justify-center h-screen">
                        <h3>Loading...</h3>
                        <svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
                            <circle cx="50" cy="50" r="40" stroke="#007bff" stroke-width="4" fill="none">
                                <animateTransform attributeName="transform" attributeType="XML" type="rotate" from="0 50 50" to="360 50 50" dur="1s" repeatCount="indefinite" />
                            </circle>
                        </svg>
                    </div>
                }.into_any(),

                ViewSteps::Preview => view! {
                    <div class="flex flex-col items-center justify-center min-h-screen p-4">
                        <h3 class="text-1xl font-bold my-4">{move || migration_status_rw.get()}</h3>
                        <button class="btn btn-secondary mb-4" on:click=move |_| { current_step_rw.set(ViewSteps::Config); }>"Back"</button>

                        <button class="btn btn-primary mt-4" on:click=move |_| {
                            spawn_local(async move {
                                let migrate_result = migrate_config(preview_results_rw.get(), dest_project_rw.get()).await;
                                match migrate_result {
                                    Ok(status) => migration_status_rw.set("Success!"),
                                    Err(e) => {
                                        eprintln!("{:?}", e);
                                        
                                        migration_status_rw.set("Failure");
                                    }
                                }
                            });
                        }>"Migrate Project Configuration!"</button>

                        <h3 class="text-2xl font-bold mb-4">Preview Results</h3>
                        <div class="w-full max-w-8xl overflow-x-auto">
                            <table class="table w-full border-collapse border border-black">
                                <thead>
                                    <tr>
                                        <th class="p-2 text-center bg-gray-300 border border-black">"Service"</th>
                                        <th class="p-2 text-center bg-gray-300 border border-black">"Config Item"</th>
                                        <th class="p-2 text-center bg-gray-300 border border-black">"Source"</th>
                                        <th class="p-2 text-center bg-gray-300 border border-black">"Destination"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <For 
                                        each=move || preview_results_rw.get().into_iter()
                                        key=|project_config| project_config.name.clone()
                                        children=move |project_config| {
                                            project_config.diffs.iter().map(|diff| {
                                                view! {
                                                    <tr class="hover:bg-gray-200">
                                                        <td class="p-2 text-left border border-black">{project_config.name.clone()}</td>
                                                        <td class="p-2 text-left border border-black">{diff.key.clone()}</td>
                                                        <td class="p-2 border border-black">{diff.source_value.clone()}</td>
                                                        <td class="p-2 border border-black">{diff.dest_value.clone()}</td>
                                                    </tr>
                                                }    
                                            }).collect::<Vec<_>>().into_view()
                                        }
                                    />
                                </tbody>
                            </table>
                        </div>
                    </div>
                }.into_any()
            }}
        </Show>
    }
}