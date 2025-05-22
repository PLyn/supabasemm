use leptos::prelude::*;
use leptos::task::spawn_local;
use super::functions::{get_project_metrics};
use crate::shared::server_functions::{check_auth_status, get_projects};
use crate::shared::models::{Project, ProjectMetrics};

#[component]
pub fn MetricsPage() -> impl IntoView {
    let is_authenticated_rw = RwSignal::new(false);
    let selected_project_rw = RwSignal::new("".to_string());
    let project_metrics_rw: RwSignal<Vec<ProjectMetrics>> = RwSignal::new(Vec::new()); 
    let projects_rw: RwSignal<Vec<Project>> = RwSignal::new(Vec::new()); 

    Effect::new(move |_| {
        spawn_local(async move {
            let auth_check_result = check_auth_status().await;
            if let Ok(auth_status) = auth_check_result {
                is_authenticated_rw.set(auth_status);
            };

            if is_authenticated_rw.get_untracked() && projects_rw.get_untracked().is_empty(){
                let projects_result = get_projects().await;
                if let Ok(projects_list) = projects_result {
                    projects_rw.set(projects_list);
                };
            }

            if is_authenticated_rw.get_untracked() && !selected_project_rw.get_untracked().is_empty() {
                let metrics_result = get_project_metrics(selected_project_rw.get()).await;
                if let Ok(metrics) = metrics_result {
                    project_metrics_rw.set(metrics);
                };
            }
        });
    });

    view! {
        <h2>"Supabase Live Metrics"</h2>

        <br />
        <label>"Select Source Project"</label>
        <select
            on:change:target=move |ev| {
                selected_project_rw.set(ev.target().value().parse().unwrap());
            }
            prop:value=move || selected_project_rw.get().to_string()>
                <Suspense fallback=move || view! { <option value="">Loading projects...</option> }>
                    {move || {
                        if !projects_rw.get().is_empty() && selected_project_rw.get().is_empty() {
                            selected_project_rw.set(projects_rw.get()[0].id.clone());
                        }

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

        <h3>"Project Metrics"</h3>
        {move || {
            let metrics = project_metrics_rw.get();
                view! {
                    <table>
                        <thead>
                            <tr>
                                <th>"Metric Name"</th>
                                <th>"Value"</th>
                                <th>"Timestamp"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {metrics
                                .into_iter()
                                .map(|metric| {
                                    view! {
                                        <tr>
                                            <td>{metric.metric_name}</td>
                                            <td>{metric.value}</td>
                                            <td>{metric.timestamp}</td>
                                        </tr>
                                    }
                                })
                                .collect_view()}
                        </tbody>
                    </table>
                }.into_any()
            }
        }
    }
}