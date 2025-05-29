use crate::shared::models::{DiffEntry, Project};

use super::components::{ConfigSelectForm, ProjectSelectForm};
use super::functions::generate_preview;
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

#[derive(Clone, Copy)]
pub struct ConfigState {
    pub items: [(&'static str, &'static str); 6],
    pub values: [RwSignal<bool>; 6],
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

    let preview_results_rw: RwSignal<Vec<DiffEntry>> = RwSignal::new(Vec::new());

    let config_state_rw: RwSignal<ConfigState> = RwSignal::new(ConfigState::new());

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
                                    let diff_results_option = generate_preview(source_project_rw.get_untracked(), dest_project_rw.get_untracked()).await;
                                    match diff_results_option {
                                        Ok(diff_results) => preview_results_rw.set(diff_results),
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
                        <button class="btn btn-secondary mb-4" on:click=move |_| { current_step_rw.set(ViewSteps::Config); }>"Back"</button>
                        <h3 class="text-2xl font-bold mb-4">Preview Results</h3>
                        <Show
                            when=move || !preview_results_rw.get().is_empty()
                            fallback=move || view! { <p>"No significant differences found or an error occurred."</p> }
                        >
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
                                            each=move || preview_results_rw.get().into_iter().enumerate()
                                            key=|(idx, _)| *idx
                                            children=move |(_, diff_text)| {
                                                view! {
                                                    <tr class="hover:bg-gray-200">
                                                        <td class="p-2 text-left border border-black">{diff_text.config_type}</td>
                                                        <td class="p-2 text-left border border-black">{diff_text.key}</td>
                                                        <td class="p-2 border border-black">{diff_text.source_value}</td>
                                                        <td class="p-2 border border-black">{diff_text.dest_value}</td>
                                                    </tr>
                                                }
                                            }
                                        />
                                    </tbody>
                                </table>
                            </div>
                        </Show>
                        <button class="btn btn-primary mt-4" on:click=move |_| {
                            current_step_rw.set(ViewSteps::Loading);
                            spawn_local(async move {

                                current_step_rw.set(ViewSteps::Preview);
                            });
                        }>"Migrate Project Configuration!"</button>
                    </div>
                }.into_any()
            }}
        </Show>
    }
}
