use crate::shared::models::{Project,DiffEntry};

use super::components::{AuthorizeForm, ConfigSelectForm, ProjectSelectForm};
use super::functions::{check_auth_status, generate_preview, get_projects};
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
    let projects_validated_rw = RwSignal::new(false);
    let current_step_rw = RwSignal::new(ViewSteps::Projects);

    let config_items_rw: [RwSignal<bool>; 6] = std::array::from_fn(|_| RwSignal::new(false));
    let projects_rw: RwSignal<Vec<Project>> = RwSignal::new(Vec::new());
    let diff_results_rw: RwSignal<Vec<DiffEntry>> = RwSignal::new(Vec::new());

    let any_config_selected = Signal::derive(move || {
        config_items_rw.iter().any(|signal| signal.get())
    });

    Effect::new(move |_| {
        //Only added here to make this effect re-run every time current_step_rw changes
        let current_step = current_step_rw.get();

        spawn_local(async move {
            let auth_status = check_auth_status().await;
            let is_authenticated = match auth_status {
                Ok(status) => status,
                Err(e) => {
                    eprintln!("Error checking auth status: {:?}", e);
                    false
                }
            };
            is_authenticated_rw.set(is_authenticated);
        });
    });

    Effect::new(move |_| {
        //Only added here to make this effect re-run every time current_step_rw changes
        let current_step = current_step_rw.get();
        let is_authenticated = is_authenticated_rw.get();
        if current_step == ViewSteps::Projects && projects_rw.get().is_empty() {
            spawn_local(async move {
                if is_authenticated {
                    let projects_result = get_projects().await;
                    match projects_result {
                        Ok(projects_list) => {
                                projects_rw.set(projects_list);    
                        }
                        Err(e) => {
                            eprintln!("Failed to fetch projects: {:?}", e);
                        }
                    }
                }
            });
        }
    });

    view! {
        <h2>Supabase Migrate project configuration</h2>

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
                            projects_validated_rw
                        />

                        <Show when=move || projects_validated_rw.get() >
                            <button on:click=move |_| { current_step_rw.set(ViewSteps::Config); }>Next</button>
                        </Show>          
                    </>
                }.into_any(),
                ViewSteps::Config => view! {
                    <>
                        <ConfigSelectForm
                            config_items_rw />

                        <button on:click=move |_| { current_step_rw.set(ViewSteps::Projects); }>Back</button>

                        <Show when=move || any_config_selected.get() >
                            <button on:click=move |_| {
                                current_step_rw.set(ViewSteps::Loading);
                                spawn_local(async move {
                                    let diff_results_option = generate_preview(source_project_rw.get(), dest_project_rw.get()).await;
                                    match diff_results_option {
                                        Ok(diff_results) => diff_results_rw.set(diff_results),
                                        Err(_) => diff_results_rw.set(Vec::new())
                                    }
                                    current_step_rw.set(ViewSteps::Preview);
                                });
                            }>Preview</button>
                        </Show>
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
                        <h3>Preview Results</h3>
                        <Show
                            when=move || !diff_results_rw.get().is_empty()
                            fallback=move || view! { <p>"No significant differences found or an error occurred."</p> }
                        >
                            <table style="width:100%; border-collapse: collapse; border: 1px solid black;">
                                <thead>
                                    <tr>
                                        <th style="padding: 2px; text-align: center; background-color: #f2f2f2; border: 1px solid black;">Config Item</th>
                                        <th style="padding: 2px; text-align: center; background-color: #f2f2f2; border: 1px solid black;">Source Config</th>
                                        <th style="padding: 2px; text-align: center; background-color: #f2f2f2; border: 1px solid black;">Destination Config</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <For
                                        each=move || diff_results_rw.get().into_iter().enumerate()
                                        key=|(idx, _)| *idx
                                        children=move |(_, diff_text)| {
                                            view! {
                                                <tr>
                                                    <td style="padding: 2px; text-align: left; border: 1px solid black;">{diff_text.config_type.to_uppercase()}:" "{diff_text.key}</td>
                                                    <td style="padding: 2px; border: 1px solid black;">{diff_text.source_value}</td>
                                                    <td style="padding: 2px; border: 1px solid black;">{diff_text.dest_value}</td>
                                                </tr>
                                            }
                                        }
                                    />
                                </tbody>
                            </table>
                        </Show>
                        <button on:click=move |_| { current_step_rw.set(ViewSteps::Config); }>Back to Config</button>
                    </>
                }.into_any()
            }}
        </Show>
    }
}
